var ref = require('ref-napi');
var ffi = require('ffi-napi');

var tint = ref.refType('int');
var tlong = ref.refType('long');
var tchar = ref.refType('char *');
var tshort = ref.refType('short');
var tvoid = ref.refType('void');
var buflength = 256;

var eArqConfig = '/home/repolho/workspaces/go-start-lab/acbr_rust/lib/acbrlib.ini';
var eChaveCrypt = '';
var dllACBrLibPosPrinter = '/home/repolho/workspaces/go-start-lab/acbr_rust/lib/acbr/libacbrposprinter64.so';

var ACBrLibPosPrinter = ffi.Library(dllACBrLibPosPrinter, {
    // Métodos da Biblioteca
    POS_Inicializar: ['int', ['string', 'string']],
    POS_Finalizar: ['int', ['void']],
    POS_UltimoRetorno: ['int', [tchar, tint]],
    POS_Nome: ['int', [tchar, tint]],
    POS_Versao: ['int', [tchar, tint]],
    // Métodos de Configuração
    POS_ConfigLer: ['int', ['string']],
    POS_ConfigGravar: ['int', ['string']],
    POS_ConfigLerValor: ['int', ['string', 'string', tchar, tint]],
    POS_ConfigGravarValor: ['int', ['string', 'string', 'string']],
    // Métodos PosPrinter
    POS_Ativar: ['int', ['void']],
    POS_Desativar: ['int', ['void']],
    POS_TxRx: ['int', ['string', 'byte', 'int', 'bool', tchar, tint]],
    POS_Zerar: ['int', ['void']],
    POS_InicializarPos: ['int', ['void']],
    POS_Reset: ['int', ['void']],
    POS_PularLinhas: ['int', ['int']],
    POS_CortarPapel: ['int', ['bool']],
    POS_AbrirGaveta: ['int', ['void']],
    POS_LerInfoImpressora: ['int', [tchar, tint]],
    POS_LerStatusImpressora: ['int', ['int', tlong]],
    POS_RetornarTags: ['int', ['bool', tchar, tint]],
    // Métodos de Impressão
    POS_Imprimir: ['int', ['string', 'bool', 'bool', 'bool', 'int']],
    POS_ImprimirLinha: ['int', ['string']],
    POS_ImprimirCmd: ['int', ['string']],
    POS_ImprimirTags: ['int', ['void']],
});

var cbx = function (callback) {
    if (callback !== undefined && typeof (callback) === "function") {
        return callback;
    }
    return function () {
        console.log("-- callback is empty --");
    };
}

var sTrim = function (valor) {
    return (typeof (valor) === 'string' ? valor.replace(/\0+$/, '').replace(/^\s+|\s+$/gm, '') : valor);
}

var getResultMessage = function (result) {
    console.log('getResultMessage(' + result + ')');
    switch (result) {
        case -1:
            return 'Houve falhas na inicialização da biblioteca.';
        case -2:
            return 'Houve falhas na finalização da biblioteca.';
        case -3:
            return 'Houve erro ao ler a configuração informada.';
        case -5:
            return 'Não foi possível localizar o arquivo de configuração informado.';
        case -6:
            return 'Não foi possível encontrar o diretório do arquivo de configuração.';
        case -10:
            return 'Houve falhas na execução do método.';
        default:
            return 'Desconhecido (#' + result + ')';
    }
}

var CheckResult = function (Result, aloc_sMensagem, aloc_esTamanho, cb, iniLength) {
    try {

        if (Result !== 0)
            return cbx(cb)({ success: false, message: getResultMessage(Result), data: false });

        var sMensagem = sTrim(aloc_sMensagem.toString());
        var esTamanho = aloc_esTamanho.deref();

        console.log('CheckResult <<< sMensagem = ' + JSON.stringify(sMensagem));
        console.log('CheckResult <<< esTamanho = ' + JSON.stringify(esTamanho));

        var loopping = true;
        if (esTamanho < buflength) {
            loopping = false;
        } else {
            if (sMensagem.length < esTamanho && sMensagem.length > buflength) {
                loopping = false;
            } else {
                if (sMensagem.length >= esTamanho) {
                    loopping = false;
                }
            }
        }

        if (loopping === true) {
            aloc_sMensagem = Buffer.alloc(esTamanho);
            return POS_UltimoRetorno(aloc_sMensagem, aloc_esTamanho, cb);
        }

        cbx(cb)({ success: true, message: sMensagem, data: false });
    } catch (e) {
        console.log('CheckResult <<< ' + e.message);
        cbx(cb)({ success: false, message: e.message, data: false });
    }
}

var POS_Inicializar = function (cb) {
    try {
        console.log('POS_Inicializar >>> ' + JSON.stringify([eArqConfig, eChaveCrypt]));
        ACBrLibPosPrinter.POS_Inicializar.async(eArqConfig, eChaveCrypt, function (err, Result) {
            if (err)
                throw err;

            if (Result !== 0)
                return cbx(cb)({ success: false, message: getResultMessage(Result), data: false });

            cbx(cb)({ success: true, message: 'Ok', data: false });
        });
    } catch (e) {
        console.log('POS_Inicializar <<< ' + e.message);
        cbx(cb)({ success: false, message: e.message, data: false });
    }
}

var POS_Finalizar = function (cb) {
    try {
        console.log('POS_Finalizar >>> ' + JSON.stringify([]));
        ACBrLibPosPrinter.POS_Finalizar.async(tvoid, function (err, Result) {
            if (err)
                throw err;

            if (Result !== 0)
                return cbx(cb)({ success: false, message: getResultMessage(Result), data: false });

            cbx(cb)({ success: true, message: 'Ok', data: false });
        });
    } catch (e) {
        console.log('POS_Finalizar <<< ' + e.message);
        cbx(cb)({ success: false, message: e.message, data: false });
    }
}

var POS_UltimoRetorno = function (aloc_sMensagem, aloc_esTamanho, cb) {
    try {

        console.log('POS_UltimoRetorno >>> ' + JSON.stringify(['aloc_sMensagem', 'aloc_esTamanho']));
        ACBrLibPosPrinter.POS_UltimoRetorno.async(aloc_sMensagem, aloc_esTamanho, function (err, Result) {
            if (err)
                throw err;

            CheckResult(Result, aloc_sMensagem, aloc_esTamanho, cb);
        });

    } catch (e) {
        console.log('POS_UltimoRetorno <<< ' + e.message);
        cbx(cb)({ success: false, message: e.message, data: false });
    }
}

var POS_Versao = function (cb) {
    try {
        var aloc_sVersao = Buffer.alloc(buflength);
        var aloc_esTamanho = ref.alloc('int', buflength);

        console.log('POS_Versao >>> ' + JSON.stringify(['aloc_sVersao', 'aloc_esTamanho']));
        ACBrLibPosPrinter.POS_Versao.async(aloc_sVersao, aloc_esTamanho, function (err, Result) {
            if (err)
                throw err;

            CheckResult(Result, aloc_sVersao, aloc_esTamanho, cb);
        });
    } catch (e) {
        console.log('POS_Versao <<< ' + e.message);
        cbx(cb)({success: false, message: e.message, data: false});
    }
}

POS_Inicializar(function (resultado) {
    console.log('POS_Inicializar', resultado.message);
    if (resultado.success === true) {

        POS_Versao(function (resultado) {
            console.log('POS_Versao', resultado.message);
            POS_Finalizar();
        });

        return;
    }
    POS_Finalizar();
});
