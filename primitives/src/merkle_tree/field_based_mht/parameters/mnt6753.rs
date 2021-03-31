use algebra::{
    field_new, biginteger::BigInteger768,
    fields::mnt6753::Fr as MNT6753Fr
};

use crate::{
    crh::poseidon::parameters::mnt6753::MNT6PoseidonHash,
    FieldBasedMerkleTreePrecomputedEmptyConstants,
};

// PoseidonHash("This represents an empty Merkle Root for a MNT6753PoseidonHash based Merkle Tree.") padded with 0s
pub const MNT6753_PHANTOM_MERKLE_ROOT: MNT6753Fr =
    field_new!(MNT6753Fr, BigInteger768([
        17804545126199716292,
        13504778789346325939,
        8402790274902466878,
        12187888741009616640,
        1445484556244903294,
        10889911492809693201,
        14657366305063113250,
        15274081092393286953,
        15478659740064959215,
        10149939511891914858,
        16068212281341350278,
        467184971854198
    ])
);

pub const MNT6753_MHT_POSEIDON_PARAMETERS: FieldBasedMerkleTreePrecomputedEmptyConstants<'static, MNT6PoseidonHash> =
    FieldBasedMerkleTreePrecomputedEmptyConstants {
        nodes: &[
            field_new!(MNT6753Fr, BigInteger768([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])),
            field_new!(MNT6753Fr, BigInteger768([6532273791827364841, 8129439058117860676, 11951671852543742466, 4932126643213168419, 3432346478526347932, 17290815965948604094, 16841611152871512489, 16265777562074717134, 10956818927095834646, 11239269346882274360, 10703980782105434237, 242741090634251])),
            field_new!(MNT6753Fr, BigInteger768([6397115990560507784, 9148723575072162592, 9560680090269062678, 3358387075603874199, 2943549611989557818, 8422251740613038257, 3721287848425091675, 8855060931545600396, 16146169992572165792, 3515758596852444121, 3476512212786839045, 496282776658056])),
            field_new!(MNT6753Fr, BigInteger768([8193435237322254950, 397416662091648072, 16350036082179069772, 9590057510410951207, 16709424488020097545, 9972310166730351408, 3768241609978433635, 8613155250097940260, 9383494548692779671, 7279136739588530718, 16346404903648601854, 12606752292290])),
            field_new!(MNT6753Fr, BigInteger768([12373621395328213151, 7485305202183666277, 15851413934113610491, 5870086679580111567, 17485320997014930518, 12388669921953071164, 7554455377880544421, 7504804608945337937, 2484823991694123126, 15749759632839148553, 555592382807618863, 371867713016710])),
            field_new!(MNT6753Fr, BigInteger768([18093975764471962643, 3666472202090579669, 16218428501104502931, 3494251062696523035, 16618138744281542044, 2303744553101494694, 13256865789347308871, 11526225850590367247, 7735627041796110725, 12904596097188100135, 15532045893935425642, 170176986382358])),
            field_new!(MNT6753Fr, BigInteger768([3756105871785036419, 4065662201479226523, 9540057528274586837, 5039798590697787339, 1050116534895787179, 15480940572426055338, 16199923308809632907, 15474289492203021192, 15991912122991411762, 8917476369884613289, 12500272767462707678, 329947668046476])),
            field_new!(MNT6753Fr, BigInteger768([16010371204820592836, 7011599899378550451, 2684322824964611783, 17742193331414468358, 14706367313712177515, 15495207825020000539, 6073726048041844123, 8614353322952801909, 11299010454287971352, 773910159520485849, 1850753278166510389, 75924681739002])),
            field_new!(MNT6753Fr, BigInteger768([794403130545118313, 5101043865475270524, 15702429857764299907, 9919653825609994610, 9521745413436026279, 10462225413930324850, 7869742079022813329, 10324970627740942303, 17971590582333244023, 13404329769334042052, 580127322712779849, 439280434884595])),
            field_new!(MNT6753Fr, BigInteger768([11482552263285250890, 10604737448313464782, 17862094694325007497, 15660968179388628036, 12176161070410249282, 6980015018713988634, 11261448373704770002, 10373172527710318907, 679135417846676667, 10099544898146858302, 17094142929775691921, 207090155613114])),
            field_new!(MNT6753Fr, BigInteger768([10212005189921138049, 1731388169699505562, 1289559507310603998, 16988439089211854489, 3494220991927223608, 12136870715379239434, 3309411932679713584, 15519376684764760516, 12398049358025134723, 11565085907681961093, 17602152805125972860, 94938708740083])),
            field_new!(MNT6753Fr, BigInteger768([3074487674322137518, 7793531452943931084, 12013378720014384008, 8554113024333850352, 5061975058787031682, 2479800213543457595, 8908267359163192954, 7971911860600108726, 17447381762657585797, 13382295392296925431, 6167239007805973407, 136675550452074])),
            field_new!(MNT6753Fr, BigInteger768([17491742854997096545, 15146273778995160999, 14129140176484323206, 3632858126816120899, 5798922555083157345, 8284717930233163790, 17680470605283712180, 7754503984796689807, 6349022074948921736, 5657226674148234435, 2813293559832455344, 411781272050163])),
            field_new!(MNT6753Fr, BigInteger768([11774712770431367476, 8462830471647010217, 13678995176551118152, 6043163109064753248, 8166717753325571695, 767901680110108379, 3605848228716349807, 14300779503184147030, 6370775021317117965, 7879026566538256232, 16369439687448943956, 387608534863697])),
            field_new!(MNT6753Fr, BigInteger768([7645027089449308941, 12412349174432078586, 7993283570226734824, 14957429658733237619, 4956410016527733167, 18263624961863144740, 12351841146051952024, 16600297263984232345, 2726424323397795882, 3499875062110234443, 14291130706314125429, 101867929645697])),
            field_new!(MNT6753Fr, BigInteger768([16065325380198326553, 2186426031755906773, 22545576011902322, 13499949550339568914, 6330038014686496734, 2672690388187593533, 12018191996919540847, 16663030407152560576, 14862643090082908897, 54525310766581332, 13485511352213383315, 193303572624703])),
            field_new!(MNT6753Fr, BigInteger768([9059598820510117498, 9188436540497286313, 16188227685152310071, 2134701940771565699, 4161629512550412251, 14196286500897307380, 10780852978701816310, 6115435820788597012, 2852364532186258341, 10258019327236403086, 13036999023129561906, 375088429879001])),
            field_new!(MNT6753Fr, BigInteger768([9598862696460783085, 1080984834427407511, 16485613359067559463, 16957526212315867534, 15502481473140036583, 4519893803854135974, 5107877999302114724, 2545265241550671869, 9081416678945077831, 484771691515777633, 5154416543963837978, 45608648987133])),
            field_new!(MNT6753Fr, BigInteger768([13716427665047140884, 2075502376034677990, 13256455732100548018, 7974093355972877989, 4347808650120890559, 17259847505786586988, 6240725626197616495, 1855530573523266842, 4005406563460652820, 409515538624710010, 13650992360907576897, 228052535137342])),
            field_new!(MNT6753Fr, BigInteger768([11101481243483205085, 17210210903078183870, 4322012201136479125, 10446985227641960927, 5539441656148903472, 12121210617279908097, 10211498308203777998, 2264490370510798309, 10081794249268382623, 15791888737308518638, 16942027707510004820, 47761764925972])),
            field_new!(MNT6753Fr, BigInteger768([1147281875969033104, 12950023122422370632, 14308071174385673986, 12202823434772371218, 4755713887108038470, 17978833191653589187, 17771656437507378119, 5760992868553801011, 15499598564511468740, 6347244696152464986, 6444689746322097849, 491305921767844])),
            field_new!(MNT6753Fr, BigInteger768([5464933467908036346, 5357747808011423258, 15896146360099043625, 3771197909278431925, 9470198461608789106, 4731405946371361499, 3983834152193584274, 15409387538868150979, 5707361705089200795, 9338823108567891504, 16606105623990147949, 493525379955592])),
            field_new!(MNT6753Fr, BigInteger768([8743779181811038624, 1271135826285693645, 10331181025026049271, 15045420632252353089, 10426673481537010247, 5496742122291259672, 16828185989998029873, 18159882661333561265, 14090170749933154554, 2146775194765030955, 14040437295351564942, 92059859414488])),
            field_new!(MNT6753Fr, BigInteger768([16102903578885489885, 12086305817737143574, 3125247939582240804, 8356512119453069100, 8835869137933890629, 8718546255489790634, 310309093738603225, 5199065039609564486, 1425248603053710432, 5608159777771994055, 8420963830191136937, 396898575763312])),
            field_new!(MNT6753Fr, BigInteger768([2973076096573253709, 11941130816567704697, 16818712947046097441, 14665243141726638938, 13796641914494847964, 3006062735100874544, 14989082569564552350, 12583524336859840711, 12029573754303017166, 8956481097500682634, 11869416006027825656, 357489242544355])),
            field_new!(MNT6753Fr, BigInteger768([14967216030866112393, 676193698416614878, 2328591331415235269, 12560251955335368780, 4398773412362438456, 1250634036568321184, 14490795618954382205, 13729892135340143916, 6067561844785088088, 2911181006550225208, 4748375130199100773, 350798232822697])),
            field_new!(MNT6753Fr, BigInteger768([292356196722191993, 15985011389523096240, 9851262067956829627, 17650104737270571488, 9788206078127145516, 13228511398536198831, 2659479149006129969, 2012942778305651429, 12610992523318309950, 17117515683699945581, 10851147538124694478, 294926256393078])),
            field_new!(MNT6753Fr, BigInteger768([5176092754286105563, 17084424016779855145, 3966964054171579805, 11062889913630949187, 176794351710939111, 9519095240363490382, 15448474919972035445, 13739324839369951461, 18047710611113136797, 7081897887306182115, 12117210779123289204, 289137842609290])),
            field_new!(MNT6753Fr, BigInteger768([15568880424393099769, 15005743820481502521, 507426405232123037, 1720798598390792226, 10392432490016644011, 278975084664808106, 14362150584459907788, 1981223324889562482, 12026547713605877567, 3683978912835280072, 7024374486915732211, 234502500818062])),
            field_new!(MNT6753Fr, BigInteger768([9685190619050728173, 18398841313063857095, 9229128000573795408, 2919741647173757252, 2747161783389641114, 15584504685849431512, 2306301231800676069, 12724073030490161519, 14676901776633680287, 12352120452083264307, 8823696394899338182, 312377075845553])),
            field_new!(MNT6753Fr, BigInteger768([4013973122795951545, 3968395067503506117, 13439251809632626494, 18235293789157210684, 5619948957156507877, 16663850504748526369, 14217411656817237763, 15746115385138359655, 991511442697256284, 6535303136073949684, 8230314852819489074, 25644836342307])),
            field_new!(MNT6753Fr, BigInteger768([5887384218151647282, 1734549474524408583, 8624295651914788199, 3361061284989788689, 8259278916655142581, 9496331895863691207, 6117844371204405787, 6718695458918184160, 2704639389498915098, 7105632944500388259, 12993183043340066352, 149539870692481])),
        ],
        merkle_arity: 2,
        max_height: 32,
    };

