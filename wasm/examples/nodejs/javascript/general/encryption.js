const calico = require('../../../../nodejs/calico');

calico.initConsolePanicHook();

(async () => {

    let encrypted = calico.encryptXChaCha20Poly1305("my message", "my_password");
    console.log("encrypted:", encrypted);
    let decrypted = calico.decryptXChaCha20Poly1305(encrypted, "my_password");
    console.log("decrypted:", decrypted);

})();
