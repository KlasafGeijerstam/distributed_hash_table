class Convert {
    /**
     * Converts an integer representation of an Ipv4 address
     * to its string counterpart.
     * @param a The address to convert.
     * @return The converted address.
     */
    public static String addressToString(int a) {
        String res = "";
        int[] mask = { 0x000000FF, 0x0000FF00, 0x00FF0000, 0xFF000000 };
        for(int i = 0; i < 4; i++) {
            int b = (a & mask[i]) >> (i * 8);
            res = res + (i != 0 ? "." : "") + b;
        }

        return res;
    }

    /**
     * Converts an integer representation of an Ipv4 address
     * to its string counterpart.
     * @param a The address to convert.
     * @return The converted address.
     */
    public static int stringToIntAddress(String a) {
        var res = 0;
        var addr = a.split("\\.");
        for(int i = 0; i < 4; i++) {
            res |= Integer.parseInt(addr[i]) << (i * 8);
        }

        return res;
    }
}