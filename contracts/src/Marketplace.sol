// SPDX-License-Identifier: MIT
pragma solidity >=0.8.15;

// TODO: Fix broken ISwaps
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ILandNFT} from "./interfaces/ILandNft.sol";
import {ILandBank} from "./interfaces/ILandBank.sol";
import {ReentrancyGuard} from "@rari-capital/solmate/src/utils/ReentrancyGuard.sol";

// import {ISwapToken} from "./interfaces/ISwapToken.sol";

contract Marketplace is ReentrancyGuard {
    address private constant RIO_TOKEN =
        0xf21661D0D1d76d3ECb8e1B9F1c923DBfffAe4097;
    address public constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;

    /*
    index   _isAuction  _sellBidPrice   Meaning
    0       true        0               Item 0 is on auction and no bids so far
    1       true        10              Item 1 is on auction and the last bid is for 10 Ethers
    2       false       0               Item 2 is not on auction nor for sell
    3       false       10              Item 3 is on sale for 10 Ethers

    */

    // Auction data

    struct Auction {
        // Parameters of the auction. Times are either
        // absolute unix timestamps (seconds since 1970-01-01)
        // or time periods in seconds.
        address payable beneficiary;
        uint256 auctionEnd;
        // Current state of the auction.
        address payable highestBidder;
        uint256 highestBid;
        // Set to true at the end, disallows any change
        bool open;
        // minimum reserve price in wei
        uint256 reserve;
    }

    mapping(uint256 => Auction) public auctions;
    mapping(uint256 => uint256) public sellBidPrice;
    mapping(uint256 => address payable) public _wallets;
    mapping(uint256 => uint256) public soldFor;

    address public admin;
    address private landBank;
    address private landNft;
    address private swapToken;

    /*╔═════════════════════════════╗
      ║             event           ║
      ╚═════════════════════════════╝*/
    event Sale(
        uint256 indexed tokenId,
        address indexed from,
        address indexed to,
        uint256 value
    );

    event Refund(address bidder, uint256 amount);

    event HighestBidIncreased(
        address indexed bidder,
        uint256 amount,
        uint256 tokenId
    );

    event AuctionEnded(address winner, uint256 amount);

    /**********************************/
    /*╔═════════════════════════════╗
      ║             END             ║
      ║            EVENTS           ║
      ╚═════════════════════════════╝*/
    /**********************************/

    constructor(address _landNft, address _swapToken) {
        require(
            _landNft != address(0) && _swapToken != address(0),
            "can't set zero address"
        );

        landNft = _landNft;
        swapToken = _swapToken;
        admin = msg.sender;
    }

    function setLandBank(address _landBank) external {
        require(_landBank != address(0), "can't set zero address");
        require(msg.sender == admin, "Only admin can initialize");
        landBank = _landBank;
    }

    function sell(
        uint256 tokenId,
        // we have to send the exact price (unitPrice * num)
        uint256 price,
        address payable wallet
    ) external {
        // onlyOwner
        require(
            ILandNFT(landNft).ownerOf(tokenId) == msg.sender,
            "Only owner can sell this item"
        );

        // cannot set a price if auction is activated
        require(
            !auctions[tokenId].open,
            "Cannot sell an item which has an active auction"
        );
        // require(false, "here");

        // set sell price for index
        sellBidPrice[tokenId] = price;

        // If price is zero, means not for sale
        if (price > 0) {
            // set wallet payment
            _wallets[tokenId] = wallet;
        }
    }

    // simple function to return the price of a tokenId
    // returns: sell price, bid price, sold price, only one can be non zero
    function getPrice(uint256 tokenId)
        external
        view
        returns (
            uint256,
            uint256,
            uint256
        )
    {
        if (sellBidPrice[tokenId] > 0) return (sellBidPrice[tokenId], 0, 0);
        if (auctions[tokenId].highestBid > 0)
            return (0, auctions[tokenId].highestBid, 0);
        return (0, 0, soldFor[tokenId]);
    }

    function buy(uint256 tokenId, uint256 _rioAmount)
        external
        payable
        nonReentrant
    {
        // is on sale
        require(
            !auctions[tokenId].open && sellBidPrice[tokenId] > 0,
            "The collectible is not for sale"
        );
        address owner = ILandNFT(landNft).ownerOf(tokenId);
        require(
            msg.sender != owner,
            "The seller cannot buy his own collectible"
        );
        uint256 commissionRate = ILandNFT(landNft).commissionRate();
        if (_rioAmount == 0) {
            // require(
            //     ISwapToken(swapToken).getAmountOutMin(
            //         WETH,
            //         RIO_TOKEN,
            //         msg.value
            //     ) >= sellBidPrice[tokenId],
            //     "Not enough funds"
            // );
            ILandNFT(landNft).transferFrom(owner, msg.sender, tokenId);

            uint256 amount4DevFund = (msg.value * (commissionRate / 2)) / 100;
            uint256 amount4firstOwner = (msg.value * (commissionRate)) / 100;
            uint256 amount4LandBank = (msg.value * (commissionRate / 2)) / 100;
            uint256 amount4owner = msg.value -
                amount4DevFund -
                amount4firstOwner -
                amount4LandBank;

            // to owner
            (bool success1, ) = _wallets[tokenId].call{value: amount4owner}("");
            require(success1, "Transfer to seller failed.");

            // to devFund
            (bool success2, ) = ILandNFT(landNft).devFund().call{
                value: amount4DevFund
            }("");
            require(success2, "Transfer to devrfund failed.");

            // to first owner
            (bool success3, ) = ILandNFT(landNft).firstOwners(tokenId).call{
                value: amount4firstOwner
            }("");
            require(success3, "Transfer to first owner failed.");

            // require(false, "here");
            // ***********************************have to swap and if the valance is correct, send RIO to landbank
            // ISwapToken(swapToken).swap{value: amount4LandBank}(
            //     WETH,
            //     RIO_TOKEN,
            //     amount4LandBank,
            //     0, // we already check if user sent correct value so we don't need this param
            //     address(landBank)
            // );
        } else {
            // RIO case
            require(_rioAmount == sellBidPrice[tokenId], "Not enough funds");
            uint256 amount4DevFund = (_rioAmount * (commissionRate / 2)) / 100;
            uint256 amount4firstOwner = (_rioAmount * (commissionRate)) / 100;
            uint256 amount4LandBank = (_rioAmount * (commissionRate / 2)) / 100;
            uint256 amount4owner = _rioAmount -
                amount4DevFund -
                amount4firstOwner -
                amount4LandBank;

            IERC20(RIO_TOKEN).transferFrom(
                msg.sender,
                address(this),
                amount4DevFund
            );
            IERC20(RIO_TOKEN).approve(swapToken, amount4DevFund);
            // ISwapToken(swapToken).swap(
            //     RIO_TOKEN,
            //     WETH,
            //     amount4DevFund,
            //     0, // we can set the minPrice to 0
            //     ILandNFT(landNft).devFund()
            // );
            // // require(false, "here");
            IERC20(RIO_TOKEN).transferFrom(
                msg.sender,
                address(landBank),
                amount4LandBank
            );
            IERC20(RIO_TOKEN).transferFrom(msg.sender, owner, amount4owner);
            IERC20(RIO_TOKEN).transferFrom(
                msg.sender,
                ILandNFT(landNft).firstOwners(tokenId),
                amount4firstOwner
            );
            ILandNFT(landNft).transferFrom(owner, msg.sender, tokenId);
        }
        soldFor[tokenId] = sellBidPrice[tokenId];
        sellBidPrice[tokenId] = 0;
        _wallets[tokenId] = payable(address(0x0));

        emit Sale(tokenId, owner, msg.sender, msg.value);
    }

    // Instantiate an auction contract for a tokenId
    function createAuction(
        uint256 tokenId,
        uint256 _closingTime,
        address payable _beneficiary,
        uint256 _reservePrice
    ) external {
        require(
            sellBidPrice[tokenId] == 0,
            "The selected NFT is open for sale, cannot be auctioned"
        );
        require(
            !auctions[tokenId].open,
            "The selected NFT already has an auction"
        );
        require(
            ILandNFT(landNft).ownerOf(tokenId) == msg.sender,
            "Only owner can auction this item"
        );

        auctions[tokenId].beneficiary = _beneficiary;
        auctions[tokenId].auctionEnd = block.timestamp + _closingTime;
        auctions[tokenId].reserve = _reservePrice;
        auctions[tokenId].open = true;
    }

    /// Bid on the auction with the value sent
    /// together with this transaction.
    /// The value will only be refunded if the
    /// auction is not won.
    function bid(uint256 tokenId, uint256 rioAmount)
        external
        payable
        nonReentrant
    {
        require(auctions[tokenId].open, "No opened auction found");

        // approve was lost
        require(
            ILandNFT(landNft).getApproved(tokenId) == address(this),
            "Cannot complete the auction"
        );
        // Revert the call if the bidding
        // period is over.
        require(
            block.timestamp <= auctions[tokenId].auctionEnd,
            "Auction already ended."
        );

        address owner = ILandNFT(landNft).ownerOf(tokenId);
        require(
            msg.sender != owner,
            "The owner cannot bid his own collectible"
        );

        if (rioAmount > 0) {
            require(
                rioAmount > auctions[tokenId].highestBid,
                "There already is a higher bid."
            );

            IERC20(RIO_TOKEN).transferFrom(
                msg.sender,
                address(this),
                rioAmount
            );
            if (auctions[tokenId].highestBid > 0) {
                IERC20(RIO_TOKEN).transfer(
                    auctions[tokenId].highestBidder,
                    auctions[tokenId].highestBid
                );
                emit Refund(
                    auctions[tokenId].highestBidder,
                    auctions[tokenId].highestBid
                );
            }

            // now store the bid data
            auctions[tokenId].highestBidder = payable(msg.sender);
            auctions[tokenId].highestBid = rioAmount;
        } else {
            // uint256 rioVal = ISwapToken(swapToken).getAmountOutMin(
            //     WETH,
            //     RIO_TOKEN,
            //     msg.value
            // );
            // require(
            //     rioVal > auctions[tokenId].highestBid,
            //     "There already is a higher bid."
            // );
            // ISwapToken(swapToken).swap{value: msg.value}(
            //     WETH,
            //     RIO_TOKEN,
            //     msg.value,
            //     0, // we can set this to 0 because we already check that in require condition
            //     address(this)
            // );
            if (auctions[tokenId].highestBid > 0) {
                IERC20(RIO_TOKEN).transfer(
                    auctions[tokenId].highestBidder,
                    auctions[tokenId].highestBid
                );
                emit Refund(
                    auctions[tokenId].highestBidder,
                    auctions[tokenId].highestBid
                );
            }
            // now store the bid data
            auctions[tokenId].highestBidder = payable(msg.sender);
            // auctions[tokenId].highestBid = rioVal;
        }
    }

    // anyone can execute withdraw if auction is opened and
    // the bid time expired and the reserve was not met
    // or
    // the auction is openen but the contract is unable to transfer
    function withdrawAuction(uint256 tokenId) external {
        require(
            msg.sender == auctions[tokenId].beneficiary,
            "Only beneficiery can withdraw auction"
        );
        if (
            auctions[tokenId].open &&
            block.timestamp >= auctions[tokenId].auctionEnd &&
            auctions[tokenId].highestBid > 0
        ) {
            IERC20(RIO_TOKEN).transfer(
                auctions[tokenId].highestBidder,
                auctions[tokenId].highestBid
            );
        }

        // finalize the auction
        delete auctions[tokenId];
    }

    function canFinalize(uint256 tokenId) internal view returns (bool) {
        if (
            auctions[tokenId].open &&
            // block.timestamp >= auctions[tokenId].auctionEnd &&
            (auctions[tokenId].highestBid >= auctions[tokenId].reserve ||
                auctions[tokenId].highestBid == 0)
        ) {
            return true;
        } else {
            return false;
        }
    }

    // implement the auctionFinalize including the NFT transfer logic
    // this function will be called when the owner of NFT call this function.
    // owners can call this function when the NFT is sold in reasonable price.
    function auctionFinalize(uint256 tokenId) external nonReentrant {
        require(
            ILandNFT(landNft).ownerOf(tokenId) == msg.sender,
            "Only owner can finalize auction"
        );
        require(canFinalize(tokenId), "Cannot finalize");

        if (auctions[tokenId].highestBid > 0) {
            // transfer the ownership of token to the highest bidder
            address payable finalBidder = auctions[tokenId].highestBidder;
            uint256 commissionRate = ILandNFT(landNft).commissionRate();

            uint256 amount4DevFund = (auctions[tokenId].highestBid *
                (commissionRate / 2)) / 100;
            uint256 amount4firstOwner = (auctions[tokenId].highestBid *
                (commissionRate)) / 100;
            uint256 amount4LandBank = (auctions[tokenId].highestBid *
                (commissionRate / 2)) / 100;
            uint256 amount4owner = auctions[tokenId].highestBid -
                amount4DevFund -
                amount4firstOwner -
                amount4LandBank;

            IERC20(RIO_TOKEN).approve(swapToken, amount4DevFund);
            // ISwapToken(swapToken).swap(
            //     RIO_TOKEN,
            //     WETH,
            //     amount4DevFund,
            //     0,
            //     ILandNFT(landNft).devFund()
            // );
            IERC20(RIO_TOKEN).transfer(address(landBank), amount4LandBank);
            IERC20(RIO_TOKEN).transfer(
                auctions[tokenId].beneficiary,
                amount4owner
            );
            IERC20(RIO_TOKEN).transfer(
                ILandNFT(landNft).firstOwners(tokenId),
                amount4firstOwner
            );

            emit Sale(
                tokenId,
                auctions[tokenId].beneficiary,
                finalBidder,
                auctions[tokenId].highestBid
            );

            // transfer ownership
            address owner = ILandNFT(landNft).ownerOf(tokenId);
            ILandNFT(landNft).transferFrom(owner, finalBidder, tokenId);
        }

        emit AuctionEnded(
            auctions[tokenId].highestBidder,
            auctions[tokenId].highestBid
        );

        // finalize the auction
        delete auctions[tokenId];
    }

    function sellToBank(uint256 tokenId) external nonReentrant {
        require(
            ILandNFT(landNft).ownerOf(tokenId) == msg.sender,
            "Must be owned by msg.sender"
        );
        //transfer NFT from seller to bank contract
        ILandNFT(landNft).transferFrom(msg.sender, address(landBank), tokenId);
        //transfer floor price from bank contract to seller
        ILandBank(landBank).sellLandToBank(msg.sender, tokenId);
    }

    function buyFromBank(uint256 tokenId, uint256 rioAmount)
        external
        payable
        nonReentrant
    {
        require(
            ILandNFT(landNft).ownerOf(tokenId) == address(landBank),
            "Must be owned by bank contract"
        );
        if (rioAmount == 0) {
            // require(
            //     ISwapToken(swapToken).getAmountOutMin(
            //         WETH,
            //         RIO_TOKEN,
            //         (msg.value * 5) / 6
            //     ) >=
            //         (IERC20(RIO_TOKEN).balanceOf(address(landBank)) /
            //             ILandNFT(landNft).totalTileNum()) *
            //             ILandNFT(landNft).getLength(tokenId),
            //     "Must pay exact coin"
            // );

            ILandBank(landBank).buyLandFromBank(msg.sender, tokenId);
            // ISwapToken(swapToken).swap{value: (msg.value * 5) / 6}(
            //     WETH,
            //     RIO_TOKEN,
            //     (msg.value * 5) / 6,
            //     ISwapToken(swapToken).getAmountOutMin(
            //         WETH,
            //         RIO_TOKEN,
            //         (msg.value * 5) / 6
            //     ),
            //     address(landBank)
            // );
            (bool success2, ) = ILandNFT(landNft).devFund().call{
                value: msg.value / 6
            }("");
            require(success2, "Transfer to devFund failed.");
        } else {
            require(
                rioAmount >=
                    (IERC20(RIO_TOKEN).balanceOf(address(landBank)) /
                        ILandNFT(landNft).totalTileNum()) *
                        ILandNFT(landNft).getLength(tokenId),
                "Must pay exact coin"
            );
            IERC20(RIO_TOKEN).transferFrom(
                msg.sender,
                address(this),
                rioAmount
            );
            IERC20(RIO_TOKEN).approve(swapToken, rioAmount / 6);
            // ISwapToken(swapToken).swap(
            //     RIO_TOKEN,
            //     WETH,
            //     rioAmount / 6,
            //     0,
            //     address(ILandNFT(landNft).devFund())
            // );
            IERC20(RIO_TOKEN).transfer(address(landBank), (rioAmount * 5) / 6);
            ILandBank(landBank).buyLandFromBank(msg.sender, tokenId);
        }
    }

    function withdrawFromLandBank(address _beneficiary, uint256 _amount)
        external
    {
        require(admin == msg.sender, "Only admin can withdraw money from bank");
        ILandBank(landBank).withdraw(_beneficiary, _amount);
    }
}
