// SPDX-License-Identifier: MIT
pragma solidity >=0.8.15;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ReentrancyGuard} from "@rari-capital/solmate/src/utils/ReentrancyGuard.sol";
import {ILandNFT} from "./interfaces/ILandNft.sol";
import "./interfaces/ILandBank.sol";

contract LandBank is ReentrancyGuard {
    // RIO token address
    address private constant RIO_TOKEN =
        0xf21661D0D1d76d3ECb8e1B9F1c923DBfffAe4097;

    address public owner;
    address public landNft;

    constructor(address _marketplace, address _landNft) {
        require(
            _marketplace != address(0) && _landNft != address(0),
            "can't set zero address"
        );
        owner = _marketplace;
        landNft = _landNft;
    }

    // receive() external payable {}

    function buyLandFromBank(address _buyer, uint256 tokenId)
        external
        nonReentrant
    {
        require(owner == msg.sender, "Only owner contract can run transaction");
        ILandNFT(landNft).transferFrom(address(this), _buyer, tokenId);
    }

    function sellLandToBank(address _seller, uint256 _tokenId)
        external
        nonReentrant
    {
        require(owner == msg.sender, "Only owner contract can run transaction");
        IERC20(RIO_TOKEN).transfer(
            _seller,
            (IERC20(RIO_TOKEN).balanceOf(address(this)) /
                ILandNFT(landNft).totalTileNum()) *
                ILandNFT(landNft).getLength(_tokenId)
        );
    }

    function withdraw(address _beneficiary, uint256 _amount) external {
        require(
            owner == msg.sender,
            "Only owner contract can call withdraw function"
        );
        require(
            _amount <= IERC20(RIO_TOKEN).balanceOf(address(this)),
            "Too large amount"
        );
        IERC20(RIO_TOKEN).transfer(_beneficiary, _amount);
    }
}
