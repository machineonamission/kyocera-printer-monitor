
//<script>
/**
 * P001 Model
 * @constructor
 * @name Hme_PaperModel
 * @return Hme_PaperModel
 */
var Hme_PaperModel;

(function () {
    'use strict';

    var Constructor = function () {
        },
        _pp = {};

    Constructor = function () {
        'use strict';

        var self = this;

        self.name = 'Hme_PaperModel';

// execute model class constructor
        self = Model.apply(self, arguments);

        _pp = self.pp().properties;

        _pp.getCassetCount = parseInt('4', 10);
        _pp.mpTrayStatus = parseInt('1', 10);
        _pp.getCasset0PaperLayout = parseInt('1', 10);
        _pp.getCasset1PaperLayout = parseInt('1', 10);
        _pp.getCasset2PaperLayout = parseInt('1', 10);
        _pp.getCasset3PaperLayout = parseInt('0', 10);
        _pp.getCasset4PaperLayout = parseInt('0', 10);
        _pp.getCasset5PaperLayout = parseInt('1', 10);
        _pp.getCasset6PaperLayout = parseInt('1', 10);
        _pp.getCasset7PaperLayout = parseInt('1', 10);
        _pp.getCasset8PaperLayout = parseInt('', 10);
        _pp.getOptMPTrayPaperLayout = parseInt('', 10);
        _pp.inserterStatus = '';

        _pp.getCassetLevel = [];
        _pp.getCassetLevel.push('100');

        _pp.getPaperStatus = [];
        _pp.getPaperStatus.push('_mode_on');


        /*For Paper Level and Paper Status*/
        switch (_pp.getCassetCount) {
            case 1:
                if (_pp.mpTrayStatus === 1) {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                break;
            case 2:
                _pp.getCassetLevel.push('100');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getCassetLevel.push('level_empty');
                }
                _pp.getPaperStatus.push('_mode_on');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getPaperStatus.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                break;
            case 3:
                _pp.getCassetLevel.push('100');
                _pp.getCassetLevel.push('70');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getCassetLevel.push('level_empty');
                }
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getPaperStatus.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                break;
            case 4:
                _pp.getCassetLevel.push('100');
                _pp.getCassetLevel.push('70');
                _pp.getCassetLevel.push('70');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getCassetLevel.push('level_empty');
                }
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getPaperStatus.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                break;
            case 5:
                _pp.getCassetLevel.push('100');
                _pp.getCassetLevel.push('70');
                _pp.getCassetLevel.push('70');
                _pp.getCassetLevel.push('level_empty');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getCassetLevel.push('level_empty');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.getCassetLevel.push('');
                }
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_off');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getPaperStatus.push('');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.getPaperStatus.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                break;
            case 6:
                _pp.getCassetLevel.push('100');
                _pp.getCassetLevel.push('70');
                _pp.getCassetLevel.push('70');
                _pp.getCassetLevel.push('level_empty');
                _pp.getCassetLevel.push('level_empty');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getCassetLevel.push('level_empty');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.getCassetLevel.push('');
                }
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_off');
                _pp.getPaperStatus.push('_mode_off');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getPaperStatus.push('');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.getPaperStatus.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                break;
            case 7:
                _pp.getCassetLevel.push('100');
                _pp.getCassetLevel.push('70');
                _pp.getCassetLevel.push('70');
                _pp.getCassetLevel.push('level_empty');
                _pp.getCassetLevel.push('level_empty');
                _pp.getCassetLevel.push('level_empty');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getCassetLevel.push('level_empty');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.getCassetLevel.push('');
                }
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_off');
                _pp.getPaperStatus.push('_mode_off');
                _pp.getPaperStatus.push('_mode_off');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getPaperStatus.push('');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.getPaperStatus.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                break;
            case 8:
                _pp.getCassetLevel.push('100');
                _pp.getCassetLevel.push('70');
                _pp.getCassetLevel.push('70');
                _pp.getCassetLevel.push('level_empty');
                _pp.getCassetLevel.push('level_empty');
                _pp.getCassetLevel.push('level_empty');
                _pp.getCassetLevel.push('');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getCassetLevel.push('level_empty');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.getCassetLevel.push('');
                }
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_off');
                _pp.getPaperStatus.push('_mode_off');
                _pp.getPaperStatus.push('_mode_off');
                _pp.getPaperStatus.push('');
                if (_pp.mpTrayStatus === 1) {
                    _pp.getPaperStatus.push('');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.getPaperStatus.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                    _pp.getCassetLevel.push('level_empty');
                    _pp.getPaperStatus.push('_mode_off');
                }
                break;
            default:
                _pp.getCassetLevel.push('100');
                _pp.getCassetLevel.push('70');
                _pp.getCassetLevel.push('70');
                _pp.getCassetLevel.push('level_empty');
                _pp.getCassetLevel.push('level_empty');
                _pp.getCassetLevel.push('level_empty');
                _pp.getCassetLevel.push('');
                _pp.getCassetLevel.push('level_empty');
                _pp.getCassetLevel.push('');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_on');
                _pp.getPaperStatus.push('_mode_off');
                _pp.getPaperStatus.push('_mode_off');
                _pp.getPaperStatus.push('_mode_off');
                _pp.getPaperStatus.push('');
                _pp.getPaperStatus.push('');
                _pp.getPaperStatus.push('');
                break;
        }


        _pp.PaperSize = [];
        _pp.PaperSize.push('papersize_us_legal');
        switch (_pp.getCassetCount) {
            case 1:
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperSize.push('papersize_us_letter_r');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperSize.push('');
                    _pp.PaperSize.push('');
                }
                break;
            case 2:
                _pp.PaperSize.push('papersize_us_ledger');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperSize.push('papersize_us_letter_r');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperSize.push('');
                    _pp.PaperSize.push('');
                }
                break;
            case 3:
                _pp.PaperSize.push('papersize_us_ledger');
                _pp.PaperSize.push('papersize_us_letter');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperSize.push('papersize_us_letter_r');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperSize.push('');
                    _pp.PaperSize.push('');
                }
                break;
            case 4:
                _pp.PaperSize.push('papersize_us_ledger');
                _pp.PaperSize.push('papersize_us_letter');
                _pp.PaperSize.push('papersize_us_letter');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperSize.push('papersize_us_letter_r');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperSize.push('');
                    _pp.PaperSize.push('');
                }
                break;
            case 5:
                _pp.PaperSize.push('papersize_us_ledger');
                _pp.PaperSize.push('papersize_us_letter');
                _pp.PaperSize.push('papersize_us_letter');
                _pp.PaperSize.push('');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperSize.push('papersize_us_letter_r');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperSize.push('');
                    _pp.PaperSize.push('');
                }
                break;
            case 6:
                _pp.PaperSize.push('papersize_us_ledger');
                _pp.PaperSize.push('papersize_us_letter');
                _pp.PaperSize.push('papersize_us_letter');
                _pp.PaperSize.push('');
                _pp.PaperSize.push('');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperSize.push('papersize_us_letter_r');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperSize.push('');
                    _pp.PaperSize.push('');
                }
                break;
            case 7:
                _pp.PaperSize.push('papersize_us_ledger');
                _pp.PaperSize.push('papersize_us_letter');
                _pp.PaperSize.push('papersize_us_letter');
                _pp.PaperSize.push('');
                _pp.PaperSize.push('');
                _pp.PaperSize.push('');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperSize.push('papersize_us_letter_r');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperSize.push('');
                    _pp.PaperSize.push('');
                }
                break;
            case 8:
                _pp.PaperSize.push('papersize_us_ledger');
                _pp.PaperSize.push('papersize_us_letter');
                _pp.PaperSize.push('papersize_us_letter');
                _pp.PaperSize.push('');
                _pp.PaperSize.push('');
                _pp.PaperSize.push('');
                _pp.PaperSize.push('');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperSize.push('papersize_us_letter_r');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperSize.push('');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperSize.push('');
                    _pp.PaperSize.push('');
                }
                break;
        }

        _pp.PaperType = [];
        _pp.PaperType.push('mediatype_plain');
        switch (_pp.getCassetCount) {
            case 1:
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperType.push('mediatype_plain');
                    _pp.PaperType.push('mediatype_plain');
                }
                break;
            case 2:
                _pp.PaperType.push('mediatype_plain');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperType.push('mediatype_plain');
                    _pp.PaperType.push('mediatype_plain');
                }
                break;
            case 3:
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperType.push('mediatype_plain');
                    _pp.PaperType.push('mediatype_plain');
                }
                break;
            case 4:
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperType.push('mediatype_plain');
                    _pp.PaperType.push('mediatype_plain');
                }
                break;
            case 5:
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperType.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperType.push('mediatype_plain');
                    _pp.PaperType.push('mediatype_plain');
                }
                break;
            case 6:
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperType.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperType.push('mediatype_plain');
                    _pp.PaperType.push('mediatype_plain');
                }
                break;
            case 7:
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperType.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperType.push('mediatype_plain');
                    _pp.PaperType.push('mediatype_plain');
                }
                break;
            case 8:
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('mediatype_plain');
                _pp.PaperType.push('');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperType.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperType.push('mediatype_plain');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperType.push('mediatype_plain');
                    _pp.PaperType.push('mediatype_plain');
                }
                break;
        }

        _pp.PageLayout = [];
        _pp.PageLayout.push('paper_E.png');
        _pp.PageLayout.push('paper_R.png');

        _pp.PaperLayout = [];
        _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
        switch (_pp.getCassetCount) {
            case 1:
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                break;
            case 2:
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                break;
            case 3:
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('0', 10)]);
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                break;
            case 4:
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('0', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('0', 10)]);
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                break;
            case 5:
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('0', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('0', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('', 10)]);
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                break;
            case 6:
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('0', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('0', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('', 10)]);
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                break;
            case 7:
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('0', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('0', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('', 10)]);
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                break;
            case 8:
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('0', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('0', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                _pp.PaperLayout.push(_pp.PageLayout[parseInt('', 10)]);
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('', 10)]);
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                    _pp.PaperLayout.push(_pp.PageLayout[parseInt('1', 10)]);
                }
                break;
        }

        _pp.PaperStatusDisplay = '_mode_on';
        _pp.PaperCapacity = [];
        _pp.PaperCapacity.push('500');