#[cfg(test)]
mod test {

    use algebra::{
        fields::mnt6753::Fr,
        Field,
    };
    use crate::{crh::MNT6PoseidonHash, merkle_tree::field_based_mht::parameters::{
        generate_phantom_merkle_root_from_magic_string,
        generate_mht_empty_nodes,
    }, FieldBasedMerkleTreePrecomputedEmptyConstants};
    use super::{
        MNT6753_PHANTOM_MERKLE_ROOT, MNT6753_MHT_POSEIDON_PARAMETERS
    };

    #[ignore]
    #[test]
    fn test_generate_mnt6753_phantom_merkle_root(){
        let expected_root = generate_phantom_merkle_root_from_magic_string::<Fr, MNT6PoseidonHash>(
            "This represents an empty Merkle Root for a MNT6753PoseidonHash based Merkle Tree."
        );
        assert_eq!(expected_root, MNT6753_PHANTOM_MERKLE_ROOT);
    }


    #[ignore]
    #[test]
    fn test_generate_binary_mnt6753_mht_empty_nodes() {
        let merkle_arity = 2;
        let max_height = 32;

        let empty_nodes = generate_mht_empty_nodes::<Fr, MNT6PoseidonHash>(merkle_arity, max_height, Fr::zero());
        assert_eq!(empty_nodes.len(), max_height);

        let params = FieldBasedMerkleTreePrecomputedEmptyConstants::<MNT6PoseidonHash> {
            nodes: empty_nodes.as_slice(), merkle_arity, max_height
        };

        assert_eq!(params, MNT6753_MHT_POSEIDON_PARAMETERS)
    }
}