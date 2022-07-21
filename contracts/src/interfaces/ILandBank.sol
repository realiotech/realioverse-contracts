// SPDX-License-Identifier: MIT
pragma solidity >=0.8.15;

interface ILandBank {
    function withdraw(address _beneficiary, uint256 _amount) external;

    function sellLandToBank(address _seller, uint256 _tokenId) external;

    function buyLandFromBank(address _buyer, uint256 tokenId) external;
}