//_pp.PaperStatusDisplay.push('');

        switch (_pp.getCassetCount) {
            case 1:
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperCapacity.push('150');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperCapacity.push('0');
                    _pp.PaperCapacity.push('0');
                }
                break;
            case 2:
                _pp.PaperCapacity.push('500');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperCapacity.push('150');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperCapacity.push('0');
                    _pp.PaperCapacity.push('0');
                }
                break;
            case 3:
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('150');
                if (_pp.inserterStatus === '1') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperCapacity.push('0');
                    _pp.PaperCapacity.push('0');
                }
                break;
            case 4:
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('500');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperCapacity.push('150');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperCapacity.push('0');
                    _pp.PaperCapacity.push('0');
                }
                break;
            case 5:
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('0');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperCapacity.push('150');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperCapacity.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperCapacity.push('0');
                    _pp.PaperCapacity.push('0');
                }
                break;
            case 6:
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('0');
                _pp.PaperCapacity.push('0');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperCapacity.push('150');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperCapacity.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperCapacity.push('0');
                    _pp.PaperCapacity.push('0');
                }
                break;
            case 7:
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('0');
                _pp.PaperCapacity.push('0');
                _pp.PaperCapacity.push('0');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperCapacity.push('150');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperCapacity.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperCapacity.push('0');
                    _pp.PaperCapacity.push('0');
                }
                break;
            case 8:
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('500');
                _pp.PaperCapacity.push('0');
                _pp.PaperCapacity.push('0');
                _pp.PaperCapacity.push('0');
                _pp.PaperCapacity.push('');
                if (_pp.mpTrayStatus === 1) {
                    _pp.PaperCapacity.push('150');
                }
                if (_pp.mpTrayStatus === 2) {
                    _pp.PaperCapacity.push('');
                }
                if (_pp.inserterStatus === '1') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '2') {
                    _pp.PaperCapacity.push('0');
                }
                if (_pp.inserterStatus === '3') {
                    _pp.PaperCapacity.push('0');
                    _pp.PaperCapacity.push('0');
                }
                break;
        }

        _pp.Side_feed_model = '0';

        self.pp().properties = _pp;
    };

    Hme_PaperModel = ModelFactory.create(ModelFactory.type.MODEL, Constructor);

}());
//</script>
