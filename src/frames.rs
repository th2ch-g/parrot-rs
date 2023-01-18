

//pub struct Frames<'a> {
pub struct Frames {
    // pub FRAME_LIST: [&'a str; 10],
    // pub FRAME_NONE: &'a str,
}

//impl Frames<'_> {
impl Frames {

    // frame for clean
    pub const FRAME_NONE: &str = "
\r
\r
\r
\r
\r
\r
\r
\r
\r
";
    // parrot frame list
    // These frame is made from parrot.live. (see gitsubmodule path for more information)
    // CMD: ls ../parrot.live/frames/*.txt | while IFS= read -r line; do echo '"'; cat $line; echo '",'; done >> frames.rs
    // VIM: :s/^/\\r/g
    pub const FRAME_LIST: [&str; 10] = [

"
\r                         .cccc;;cc;';c.
\r                      .,:dkdc:;;:c:,:d:.
\r                     .loc'.,cc::c:::,..;:.
\r                   .cl;....;dkdccc::,...c;
\r                  .c:,';:'..ckc',;::;....;c.
\r                .c:'.,dkkoc:ok:;llllc,,c,';:.
\r               .;c,';okkkkkkkk:;lllll,:kd;.;:,.
\r               co..:kkkkkkkkkk:;llllc':kkc..oNc
\r             .cl;.,oxkkkkkkkkkc,:cll;,okkc'.cO;
\r             ;k:..ckkkkkkkkkkkl..,;,.;xkko:',l'
\r            .,...';dkkkkkkkkkkd;.....ckkkl'.cO;
\r         .,,:,.;oo:ckkkkkkkkkkkdoc;;cdkkkc..cd,
\r      .cclo;,ccdkkl;llccdkkkkkkkkkkkkkkkd,.c;
\r     .lol:;;okkkkkxooc::coodkkkkkkkkkkkko'.oc
\r   .c:'..lkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkd,.oc
\r  .lo;,:cdkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkd,.c;
\r,dx:..;lllllllllllllllllllllllllllllllllc'...
\rcNO;........................................
\r",
"
\r                .ckx;'........':c.
\r             .,:c:::::oxxocoo::::,',.
\r            .odc'..:lkkoolllllo;..;d,
\r            ;c..:o:..;:..',;'.......;.
\r           ,c..:0Xx::o:.,cllc:,'::,.,c.
\r           ;c;lkXKXXXXl.;lllll;lKXOo;':c.
\r         ,dc.oXXXXXXXXl.,lllll;lXXXXx,c0:
\r         ;Oc.oXXXXXXXXo.':ll:;'oXXXXO;,l'
\r         'l;;kXXXXXXXXd'.'::'..dXXXXO;,l'
\r         'l;:0XXXXXXXX0x:...,:o0XXXXx,:x,
\r         'l;;kXXXXXXXXXKkol;oXXXXXXXO;oNc
\r        ,c'..ckk0XXXXXXXXXX00XXXXXXX0:;o:.
\r      .':;..:do::ooookXXXXXXXXXXXXXXXo..c;
\r    .',',:co0XX0kkkxxOXXXXXXXXXXXXXXXOc..;l.
\r  .:;'..oXXXXXXXXXXXXXXXXXXXXXXXXXXXXXko;';:.
\r.ldc..:oOXKXXXXXXKXXKXXXXXXXXXXXXXXXXXXXo..oc
\r:0o...:dxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxo,.:,
\rcNo........................................;'
\r",
"
\r            .cc;.  ...  .;c.
\r         .,,cc:cc:lxxxl:ccc:;,.
\r        .lo;...lKKklllookl..cO;
\r      .cl;.,:'.okl;..''.;,..';:.
\r     .:o;;dkd,.ll..,cc::,..,'.;:,.
\r     co..lKKKkokl.':lloo;''ol..;dl.
\r   .,c;.,xKKKKKKo.':llll;.'oOxl,.cl,.
\r   cNo..lKKKKKKKo'';llll;;okKKKl..oNc
\r   cNo..lKKKKKKKko;':c:,'lKKKKKo'.oNc
\r   cNo..lKKKKKKKKKl.....'dKKKKKxc,l0:
\r   .c:'.lKKKKKKKKKk;....lKKKKKKo'.oNc
\r     ,:.'oxOKKKKKKKOxxxxOKKKKKKxc,;ol:.
\r     ;c..'':oookKKKKKKKKKKKKKKKKKk:.'clc.
\r   ,xl'.,oxo;'';oxOKKKKKKKKKKKKKKKOxxl:::;,.
\r  .dOc..lKKKkoooookKKKKKKKKKKKKKKKKKKKxl,;ol.
\r  cx,';okKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKl..;lc.
\r  co..:dddddddddddddddddddddddddddddddddl::',::.
\r  co...........................................
\r",
"
\r           .ccccccc.
\r      .,,,;cooolccoo;;,,.
\r     .dOx;..;lllll;..;xOd.
\r   .cdo;',loOXXXXXkll;';odc.
\r  ,ol:;c,':oko:cccccc,...ckl.
\r  ;c.;kXo..::..;c::'.......oc
\r,dc..oXX0kk0o.':lll;..cxxc.,ld,
\rkNo.'oXXXXXXo',:lll;..oXXOo;cOd.
\rKOc;oOXXXXXXo.':lol;..dXXXXl';xc
\rOl,:k0XXXXXX0c.,clc'.:0XXXXx,.oc
\rKOc;dOXXXXXXXl..';'..lXXXXXo..oc
\rdNo..oXXXXXXXOx:..'lxOXXXXXk,.:; ..
\rcNo..lXXXXXXXXXOolkXXXXXXXXXkl,..;:';.
\r.,;'.,dkkkkk0XXXXXXXXXXXXXXXXXOxxl;,;,;l:.
\r  ;c.;:''''':doOXXXXXXXXXXXXXXXXXXOdo;';clc.
\r  ;c.lOdood:'''oXXXXXXXXXXXXXXXXXXXXXk,..;ol.
\r  ';.:xxxxxocccoxxxxxxxxxxxxxxxxxxxxxxl::'.';;.
\r  ';........................................;l'
\r",
"
\r
\r        .;:;;,.,;;::,.
\r     .;':;........'co:.
\r   .clc;'':cllllc::,.':c.
\r  .lo;;o:coxdllllllc;''::,,.
\r.c:'.,cl,.'l:',,;;'......cO;
\rdo;';oxoc;:l;;llllc'.';;'.,;.
\rc..ckkkkkkkd,;llllc'.:kkd;.':c.
\r'.,okkkkkkkkc;lllll,.:kkkdl,cO;
\r..;xkkkkkkkkc,ccll:,;okkkkk:,co,
\r..,dkkkkkkkkc..,;,'ckkkkkkkc;ll.
\r..'okkkkkkkko,....'okkkkkkkc,:c.
\rc..ckkkkkkkkkdl;,:okkkkkkkkd,.',';.
\rd..':lxkkkkkkkkxxkkkkkkkkkkkdoc;,;'..'.,.
\ro...'';llllldkkkkkkkkkkkkkkkkkkdll;..'cdo.
\ro..,l;'''''';dkkkkkkkkkkkkkkkkkkkkdlc,..;lc.
\ro..;lc;;;;;;,,;clllllllllllllllllllllc'..,:c.
\ro..........................................;'
\r",
"
\r
\r           .,,,,,,,,,.
\r         .ckKxodooxOOdcc.
\r      .cclooc'....';;cool.
\r     .loc;;;;clllllc;;;;;:;,.
\r   .c:'.,okd;;cdo:::::cl,..oc
\r  .:o;';okkx;';;,';::;'....,:,.
\r  co..ckkkkkddkc,cclll;.,c:,:o:.
\r  co..ckkkkkkkk:,cllll;.:kkd,.':c.
\r.,:;.,okkkkkkkk:,cclll;.ckkkdl;;o:.
\rcNo..ckkkkkkkkko,.;loc,.ckkkkkc..oc
\r,dd;.:kkkkkkkkkx;..;:,.'lkkkkko,.:,
\r  ;:.ckkkkkkkkkkc.....;ldkkkkkk:.,'
\r,dc..'okkkkkkkkkxoc;;cxkkkkkkkkc..,;,.
\rkNo..':lllllldkkkkkkkkkkkkkkkkkdcc,.;l.
\rKOc,c;''''''';lldkkkkkkkkkkkkkkkkkc..;lc.
\rxx:':;;;;,.,,...,;;cllllllllllllllc;'.;od,
\rcNo.....................................oc
\r",
"
\r
\r
\r                   .ccccccc.
\r               .ccckNKOOOOkdcc.
\r            .;;cc:ccccccc:,:c::,,.
\r         .c;:;.,cccllxOOOxlllc,;ol.
\r        .lkc,coxo:;oOOxooooooo;..:,
\r      .cdc.,dOOOc..cOd,.',,;'....':l.
\r      cNx'.lOOOOxlldOc..;lll;.....cO;
\r     ,do;,:dOOOOOOOOOl'':lll;..:d:''c,
\r     co..lOOOOOOOOOOOl'':lll;.'lOd,.cd.
\r     co.'dOOOOOOOOOOOo,.;llc,.,dOOc..dc
\r     co..lOOOOOOOOOOOOc.';:,..cOOOl..oc
\r   .,:;.'::lxOOOOOOOOOo:'...,:oOOOc.'dc
\r   ;Oc..cl'':lldOOOOOOOOdcclxOOOOx,.cd.
\r  .:;';lxl''''':lldOOOOOOOOOOOOOOc..oc
\r,dl,.'cooc:::,....,::coooooooooooc'.c:
\rcNo.................................oc
\r",
"
\r
\r
\r
\r                        .cccccccc.
\r                  .,,,;;cc:cccccc:;;,.
\r                .cdxo;..,::cccc::,..;l.
\r               ,do:,,:c:coxxdllll:;,';:,.
\r             .cl;.,oxxc'.,cc,.';;;'...oNc
\r             ;Oc..cxxxc'.,c;..;lll;...cO;
\r           .;;',:ldxxxdoldxc..;lll:'...'c,
\r           ;c..cxxxxkxxkxxxc'.;lll:'','.cdc.
\r         .c;.;odxxxxxxxxxxxd;.,cll;.,l:.'dNc
\r        .:,''ccoxkxxkxxxxxxx:..,:;'.:xc..oNc
\r      .lc,.'lc':dxxxkxxxxxxxol,...',lx:..dNc
\r     .:,',coxoc;;ccccoxxxxxxxxo:::oxxo,.cdc.
\r  .;':;.'oxxxxxc''''';cccoxxxxxxxxxxxc..oc
\r,do:'..,:llllll:;;;;;;,..,;:lllllllll;..oc
\rcNo.....................................oc
\r",
"
\r
\r
\r                              .ccccc.
\r                         .cc;'coooxkl;.
\r                     .:c:::c:,,,,,;c;;,.'.
\r                   .clc,',:,..:xxocc;'..c;
\r                  .c:,';:ox:..:c,,,,,,...cd,
\r                .c:'.,oxxxxl::l:.,loll;..;ol.
\r                ;Oc..:xxxxxxxxx:.,llll,....oc
\r             .,;,',:loxxxxxxxxx:.,llll;.,,.'ld,
\r            .lo;..:xxxxxxxxxxxx:.'cllc,.:l:'cO;
\r           .:;...'cxxxxxxxxxxxxoc;,::,..cdl;;l'
\r         .cl;':,'';oxxxxxxdxxxxxx:....,cooc,cO;
\r     .,,,::;,lxoc:,,:lxxxxxxxxxxxo:,,;lxxl;'oNc
\r   .cdxo;':lxxxxxxc'';cccccoxxxxxxxxxxxxo,.;lc.
\r  .loc'.'lxxxxxxxxocc;''''';ccoxxxxxxxxx:..oc
\rolc,..',:cccccccccccc:;;;;;;;;:ccccccccc,.'c,
\rOl;......................................;l'
\r",
"
\r
\r                              ,ddoodd,
\r                         .cc' ,ooccoo,'cc.
\r                      .ccldo;...',,...;oxdc.
\r                   .,,:cc;.,'..;lol;;,'..lkl.
\r                  .dOc';:ccl;..;dl,.''.....oc
\r                .,lc',cdddddlccld;.,;c::'..,cc:.
\r                cNo..:ddddddddddd;':clll;,c,';xc
\r               .lo;,clddddddddddd;':clll;:kc..;'
\r             .,c;..:ddddddddddddd:';clll,;ll,..
\r             ;Oc..';:ldddddddddddl,.,c:;';dd;..
\r           .''',:c:,'cdddddddddddo:,''..'cdd;..
\r         .cdc';lddd:';lddddddddddddd;.';lddl,..
\r      .,;::;,cdddddol;;lllllodddddddlcldddd:.'l;
\r     .dOc..,lddddddddlcc:;'';cclddddddddddd;;ll.
\r   .coc,;::ldddddddddddddlcccc:ldddddddddl:,cO;
\r,xl::,..,cccccccccccccccccccccccccccccccc:;':xx,
\rcNd.........................................;lOc
\r",];
}
