// SPDX-License-Identifier: MIT
pragma solidity >=0.8.15;

// we can use openzeppelin interface
import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";

interface ILandNFT is IERC721 {
    function admin() external returns (address);

    function WETH() external returns (address);

    function RIO_TOKEN() external returns (address);

    function commissionRate() external returns (uint256);

    function totalTileNum() external returns (uint64);

    function devFund() external returns (address);

    function firstOwners(uint256 tokenId) external returns (address);

    function getLength(uint256 index) external view returns (uint256 len);
}
