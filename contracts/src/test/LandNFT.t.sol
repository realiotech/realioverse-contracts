// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import "forge-std/Test.sol";
import "../LandNFT.sol";
import {Utilities} from "./utils/Utilities.t.sol";

contract LandNFTTest is Test {
    address public admin;
    address public devFund;
    address public landBank;
    address public brokeDude;
    address public ethDude;
    address public swapToken;
    address public owner = 0xb4c79daB8f259C7Aee6E5b2Aa729821864227e84;
    address public nftReceiver = 0x892D509964C144f501Aaa8fb1a57069789D65Ce4;
    address public rioWhale = 0x94c3857520E9151b34814FbF8B477368F4a97ea7;
    uint64 public totalTileNum;
    string public baseURI;
    uint256 public nextId;
    uint256 public commissionRate;
    uint256 public constant maxTileNum = 10**10;

    address private constant RIO_TOKEN =
        0xf21661D0D1d76d3ECb8e1B9F1c923DBfffAe4097;
    address private constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
    uint256 public constant FEE_MULTIPLIER = (97 / uint256(100));
    uint256 public price = 0.01 ether;

    LandNFT landNFT;
    Utilities internal utils;

    function setUp() public {
        utils = new Utilities();
        address payable[] memory users = utils.createUsers(4);
        devFund = users[0];
        landBank = users[1];
        brokeDude = users[2];
        ethDude = users[3];
        vm.label(devFund, "devFund");
        vm.label(landBank, "landBank");
        vm.label(brokeDude, "brokeDude");
        vm.label(ethDude, "ethDude");
        landNFT = new LandNFT(devFund, landBank, price);
    }

    function testInitialization() public {
        assertEq(landNFT.devFund(), devFund);
        assertEq(landNFT.landBank(), landBank);
        assertEq(landNFT.admin(), owner);
        assertEq(landNFT.commissionRate(), 10);
        assertEq(landNFT.baseURI(), "Realio");
        assertEq(landNFT.price(), price);
    }

    function testSetAdmin() public {
        address newAdmin = 0x118849c94F887210D933f59Cd962002dF28cB896;
        vm.prank(owner);
        landNFT.setAdmin(newAdmin);
        assertEq(landNFT.admin(), newAdmin);
        vm.stopPrank();
        vm.prank(newAdmin);
        vm.expectRevert(CannotSetAddressZero.selector);
        landNFT.setAdmin(address(0));
        vm.stopPrank();
        vm.startPrank(address(0xE1277a3465B92E329c49991D4b95Bc779ba43765));
        vm.expectRevert(NotAuthorised.selector);
        landNFT.setAdmin(newAdmin);
    }

    function testSetPrice() public {
        uint256 newPrice = 100;
        vm.prank(owner);
        landNFT.setPrice(newPrice);
        assertEq(landNFT.price(), newPrice);
    }

    function testSetCommissionRate() public {
        uint256 newCommissionRate = 20;
        vm.prank(owner);
        landNFT.setCommissionRate(newCommissionRate);
        assertEq(landNFT.commissionRate(), newCommissionRate);
    }

    function testSetLandBank() public {
        address newLandBank = 0x118849c94F887210D933f59Cd962002dF28cB896;
        vm.prank(owner);
        landNFT.setLandBank(payable(newLandBank));
        assertEq(landNFT.landBank(), newLandBank);
    }

    function testPause() public {
        vm.prank(owner);
        landNFT.pause();
        assertEq(landNFT.paused(), true);
    }

    function testUnpause() public {
        vm.prank(owner);
        landNFT.pause();
        assertEq(landNFT.paused(), true);
        vm.prank(owner);
        landNFT.unpause();
        assertEq(landNFT.paused(), false);
    }

    function testSafeMintWithRio() public {
        vm.startPrank(rioWhale);
        uint256[] memory regions = new uint256[](3);
        regions[0] = 1;
        regions[1] = 2;
        regions[2] = 3;
        address[] memory path = new address[](2);
        path[0] = address(WETH);
        path[1] = address(RIO_TOKEN);
        uint256 rioAmount = landNFT.getTokenPrice(price * regions.length);
        IERC20(RIO_TOKEN).approve(address(landNFT), rioAmount);
        landNFT.mint(regions, rioAmount);
        vm.expectRevert(RegionAlreadyOwned.selector);
        landNFT.mint(regions, rioAmount);
        vm.stopPrank();
        // Assert that the tile is minted to the rioWhale
        assertEq(regions.length, landNFT.balanceOf(rioWhale));
        assertEq(regions.length, landNFT.tilesBought());
        path = new address[](2);
        path[0] = address(RIO_TOKEN);
        path[1] = address(WETH);
        // Assert that Developer Fund is increased by 20 % of the sale price
        // We use the greater than or equals to comparison as we can't determine
        // the exact amount of tokens that are sent to the devfund, but we know that
        // it should be at least the `minOutAmount` for the swap.
        uint256 minOutAmountDevFund = landNFT.getAmountOutMin(
            ((rioAmount * 20) / 100),
            path
        );
        assertGe(address(devFund).balance, minOutAmountDevFund);
        assertEq((rioAmount * 8) / 10, IERC20(RIO_TOKEN).balanceOf(landBank));
        vm.startPrank(rioWhale);
        IERC20(RIO_TOKEN).transfer(brokeDude, 1 * 10**20);
        vm.stopPrank();
        vm.startPrank(brokeDude);
        regions = new uint256[](3);
        regions[0] = 4;
        regions[1] = 5;
        regions[2] = 6;
        uint256 newRioAmount = 1 * 10**18;
        IERC20(RIO_TOKEN).approve(address(landNFT), newRioAmount);
        vm.expectRevert(InsufficientBalance.selector);
        landNFT.mint(regions, rioAmount);
    }

    function testSafeMintWithETH() public {
        vm.prank(ethDude);
        uint256[] memory regions = new uint256[](3);
        regions[0] = 4;
        regions[1] = 5;
        regions[2] = 6;
        uint256 ethAmount = price * regions.length;
        landNFT.mint{value: ethAmount}(regions, 0);
        assertEq(regions.length, landNFT.balanceOf(ethDude));
        assertEq(regions.length, landNFT.tilesBought());
        vm.expectRevert(RegionAlreadyOwned.selector);
        landNFT.mint{value: ethAmount}(regions, 0);
        /// @dev: the dev account is seeded with 100 Ether so we need to reflex that
        /// in the assertion.
        assertEq(address(devFund).balance, 100 ether + (ethAmount * 2) / 10);
        address[] memory path = new address[](2);
        path[0] = address(WETH);
        path[1] = address(RIO_TOKEN);
        // Assert that LandBank's Balance is increased by 80 % of the sale price
        // We use the greater than or equals to comparison as we can't determine
        // the exact amount of tokens that are sent to the devfund, but we know that
        // it should be at least the `minOutAmount` for the swap.
        uint256 minOutAmountLandBankFund = landNFT.getAmountOutMin(
            ((ethAmount * 80) / 100),
            path
        );
        assertGe(
            IERC20(RIO_TOKEN).balanceOf(landBank),
            minOutAmountLandBankFund
        );
        uint256 cheapSkate = (price * regions.length) / 10;
        regions = new uint256[](3);
        regions[0] = 8;
        regions[1] = 9;
        regions[2] = 10;
        vm.expectRevert(InsufficientBalance.selector);
        landNFT.mint{value: cheapSkate}(regions, 0);
    }

    // function testTokenURI() public {
    //     string memory uri = landNFT.tokenURI(1);
    //     assertEq(uri, "");
    // }

    receive() external payable {}
}
