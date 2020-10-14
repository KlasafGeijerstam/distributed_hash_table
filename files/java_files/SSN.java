class SSN {
    private String ssn;

    public SSN(String ssn) {
        this.ssn = ssn;
    }

    public String getSSN() {
        return ssn;
    }

    @Override
    public int hashCode() {
        return Hasher.hashSSN(ssn);
    }
    
    @Override
    public boolean equals(Object o) {
        return this.ssn.equals(o);
    }
}
