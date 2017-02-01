# ASNom.1

ASN.1 BER implementation using (big surprise) nom.

Currently used to implemenent [LDAP](https://github.com/Dean4Devil/rust-ldap), so I will for now only implement what is needed for LDAP:

- [ ] Implicit Tags
- [x] Sequence
- [x] Choice (No special code for now)
- [x] Integer
- [x] OctetString
- [ ] With Components (Probably won't do any special code for that)
- [x] Enumerated (Just an integer so probably no special code for now)
- [ ] Sequence Of
- [x] Boolean
- [x] Null
- [ ] Set Of

I may implement a full ASN.1 suite later on but that is out of scope in the medium term.

# License

[MIT](LICENSE)
