#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use HavePANExt::*;
use HasUnprivileged::*;
use common::*;
pub fn AArch64_S1IndirectBasePermissions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    walkstate: ProductType96e7acababe246a1,
    walkparams: ProductTypeef284266e139aee2,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductTypea231b9ca5c98dc3c {
    #[derive(Default)]
    struct FunctionState {
        pwxn: bool,
        ga_12686: ProductTypede60d0d1f6e7c94c,
        ga_12667: ProductTypede60d0d1f6e7c94c,
        uwxnshadow_292: bool,
        ga_12692: ProductTypede60d0d1f6e7c94c,
        gs_17157: bool,
        ga_12663: ProductTypede60d0d1f6e7c94c,
        gs_17130: bool,
        ga_12677: ProductTypede60d0d1f6e7c94c,
        ga_12737: ProductTypeda0231e9dc169f81,
        ga_12691: ProductTypede60d0d1f6e7c94c,
        wxn: bool,
        permissions: ProductTypebf05c51f33174538,
        ga_12668: ProductTypede60d0d1f6e7c94c,
        ga_12690: ProductTypede60d0d1f6e7c94c,
        ga_12685: ProductTypede60d0d1f6e7c94c,
        p_overlay: bool,
        x: bool,
        gs_17158: bool,
        ux: bool,
        ga_12669: ProductTypede60d0d1f6e7c94c,
        s1perms: ProductTypea231b9ca5c98dc3c,
        overlay: bool,
        ugcs: bool,
        ga_12672: ProductTypede60d0d1f6e7c94c,
        ga_12679: u8,
        gs_17129: bool,
        gs_17156: bool,
        ga_12675: ProductTypede60d0d1f6e7c94c,
        gs_17159: bool,
        ga_12687: ProductTypede60d0d1f6e7c94c,
        ga_12673: ProductTypede60d0d1f6e7c94c,
        ga_12706: ProductTypede60d0d1f6e7c94c,
        ga_12694: ProductTypede60d0d1f6e7c94c,
        uw: bool,
        uwxn: bool,
        ga_12680: ProductTypede60d0d1f6e7c94c,
        r: bool,
        ga_12676: ProductTypede60d0d1f6e7c94c,
        ga_12670: ProductTypede60d0d1f6e7c94c,
        gs_17131: bool,
        ga_12664: ProductTypede60d0d1f6e7c94c,
        ga_12730: ProductTypeda0231e9dc169f81,
        ga_12695: ProductTypede60d0d1f6e7c94c,
        ga_12671: ProductTypede60d0d1f6e7c94c,
        gs_17126: bool,
        ga_12723: ProductTypeda0231e9dc169f81,
        w: bool,
        gs_17160: bool,
        ga_12681: ProductTypede60d0d1f6e7c94c,
        ga_12717: ProductType8b847afc727d5818,
        ga_12682: ProductTypede60d0d1f6e7c94c,
        ga_12661: u8,
        ga_12662: ProductTypede60d0d1f6e7c94c,
        gs_17133: bool,
        pw: bool,
        ga_12665: ProductTypede60d0d1f6e7c94c,
        u_overlayshadow_293: bool,
        u_overlay: bool,
        ga_12666: ProductTypede60d0d1f6e7c94c,
        ga_12674: ProductTypede60d0d1f6e7c94c,
        px: bool,
        pr: bool,
        ga_12684: ProductTypede60d0d1f6e7c94c,
        ga_12705: ProductTypede60d0d1f6e7c94c,
        gcs: bool,
        pgcs: bool,
        ga_12689: ProductTypede60d0d1f6e7c94c,
        ga_12721: ProductType74f83332f678823c,
        gs_17128: bool,
        gs_17127: bool,
        ga_12683: ProductTypede60d0d1f6e7c94c,
        ur: bool,
        ga_12688: ProductTypede60d0d1f6e7c94c,
        ga_12693: ProductTypede60d0d1f6e7c94c,
        ga_12720: ProductType74f83332f678823c,
        regime: u32,
        walkstate: ProductType96e7acababe246a1,
        walkparams: ProductTypeef284266e139aee2,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        regime,
        walkstate,
        walkparams,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_0_0: read-var walkstate.9:struct
        let s_0_0: ProductTypebf05c51f33174538 = fn_state.walkstate._9;
        // D s_0_1: write-var permissions <= s_0_0
        fn_state.permissions = s_0_0;
        // D s_0_2: read-var permissions.4:struct
        let s_0_2: u8 = fn_state.permissions._4;
        // D s_0_3: write-var ga#12661 <= s_0_2
        fn_state.ga_12661 = s_0_2;
        // D s_0_4: read-var ga#12661:u8
        let s_0_4: u8 = fn_state.ga_12661;
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 4u16);
        // C s_0_6: const #0u : u8
        let s_0_6: u8 = 0;
        // C s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 4u16);
        // D s_0_8: cmp-eq s_0_5 s_0_7
        let s_0_8: bool = ((s_0_5) == (s_0_7));
        // D s_0_9: not s_0_8
        let s_0_9: bool = !s_0_8;
        // N s_0_10: branch s_0_9 b100 b1
        if s_0_9 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // C s_1_1: const #0u : u8
        let s_1_1: bool = false;
        // C s_1_2: const #0u : u8
        let s_1_2: bool = false;
        // C s_1_3: const #0u : u8
        let s_1_3: bool = false;
        // D s_1_4: create-product struct = ["s_1_0", "s_1_1", "s_1_2", "s_1_3"]
        let s_1_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_1_0,
            _1: s_1_1,
            _2: s_1_2,
            _3: s_1_3,
        };
        // D s_1_5: write-var ga#12662 <= s_1_4
        fn_state.ga_12662 = s_1_4;
        // D s_1_6: read-var ga#12662.0:struct
        let s_1_6: bool = fn_state.ga_12662._0;
        // D s_1_7: read-var ga#12662.1:struct
        let s_1_7: bool = fn_state.ga_12662._1;
        // D s_1_8: read-var ga#12662.2:struct
        let s_1_8: bool = fn_state.ga_12662._2;
        // D s_1_9: read-var ga#12662.3:struct
        let s_1_9: bool = fn_state.ga_12662._3;
        // D s_1_10: write-var pr <= s_1_6
        fn_state.pr = s_1_6;
        // D s_1_11: write-var pw <= s_1_7
        fn_state.pw = s_1_7;
        // D s_1_12: write-var px <= s_1_8
        fn_state.px = s_1_8;
        // D s_1_13: write-var pgcs <= s_1_9
        fn_state.pgcs = s_1_9;
        // N s_1_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_2_0: read-var permissions.4:struct
        let s_2_0: u8 = fn_state.permissions._4;
        // C s_2_1: const #3s : i
        let s_2_1: i128 = 3;
        // D s_2_2: cast zx s_2_0 -> bv
        let s_2_2: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_1 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_1)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // D s_2_19: not s_2_18
        let s_2_19: Bits = !s_2_18;
        // D s_2_20: cast reint s_2_19 -> u8
        let s_2_20: bool = ((s_2_19.value()) != 0);
        // D s_2_21: write-var p_overlay <= s_2_20
        fn_state.p_overlay = s_2_20;
        // D s_2_22: read-var permissions.4:struct
        let s_2_22: u8 = fn_state.permissions._4;
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 4u16);
        // C s_2_24: const #6u : u8
        let s_2_24: u8 = 6;
        // C s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 4u16);
        // D s_2_26: cmp-eq s_2_23 s_2_25
        let s_2_26: bool = ((s_2_23) == (s_2_25));
        // N s_2_27: branch s_2_26 b99 b3
        if s_2_26 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var pwxn <= s_3_0
        fn_state.pwxn = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_4_0: read-var regime:u32
        let s_4_0: u32 = fn_state.regime;
        // D s_4_1: call HasUnprivileged(s_4_0)
        let s_4_1: bool = HasUnprivileged(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b34 b5
        if s_4_1 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_6_0: read-var uwxn:u8
        let s_6_0: bool = fn_state.uwxn;
        // D s_6_1: write-var uwxnshadow#292 <= s_6_0
        fn_state.uwxnshadow_292 = s_6_0;
        // D s_6_2: read-var u_overlay:u8
        let s_6_2: bool = fn_state.u_overlay;
        // D s_6_3: write-var u_overlayshadow#293 <= s_6_2
        fn_state.u_overlayshadow_293 = s_6_2;
        // D s_6_4: read-var accdesc.8:struct
        let s_6_4: u8 = fn_state.accdesc._8;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 2u16);
        // C s_6_6: const #448u : u32
        let s_6_6: u32 = 448;
        // D s_6_7: read-reg s_6_6:u8
        let s_6_7: u8 = {
            let value = state.read_register::<u8>(s_6_6 as isize);
            tracer.read_register(s_6_6 as isize, value);
            value
        };
        // D s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 2u16);
        // D s_6_9: cmp-eq s_6_5 s_6_8
        let s_6_9: bool = ((s_6_5) == (s_6_8));
        // N s_6_10: branch s_6_9 b33 b7
        if s_6_9 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_7_0: read-var pr:u8
        let s_7_0: bool = fn_state.pr;
        // D s_7_1: read-var pw:u8
        let s_7_1: bool = fn_state.pw;
        // D s_7_2: read-var px:u8
        let s_7_2: bool = fn_state.px;
        // D s_7_3: read-var pgcs:u8
        let s_7_3: bool = fn_state.pgcs;
        // D s_7_4: read-var pwxn:u8
        let s_7_4: bool = fn_state.pwxn;
        // D s_7_5: read-var p_overlay:u8
        let s_7_5: bool = fn_state.p_overlay;
        // D s_7_6: create-product struct = ["s_7_0", "s_7_1", "s_7_2", "s_7_3", "s_7_4", "s_7_5"]
        let s_7_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_7_0,
            _1: s_7_1,
            _2: s_7_2,
            _3: s_7_3,
            _4: s_7_4,
            _5: s_7_5,
        };
        // D s_7_7: write-var ga#12721 <= s_7_6
        fn_state.ga_12721 = s_7_6;
        // D s_7_8: read-var ga#12721.0:struct
        let s_7_8: bool = fn_state.ga_12721._0;
        // D s_7_9: read-var ga#12721.1:struct
        let s_7_9: bool = fn_state.ga_12721._1;
        // D s_7_10: read-var ga#12721.2:struct
        let s_7_10: bool = fn_state.ga_12721._2;
        // D s_7_11: read-var ga#12721.3:struct
        let s_7_11: bool = fn_state.ga_12721._3;
        // D s_7_12: read-var ga#12721.4:struct
        let s_7_12: bool = fn_state.ga_12721._4;
        // D s_7_13: read-var ga#12721.5:struct
        let s_7_13: bool = fn_state.ga_12721._5;
        // D s_7_14: write-var r <= s_7_8
        fn_state.r = s_7_8;
        // D s_7_15: write-var w <= s_7_9
        fn_state.w = s_7_9;
        // D s_7_16: write-var x <= s_7_10
        fn_state.x = s_7_10;
        // D s_7_17: write-var gcs <= s_7_11
        fn_state.gcs = s_7_11;
        // D s_7_18: write-var wxn <= s_7_12
        fn_state.wxn = s_7_12;
        // D s_7_19: write-var overlay <= s_7_13
        fn_state.overlay = s_7_13;
        // N s_7_20: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_8_0: read-var accdesc.25:struct
        let s_8_0: u32 = fn_state.accdesc._25;
        // C s_8_1: const #3u : u32
        let s_8_1: u32 = 3;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b32 b9
        if s_8_2 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#17156 <= s_9_0
        fn_state.gs_17156 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_10_0: read-var gs#17156:u8
        let s_10_0: bool = fn_state.gs_17156;
        // N s_10_1: branch s_10_0 b31 b11
        if s_10_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_12_0: read-var accdesc.25:struct
        let s_12_0: u32 = fn_state.accdesc._25;
        // C s_12_1: const #1u : u32
        let s_12_1: u32 = 1;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // N s_12_3: branch s_12_2 b30 b13
        if s_12_2 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#17157 <= s_13_0
        fn_state.gs_17157 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_14_0: read-var gs#17157:u8
        let s_14_0: bool = fn_state.gs_17157;
        // N s_14_1: branch s_14_0 b29 b15
        if s_14_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_16_0: read-var accdesc.25:struct
        let s_16_0: u32 = fn_state.accdesc._25;
        // C s_16_1: const #2u : u32
        let s_16_1: u32 = 2;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // N s_16_3: branch s_16_2 b25 b17
        if s_16_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#17159 <= s_17_0
        fn_state.gs_17159 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_18_0: read-var gs#17159:u8
        let s_18_0: bool = fn_state.gs_17159;
        // N s_18_1: branch s_18_0 b24 b19
        if s_18_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#17160 <= s_19_0
        fn_state.gs_17160 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_20_0: read-var gs#17160:u8
        let s_20_0: bool = fn_state.gs_17160;
        // N s_20_1: branch s_20_0 b23 b21
        if s_20_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_22_0: read-var x:u8
        let s_22_0: bool = fn_state.x;
        // D s_22_1: read-var gcs:u8
        let s_22_1: bool = fn_state.gcs;
        // D s_22_2: read-var r:u8
        let s_22_2: bool = fn_state.r;
        // D s_22_3: write-var s1perms.5 <= s_22_2
        fn_state.s1perms._5 = s_22_2;
        // D s_22_4: read-var w:u8
        let s_22_4: bool = fn_state.w;
        // D s_22_5: write-var s1perms.6 <= s_22_4
        fn_state.s1perms._6 = s_22_4;
        // D s_22_6: write-var s1perms.8 <= s_22_0
        fn_state.s1perms._8 = s_22_0;
        // D s_22_7: write-var s1perms.0 <= s_22_1
        fn_state.s1perms._0 = s_22_1;
        // D s_22_8: read-var wxn:u8
        let s_22_8: bool = fn_state.wxn;
        // D s_22_9: write-var s1perms.7 <= s_22_8
        fn_state.s1perms._7 = s_22_8;
        // D s_22_10: read-var overlay:u8
        let s_22_10: bool = fn_state.overlay;
        // D s_22_11: cast zx s_22_10 -> bv
        let s_22_11: Bits = Bits::new(s_22_10 as u128, 1u16);
        // C s_22_12: const #1u : u8
        let s_22_12: bool = true;
        // C s_22_13: cast zx s_22_12 -> bv
        let s_22_13: Bits = Bits::new(s_22_12 as u128, 1u16);
        // D s_22_14: cmp-eq s_22_11 s_22_13
        let s_22_14: bool = ((s_22_11) == (s_22_13));
        // D s_22_15: write-var s1perms.2 <= s_22_14
        fn_state.s1perms._2 = s_22_14;
        // D s_22_16: read-var s1perms:struct
        let s_22_16: ProductTypea231b9ca5c98dc3c = fn_state.s1perms;
        // N s_22_17: return s_22_16
        return s_22_16;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var x <= s_23_0
        fn_state.x = s_23_0;
        // C s_23_2: const #0u : u8
        let s_23_2: bool = false;
        // D s_23_3: write-var gcs <= s_23_2
        fn_state.gcs = s_23_2;
        // N s_23_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_24_0: read-var walkstate.0:struct
        let s_24_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_24_1: write-var ga#12737 <= s_24_0
        fn_state.ga_12737 = s_24_0;
        // D s_24_2: read-var ga#12737.1:struct
        let s_24_2: u32 = fn_state.ga_12737._1;
        // C s_24_3: const #3u : u32
        let s_24_3: u32 = 3;
        // D s_24_4: cmp-eq s_24_2 s_24_3
        let s_24_4: bool = ((s_24_2) == (s_24_3));
        // D s_24_5: write-var gs#17160 <= s_24_4
        fn_state.gs_17160 = s_24_4;
        // N s_24_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_25_0: read-var regime:u32
        let s_25_0: u32 = fn_state.regime;
        // C s_25_1: const #2u : u32
        let s_25_1: u32 = 2;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // N s_25_3: branch s_25_2 b28 b26
        if s_25_2 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_26_0: read-var regime:u32
        let s_26_0: u32 = fn_state.regime;
        // C s_26_1: const #3u : u32
        let s_26_1: u32 = 3;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // D s_26_3: write-var gs#17158 <= s_26_2
        fn_state.gs_17158 = s_26_2;
        // N s_26_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_27_0: read-var gs#17158:u8
        let s_27_0: bool = fn_state.gs_17158;
        // D s_27_1: write-var gs#17159 <= s_27_0
        fn_state.gs_17159 = s_27_0;
        // N s_27_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#17158 <= s_28_0
        fn_state.gs_17158 = s_28_0;
        // N s_28_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var x <= s_29_0
        fn_state.x = s_29_0;
        // C s_29_2: const #0u : u8
        let s_29_2: bool = false;
        // D s_29_3: write-var gcs <= s_29_2
        fn_state.gcs = s_29_2;
        // N s_29_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_30_0: read-var walkstate.0:struct
        let s_30_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_30_1: write-var ga#12730 <= s_30_0
        fn_state.ga_12730 = s_30_0;
        // D s_30_2: read-var ga#12730.1:struct
        let s_30_2: u32 = fn_state.ga_12730._1;
        // C s_30_3: const #2u : u32
        let s_30_3: u32 = 2;
        // D s_30_4: cmp-eq s_30_2 s_30_3
        let s_30_4: bool = ((s_30_2) == (s_30_3));
        // D s_30_5: write-var gs#17157 <= s_30_4
        fn_state.gs_17157 = s_30_4;
        // N s_30_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_31_0: read-var walkparams.30:struct
        let s_31_0: bool = fn_state.walkparams._30;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // D s_31_2: not s_31_1
        let s_31_2: Bits = !s_31_1;
        // D s_31_3: cast reint s_31_2 -> u8
        let s_31_3: bool = ((s_31_2.value()) != 0);
        // D s_31_4: read-var x:u8
        let s_31_4: bool = fn_state.x;
        // D s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 1u16);
        // D s_31_6: cast zx s_31_3 -> bv
        let s_31_6: Bits = Bits::new(s_31_3 as u128, 1u16);
        // D s_31_7: and s_31_5 s_31_6
        let s_31_7: Bits = ((s_31_5) & (s_31_6));
        // D s_31_8: cast reint s_31_7 -> u8
        let s_31_8: bool = ((s_31_7.value()) != 0);
        // D s_31_9: write-var x <= s_31_8
        fn_state.x = s_31_8;
        // C s_31_10: const #0u : u8
        let s_31_10: bool = false;
        // D s_31_11: write-var gcs <= s_31_10
        fn_state.gcs = s_31_10;
        // N s_31_12: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_32_0: read-var walkstate.0:struct
        let s_32_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_32_1: write-var ga#12723 <= s_32_0
        fn_state.ga_12723 = s_32_0;
        // D s_32_2: read-var ga#12723.1:struct
        let s_32_2: u32 = fn_state.ga_12723._1;
        // C s_32_3: const #0u : u32
        let s_32_3: u32 = 0;
        // D s_32_4: cmp-eq s_32_2 s_32_3
        let s_32_4: bool = ((s_32_2) == (s_32_3));
        // D s_32_5: write-var gs#17156 <= s_32_4
        fn_state.gs_17156 = s_32_4;
        // N s_32_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_33_0: read-var ur:u8
        let s_33_0: bool = fn_state.ur;
        // D s_33_1: read-var uw:u8
        let s_33_1: bool = fn_state.uw;
        // D s_33_2: read-var ux:u8
        let s_33_2: bool = fn_state.ux;
        // D s_33_3: read-var ugcs:u8
        let s_33_3: bool = fn_state.ugcs;
        // D s_33_4: read-var uwxnshadow#292:u8
        let s_33_4: bool = fn_state.uwxnshadow_292;
        // D s_33_5: read-var u_overlayshadow#293:u8
        let s_33_5: bool = fn_state.u_overlayshadow_293;
        // D s_33_6: create-product struct = ["s_33_0", "s_33_1", "s_33_2", "s_33_3", "s_33_4", "s_33_5"]
        let s_33_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_33_0,
            _1: s_33_1,
            _2: s_33_2,
            _3: s_33_3,
            _4: s_33_4,
            _5: s_33_5,
        };
        // D s_33_7: write-var ga#12720 <= s_33_6
        fn_state.ga_12720 = s_33_6;
        // D s_33_8: read-var ga#12720.0:struct
        let s_33_8: bool = fn_state.ga_12720._0;
        // D s_33_9: read-var ga#12720.1:struct
        let s_33_9: bool = fn_state.ga_12720._1;
        // D s_33_10: read-var ga#12720.2:struct
        let s_33_10: bool = fn_state.ga_12720._2;
        // D s_33_11: read-var ga#12720.3:struct
        let s_33_11: bool = fn_state.ga_12720._3;
        // D s_33_12: read-var ga#12720.4:struct
        let s_33_12: bool = fn_state.ga_12720._4;
        // D s_33_13: read-var ga#12720.5:struct
        let s_33_13: bool = fn_state.ga_12720._5;
        // D s_33_14: write-var r <= s_33_8
        fn_state.r = s_33_8;
        // D s_33_15: write-var w <= s_33_9
        fn_state.w = s_33_9;
        // D s_33_16: write-var x <= s_33_10
        fn_state.x = s_33_10;
        // D s_33_17: write-var gcs <= s_33_11
        fn_state.gcs = s_33_11;
        // D s_33_18: write-var wxn <= s_33_12
        fn_state.wxn = s_33_12;
        // D s_33_19: write-var overlay <= s_33_13
        fn_state.overlay = s_33_13;
        // N s_33_20: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_34_0: read-var permissions.14:struct
        let s_34_0: u8 = fn_state.permissions._14;
        // D s_34_1: write-var ga#12679 <= s_34_0
        fn_state.ga_12679 = s_34_0;
        // D s_34_2: read-var ga#12679:u8
        let s_34_2: u8 = fn_state.ga_12679;
        // D s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 4u16);
        // C s_34_4: const #0u : u8
        let s_34_4: u8 = 0;
        // C s_34_5: cast zx s_34_4 -> bv
        let s_34_5: Bits = Bits::new(s_34_4 as u128, 4u16);
        // D s_34_6: cmp-eq s_34_3 s_34_5
        let s_34_6: bool = ((s_34_3) == (s_34_5));
        // D s_34_7: not s_34_6
        let s_34_7: bool = !s_34_6;
        // N s_34_8: branch s_34_7 b70 b35
        if s_34_7 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // C s_35_1: const #0u : u8
        let s_35_1: bool = false;
        // C s_35_2: const #0u : u8
        let s_35_2: bool = false;
        // C s_35_3: const #0u : u8
        let s_35_3: bool = false;
        // D s_35_4: create-product struct = ["s_35_0", "s_35_1", "s_35_2", "s_35_3"]
        let s_35_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_35_0,
            _1: s_35_1,
            _2: s_35_2,
            _3: s_35_3,
        };
        // D s_35_5: write-var ga#12680 <= s_35_4
        fn_state.ga_12680 = s_35_4;
        // D s_35_6: read-var ga#12680.0:struct
        let s_35_6: bool = fn_state.ga_12680._0;
        // D s_35_7: read-var ga#12680.1:struct
        let s_35_7: bool = fn_state.ga_12680._1;
        // D s_35_8: read-var ga#12680.2:struct
        let s_35_8: bool = fn_state.ga_12680._2;
        // D s_35_9: read-var ga#12680.3:struct
        let s_35_9: bool = fn_state.ga_12680._3;
        // D s_35_10: write-var ur <= s_35_6
        fn_state.ur = s_35_6;
        // D s_35_11: write-var uw <= s_35_7
        fn_state.uw = s_35_7;
        // D s_35_12: write-var ux <= s_35_8
        fn_state.ux = s_35_8;
        // D s_35_13: write-var ugcs <= s_35_9
        fn_state.ugcs = s_35_9;
        // N s_35_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_36_0: read-var permissions.14:struct
        let s_36_0: u8 = fn_state.permissions._14;
        // C s_36_1: const #3s : i
        let s_36_1: i128 = 3;
        // D s_36_2: cast zx s_36_0 -> bv
        let s_36_2: Bits = Bits::new(s_36_0 as u128, 4u16);
        // C s_36_3: const #1u : u64
        let s_36_3: u64 = 1;
        // D s_36_4: bit-extract s_36_2 s_36_1 s_36_3
        let s_36_4: Bits = (Bits::new(
            ((s_36_2) >> (s_36_1)).value(),
            u16::try_from(s_36_3).unwrap(),
        ));
        // D s_36_5: cast reint s_36_4 -> u8
        let s_36_5: bool = ((s_36_4.value()) != 0);
        // C s_36_6: const #0s : i
        let s_36_6: i128 = 0;
        // C s_36_7: const #0u : u64
        let s_36_7: u64 = 0;
        // D s_36_8: cast zx s_36_5 -> u64
        let s_36_8: u64 = (s_36_5 as u64);
        // C s_36_9: const #1u : u64
        let s_36_9: u64 = 1;
        // D s_36_10: and s_36_8 s_36_9
        let s_36_10: u64 = ((s_36_8) & (s_36_9));
        // D s_36_11: cmp-eq s_36_10 s_36_9
        let s_36_11: bool = ((s_36_10) == (s_36_9));
        // D s_36_12: lsl s_36_8 s_36_6
        let s_36_12: u64 = s_36_8 << s_36_6;
        // D s_36_13: or s_36_7 s_36_12
        let s_36_13: u64 = ((s_36_7) | (s_36_12));
        // D s_36_14: cmpl s_36_12
        let s_36_14: u64 = !s_36_12;
        // D s_36_15: and s_36_7 s_36_14
        let s_36_15: u64 = ((s_36_7) & (s_36_14));
        // D s_36_16: select s_36_11 s_36_13 s_36_15
        let s_36_16: u64 = if s_36_11 { s_36_13 } else { s_36_15 };
        // D s_36_17: cast trunc s_36_16 -> u8
        let s_36_17: bool = ((s_36_16) != 0);
        // D s_36_18: cast zx s_36_17 -> bv
        let s_36_18: Bits = Bits::new(s_36_17 as u128, 1u16);
        // D s_36_19: not s_36_18
        let s_36_19: Bits = !s_36_18;
        // D s_36_20: cast reint s_36_19 -> u8
        let s_36_20: bool = ((s_36_19.value()) != 0);
        // D s_36_21: write-var u_overlay <= s_36_20
        fn_state.u_overlay = s_36_20;
        // D s_36_22: read-var permissions.14:struct
        let s_36_22: u8 = fn_state.permissions._14;
        // D s_36_23: cast zx s_36_22 -> bv
        let s_36_23: Bits = Bits::new(s_36_22 as u128, 4u16);
        // C s_36_24: const #6u : u8
        let s_36_24: u8 = 6;
        // C s_36_25: cast zx s_36_24 -> bv
        let s_36_25: Bits = Bits::new(s_36_24 as u128, 4u16);
        // D s_36_26: cmp-eq s_36_23 s_36_25
        let s_36_26: bool = ((s_36_23) == (s_36_25));
        // N s_36_27: branch s_36_26 b69 b37
        if s_36_26 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var uwxn <= s_37_0
        fn_state.uwxn = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_38_0: read-var px:u8
        let s_38_0: bool = fn_state.px;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #1u : u8
        let s_38_2: bool = true;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // N s_38_5: branch s_38_4 b68 b39
        if s_38_4 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_39_0: read-var pgcs:u8
        let s_39_0: bool = fn_state.pgcs;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#17126 <= s_39_4
        fn_state.gs_17126 = s_39_4;
        // N s_39_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_40_0: read-var gs#17126:u8
        let s_40_0: bool = fn_state.gs_17126;
        // N s_40_1: branch s_40_0 b64 b41
        if s_40_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#17128 <= s_41_0
        fn_state.gs_17128 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_42_0: read-var gs#17128:u8
        let s_42_0: bool = fn_state.gs_17128;
        // N s_42_1: branch s_42_0 b63 b43
        if s_42_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_43_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call HavePANExt(s_44_0)
        let s_44_1: bool = HavePANExt(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b62 b45
        if s_44_1 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#17129 <= s_45_0
        fn_state.gs_17129 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_46_0: read-var gs#17129:u8
        let s_46_0: bool = fn_state.gs_17129;
        // N s_46_1: branch s_46_0 b58 b47
        if s_46_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#17131 <= s_47_0
        fn_state.gs_17131 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_48_0: read-var gs#17131:u8
        let s_48_0: bool = fn_state.gs_17131;
        // N s_48_1: branch s_48_0 b51 b49
        if s_48_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_49_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_50_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_51_0: const #16985u : u32
        let s_51_0: u32 = 16985;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: bool = {
            let value = state.read_register::<bool>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: cast zx s_51_1 -> bv
        let s_51_2: Bits = Bits::new(s_51_1 as u128, 1u16);
        // C s_51_3: const #1u : u8
        let s_51_3: bool = true;
        // C s_51_4: cast zx s_51_3 -> bv
        let s_51_4: Bits = Bits::new(s_51_3 as u128, 1u16);
        // D s_51_5: cmp-eq s_51_2 s_51_4
        let s_51_5: bool = ((s_51_2) == (s_51_4));
        // N s_51_6: branch s_51_5 b57 b52
        if s_51_5 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#17133 <= s_52_0
        fn_state.gs_17133 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_53_0: read-var gs#17133:u8
        let s_53_0: bool = fn_state.gs_17133;
        // N s_53_1: branch s_53_0 b56 b54
        if s_53_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_54_0: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_55_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // C s_56_1: const #0u : u8
        let s_56_1: bool = false;
        // D s_56_2: create-product struct = ["s_56_0", "s_56_1"]
        let s_56_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_56_0,
            _1: s_56_1,
        };
        // D s_56_3: write-var ga#12717 <= s_56_2
        fn_state.ga_12717 = s_56_2;
        // D s_56_4: read-var ga#12717.0:struct
        let s_56_4: bool = fn_state.ga_12717._0;
        // D s_56_5: read-var ga#12717.1:struct
        let s_56_5: bool = fn_state.ga_12717._1;
        // D s_56_6: write-var pr <= s_56_4
        fn_state.pr = s_56_4;
        // D s_56_7: write-var pw <= s_56_5
        fn_state.pw = s_56_5;
        // N s_56_8: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_57_0: read-var permissions.14:struct
        let s_57_0: u8 = fn_state.permissions._14;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 4u16);
        // C s_57_2: const #0u : u8
        let s_57_2: u8 = 0;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 4u16);
        // D s_57_4: cmp-ne s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) != (s_57_3));
        // D s_57_5: write-var gs#17133 <= s_57_4
        fn_state.gs_17133 = s_57_4;
        // N s_57_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_58_0: read-var regime:u32
        let s_58_0: u32 = fn_state.regime;
        // C s_58_1: const #4u : u32
        let s_58_1: u32 = 4;
        // D s_58_2: cmp-eq s_58_0 s_58_1
        let s_58_2: bool = ((s_58_0) == (s_58_1));
        // N s_58_3: branch s_58_2 b61 b59
        if s_58_2 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#17130 <= s_59_0
        fn_state.gs_17130 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_60_0: read-var gs#17130:u8
        let s_60_0: bool = fn_state.gs_17130;
        // D s_60_1: not s_60_0
        let s_60_1: bool = !s_60_0;
        // D s_60_2: write-var gs#17131 <= s_60_1
        fn_state.gs_17131 = s_60_1;
        // N s_60_3: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_61_0: read-var walkparams.22:struct
        let s_61_0: bool = fn_state.walkparams._22;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 1u16);
        // C s_61_2: const #1u : u8
        let s_61_2: bool = true;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // D s_61_4: cmp-eq s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) == (s_61_3));
        // D s_61_5: write-var gs#17130 <= s_61_4
        fn_state.gs_17130 = s_61_4;
        // N s_61_6: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_62_0: read-var accdesc.20:struct
        let s_62_0: bool = fn_state.accdesc._20;
        // D s_62_1: write-var gs#17129 <= s_62_0
        fn_state.gs_17129 = s_62_0;
        // N s_62_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // C s_63_1: const #0u : u8
        let s_63_1: bool = false;
        // C s_63_2: const #0u : u8
        let s_63_2: bool = false;
        // C s_63_3: const #0u : u8
        let s_63_3: bool = false;
        // D s_63_4: create-product struct = ["s_63_0", "s_63_1", "s_63_2", "s_63_3"]
        let s_63_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_63_0,
            _1: s_63_1,
            _2: s_63_2,
            _3: s_63_3,
        };
        // D s_63_5: write-var ga#12705 <= s_63_4
        fn_state.ga_12705 = s_63_4;
        // D s_63_6: read-var ga#12705.0:struct
        let s_63_6: bool = fn_state.ga_12705._0;
        // D s_63_7: read-var ga#12705.1:struct
        let s_63_7: bool = fn_state.ga_12705._1;
        // D s_63_8: read-var ga#12705.2:struct
        let s_63_8: bool = fn_state.ga_12705._2;
        // D s_63_9: read-var ga#12705.3:struct
        let s_63_9: bool = fn_state.ga_12705._3;
        // D s_63_10: write-var pr <= s_63_6
        fn_state.pr = s_63_6;
        // D s_63_11: write-var pw <= s_63_7
        fn_state.pw = s_63_7;
        // D s_63_12: write-var px <= s_63_8
        fn_state.px = s_63_8;
        // D s_63_13: write-var pgcs <= s_63_9
        fn_state.pgcs = s_63_9;
        // C s_63_14: const #0u : u8
        let s_63_14: bool = false;
        // C s_63_15: const #0u : u8
        let s_63_15: bool = false;
        // C s_63_16: const #0u : u8
        let s_63_16: bool = false;
        // C s_63_17: const #0u : u8
        let s_63_17: bool = false;
        // D s_63_18: create-product struct = ["s_63_14", "s_63_15", "s_63_16", "s_63_17"]
        let s_63_18: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_63_14,
            _1: s_63_15,
            _2: s_63_16,
            _3: s_63_17,
        };
        // D s_63_19: write-var ga#12706 <= s_63_18
        fn_state.ga_12706 = s_63_18;
        // D s_63_20: read-var ga#12706.0:struct
        let s_63_20: bool = fn_state.ga_12706._0;
        // D s_63_21: read-var ga#12706.1:struct
        let s_63_21: bool = fn_state.ga_12706._1;
        // D s_63_22: read-var ga#12706.2:struct
        let s_63_22: bool = fn_state.ga_12706._2;
        // D s_63_23: read-var ga#12706.3:struct
        let s_63_23: bool = fn_state.ga_12706._3;
        // D s_63_24: write-var ur <= s_63_20
        fn_state.ur = s_63_20;
        // D s_63_25: write-var uw <= s_63_21
        fn_state.uw = s_63_21;
        // D s_63_26: write-var ux <= s_63_22
        fn_state.ux = s_63_22;
        // D s_63_27: write-var ugcs <= s_63_23
        fn_state.ugcs = s_63_23;
        // N s_63_28: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_64_0: read-var uw:u8
        let s_64_0: bool = fn_state.uw;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // N s_64_5: branch s_64_4 b67 b65
        if s_64_4 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_65_0: read-var ugcs:u8
        let s_65_0: bool = fn_state.ugcs;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #1u : u8
        let s_65_2: bool = true;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#17127 <= s_65_4
        fn_state.gs_17127 = s_65_4;
        // N s_65_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_66_0: read-var gs#17127:u8
        let s_66_0: bool = fn_state.gs_17127;
        // D s_66_1: write-var gs#17128 <= s_66_0
        fn_state.gs_17128 = s_66_0;
        // N s_66_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#17127 <= s_67_0
        fn_state.gs_17127 = s_67_0;
        // N s_67_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#17126 <= s_68_0
        fn_state.gs_17126 = s_68_0;
        // N s_68_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var uwxn <= s_69_0
        fn_state.uwxn = s_69_0;
        // N s_69_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_70_0: read-var ga#12679:u8
        let s_70_0: u8 = fn_state.ga_12679;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 4u16);
        // C s_70_2: const #1u : u8
        let s_70_2: u8 = 1;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 4u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: not s_70_4
        let s_70_5: bool = !s_70_4;
        // N s_70_6: branch s_70_5 b72 b71
        if s_70_5 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // C s_71_1: const #0u : u8
        let s_71_1: bool = false;
        // C s_71_2: const #0u : u8
        let s_71_2: bool = false;
        // C s_71_3: const #0u : u8
        let s_71_3: bool = false;
        // D s_71_4: create-product struct = ["s_71_0", "s_71_1", "s_71_2", "s_71_3"]
        let s_71_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_71_0,
            _1: s_71_1,
            _2: s_71_2,
            _3: s_71_3,
        };
        // D s_71_5: write-var ga#12681 <= s_71_4
        fn_state.ga_12681 = s_71_4;
        // D s_71_6: read-var ga#12681.0:struct
        let s_71_6: bool = fn_state.ga_12681._0;
        // D s_71_7: read-var ga#12681.1:struct
        let s_71_7: bool = fn_state.ga_12681._1;
        // D s_71_8: read-var ga#12681.2:struct
        let s_71_8: bool = fn_state.ga_12681._2;
        // D s_71_9: read-var ga#12681.3:struct
        let s_71_9: bool = fn_state.ga_12681._3;
        // D s_71_10: write-var ur <= s_71_6
        fn_state.ur = s_71_6;
        // D s_71_11: write-var uw <= s_71_7
        fn_state.uw = s_71_7;
        // D s_71_12: write-var ux <= s_71_8
        fn_state.ux = s_71_8;
        // D s_71_13: write-var ugcs <= s_71_9
        fn_state.ugcs = s_71_9;
        // N s_71_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_72_0: read-var ga#12679:u8
        let s_72_0: u8 = fn_state.ga_12679;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 4u16);
        // C s_72_2: const #2u : u8
        let s_72_2: u8 = 2;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 4u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: not s_72_4
        let s_72_5: bool = !s_72_4;
        // N s_72_6: branch s_72_5 b74 b73
        if s_72_5 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // C s_73_1: const #0u : u8
        let s_73_1: bool = false;
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: const #0u : u8
        let s_73_3: bool = false;
        // D s_73_4: create-product struct = ["s_73_0", "s_73_1", "s_73_2", "s_73_3"]
        let s_73_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_73_0,
            _1: s_73_1,
            _2: s_73_2,
            _3: s_73_3,
        };
        // D s_73_5: write-var ga#12682 <= s_73_4
        fn_state.ga_12682 = s_73_4;
        // D s_73_6: read-var ga#12682.0:struct
        let s_73_6: bool = fn_state.ga_12682._0;
        // D s_73_7: read-var ga#12682.1:struct
        let s_73_7: bool = fn_state.ga_12682._1;
        // D s_73_8: read-var ga#12682.2:struct
        let s_73_8: bool = fn_state.ga_12682._2;
        // D s_73_9: read-var ga#12682.3:struct
        let s_73_9: bool = fn_state.ga_12682._3;
        // D s_73_10: write-var ur <= s_73_6
        fn_state.ur = s_73_6;
        // D s_73_11: write-var uw <= s_73_7
        fn_state.uw = s_73_7;
        // D s_73_12: write-var ux <= s_73_8
        fn_state.ux = s_73_8;
        // D s_73_13: write-var ugcs <= s_73_9
        fn_state.ugcs = s_73_9;
        // N s_73_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_74_0: read-var ga#12679:u8
        let s_74_0: u8 = fn_state.ga_12679;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 4u16);
        // C s_74_2: const #3u : u8
        let s_74_2: u8 = 3;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 4u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: not s_74_4
        let s_74_5: bool = !s_74_4;
        // N s_74_6: branch s_74_5 b76 b75
        if s_74_5 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // C s_75_1: const #0u : u8
        let s_75_1: bool = false;
        // C s_75_2: const #1u : u8
        let s_75_2: bool = true;
        // C s_75_3: const #0u : u8
        let s_75_3: bool = false;
        // D s_75_4: create-product struct = ["s_75_0", "s_75_1", "s_75_2", "s_75_3"]
        let s_75_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_75_0,
            _1: s_75_1,
            _2: s_75_2,
            _3: s_75_3,
        };
        // D s_75_5: write-var ga#12683 <= s_75_4
        fn_state.ga_12683 = s_75_4;
        // D s_75_6: read-var ga#12683.0:struct
        let s_75_6: bool = fn_state.ga_12683._0;
        // D s_75_7: read-var ga#12683.1:struct
        let s_75_7: bool = fn_state.ga_12683._1;
        // D s_75_8: read-var ga#12683.2:struct
        let s_75_8: bool = fn_state.ga_12683._2;
        // D s_75_9: read-var ga#12683.3:struct
        let s_75_9: bool = fn_state.ga_12683._3;
        // D s_75_10: write-var ur <= s_75_6
        fn_state.ur = s_75_6;
        // D s_75_11: write-var uw <= s_75_7
        fn_state.uw = s_75_7;
        // D s_75_12: write-var ux <= s_75_8
        fn_state.ux = s_75_8;
        // D s_75_13: write-var ugcs <= s_75_9
        fn_state.ugcs = s_75_9;
        // N s_75_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_76_0: read-var ga#12679:u8
        let s_76_0: u8 = fn_state.ga_12679;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 4u16);
        // C s_76_2: const #4u : u8
        let s_76_2: u8 = 4;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 4u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: not s_76_4
        let s_76_5: bool = !s_76_4;
        // N s_76_6: branch s_76_5 b78 b77
        if s_76_5 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // C s_77_1: const #0u : u8
        let s_77_1: bool = false;
        // C s_77_2: const #0u : u8
        let s_77_2: bool = false;
        // C s_77_3: const #0u : u8
        let s_77_3: bool = false;
        // D s_77_4: create-product struct = ["s_77_0", "s_77_1", "s_77_2", "s_77_3"]
        let s_77_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_77_0,
            _1: s_77_1,
            _2: s_77_2,
            _3: s_77_3,
        };
        // D s_77_5: write-var ga#12684 <= s_77_4
        fn_state.ga_12684 = s_77_4;
        // D s_77_6: read-var ga#12684.0:struct
        let s_77_6: bool = fn_state.ga_12684._0;
        // D s_77_7: read-var ga#12684.1:struct
        let s_77_7: bool = fn_state.ga_12684._1;
        // D s_77_8: read-var ga#12684.2:struct
        let s_77_8: bool = fn_state.ga_12684._2;
        // D s_77_9: read-var ga#12684.3:struct
        let s_77_9: bool = fn_state.ga_12684._3;
        // D s_77_10: write-var ur <= s_77_6
        fn_state.ur = s_77_6;
        // D s_77_11: write-var uw <= s_77_7
        fn_state.uw = s_77_7;
        // D s_77_12: write-var ux <= s_77_8
        fn_state.ux = s_77_8;
        // D s_77_13: write-var ugcs <= s_77_9
        fn_state.ugcs = s_77_9;
        // N s_77_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_78_0: read-var ga#12679:u8
        let s_78_0: u8 = fn_state.ga_12679;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 4u16);
        // C s_78_2: const #5u : u8
        let s_78_2: u8 = 5;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 4u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: not s_78_4
        let s_78_5: bool = !s_78_4;
        // N s_78_6: branch s_78_5 b80 b79
        if s_78_5 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // C s_79_1: const #1u : u8
        let s_79_1: bool = true;
        // C s_79_2: const #0u : u8
        let s_79_2: bool = false;
        // C s_79_3: const #0u : u8
        let s_79_3: bool = false;
        // D s_79_4: create-product struct = ["s_79_0", "s_79_1", "s_79_2", "s_79_3"]
        let s_79_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_79_0,
            _1: s_79_1,
            _2: s_79_2,
            _3: s_79_3,
        };
        // D s_79_5: write-var ga#12685 <= s_79_4
        fn_state.ga_12685 = s_79_4;
        // D s_79_6: read-var ga#12685.0:struct
        let s_79_6: bool = fn_state.ga_12685._0;
        // D s_79_7: read-var ga#12685.1:struct
        let s_79_7: bool = fn_state.ga_12685._1;
        // D s_79_8: read-var ga#12685.2:struct
        let s_79_8: bool = fn_state.ga_12685._2;
        // D s_79_9: read-var ga#12685.3:struct
        let s_79_9: bool = fn_state.ga_12685._3;
        // D s_79_10: write-var ur <= s_79_6
        fn_state.ur = s_79_6;
        // D s_79_11: write-var uw <= s_79_7
        fn_state.uw = s_79_7;
        // D s_79_12: write-var ux <= s_79_8
        fn_state.ux = s_79_8;
        // D s_79_13: write-var ugcs <= s_79_9
        fn_state.ugcs = s_79_9;
        // N s_79_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_80_0: read-var ga#12679:u8
        let s_80_0: u8 = fn_state.ga_12679;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 4u16);
        // C s_80_2: const #6u : u8
        let s_80_2: u8 = 6;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 4u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: not s_80_4
        let s_80_5: bool = !s_80_4;
        // N s_80_6: branch s_80_5 b82 b81
        if s_80_5 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_81_0: const #1u : u8
        let s_81_0: bool = true;
        // C s_81_1: const #1u : u8
        let s_81_1: bool = true;
        // C s_81_2: const #1u : u8
        let s_81_2: bool = true;
        // C s_81_3: const #0u : u8
        let s_81_3: bool = false;
        // D s_81_4: create-product struct = ["s_81_0", "s_81_1", "s_81_2", "s_81_3"]
        let s_81_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_81_0,
            _1: s_81_1,
            _2: s_81_2,
            _3: s_81_3,
        };
        // D s_81_5: write-var ga#12686 <= s_81_4
        fn_state.ga_12686 = s_81_4;
        // D s_81_6: read-var ga#12686.0:struct
        let s_81_6: bool = fn_state.ga_12686._0;
        // D s_81_7: read-var ga#12686.1:struct
        let s_81_7: bool = fn_state.ga_12686._1;
        // D s_81_8: read-var ga#12686.2:struct
        let s_81_8: bool = fn_state.ga_12686._2;
        // D s_81_9: read-var ga#12686.3:struct
        let s_81_9: bool = fn_state.ga_12686._3;
        // D s_81_10: write-var ur <= s_81_6
        fn_state.ur = s_81_6;
        // D s_81_11: write-var uw <= s_81_7
        fn_state.uw = s_81_7;
        // D s_81_12: write-var ux <= s_81_8
        fn_state.ux = s_81_8;
        // D s_81_13: write-var ugcs <= s_81_9
        fn_state.ugcs = s_81_9;
        // N s_81_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_82_0: read-var ga#12679:u8
        let s_82_0: u8 = fn_state.ga_12679;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 4u16);
        // C s_82_2: const #7u : u8
        let s_82_2: u8 = 7;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 4u16);
        // D s_82_4: cmp-eq s_82_1 s_82_3
        let s_82_4: bool = ((s_82_1) == (s_82_3));
        // D s_82_5: not s_82_4
        let s_82_5: bool = !s_82_4;
        // N s_82_6: branch s_82_5 b84 b83
        if s_82_5 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_83_0: const #1u : u8
        let s_83_0: bool = true;
        // C s_83_1: const #1u : u8
        let s_83_1: bool = true;
        // C s_83_2: const #1u : u8
        let s_83_2: bool = true;
        // C s_83_3: const #0u : u8
        let s_83_3: bool = false;
        // D s_83_4: create-product struct = ["s_83_0", "s_83_1", "s_83_2", "s_83_3"]
        let s_83_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_83_0,
            _1: s_83_1,
            _2: s_83_2,
            _3: s_83_3,
        };
        // D s_83_5: write-var ga#12687 <= s_83_4
        fn_state.ga_12687 = s_83_4;
        // D s_83_6: read-var ga#12687.0:struct
        let s_83_6: bool = fn_state.ga_12687._0;
        // D s_83_7: read-var ga#12687.1:struct
        let s_83_7: bool = fn_state.ga_12687._1;
        // D s_83_8: read-var ga#12687.2:struct
        let s_83_8: bool = fn_state.ga_12687._2;
        // D s_83_9: read-var ga#12687.3:struct
        let s_83_9: bool = fn_state.ga_12687._3;
        // D s_83_10: write-var ur <= s_83_6
        fn_state.ur = s_83_6;
        // D s_83_11: write-var uw <= s_83_7
        fn_state.uw = s_83_7;
        // D s_83_12: write-var ux <= s_83_8
        fn_state.ux = s_83_8;
        // D s_83_13: write-var ugcs <= s_83_9
        fn_state.ugcs = s_83_9;
        // N s_83_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_84_0: read-var ga#12679:u8
        let s_84_0: u8 = fn_state.ga_12679;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 4u16);
        // C s_84_2: const #8u : u8
        let s_84_2: u8 = 8;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 4u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: not s_84_4
        let s_84_5: bool = !s_84_4;
        // N s_84_6: branch s_84_5 b86 b85
        if s_84_5 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_85_0: const #1u : u8
        let s_85_0: bool = true;
        // C s_85_1: const #0u : u8
        let s_85_1: bool = false;
        // C s_85_2: const #0u : u8
        let s_85_2: bool = false;
        // C s_85_3: const #0u : u8
        let s_85_3: bool = false;
        // D s_85_4: create-product struct = ["s_85_0", "s_85_1", "s_85_2", "s_85_3"]
        let s_85_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_85_0,
            _1: s_85_1,
            _2: s_85_2,
            _3: s_85_3,
        };
        // D s_85_5: write-var ga#12688 <= s_85_4
        fn_state.ga_12688 = s_85_4;
        // D s_85_6: read-var ga#12688.0:struct
        let s_85_6: bool = fn_state.ga_12688._0;
        // D s_85_7: read-var ga#12688.1:struct
        let s_85_7: bool = fn_state.ga_12688._1;
        // D s_85_8: read-var ga#12688.2:struct
        let s_85_8: bool = fn_state.ga_12688._2;
        // D s_85_9: read-var ga#12688.3:struct
        let s_85_9: bool = fn_state.ga_12688._3;
        // D s_85_10: write-var ur <= s_85_6
        fn_state.ur = s_85_6;
        // D s_85_11: write-var uw <= s_85_7
        fn_state.uw = s_85_7;
        // D s_85_12: write-var ux <= s_85_8
        fn_state.ux = s_85_8;
        // D s_85_13: write-var ugcs <= s_85_9
        fn_state.ugcs = s_85_9;
        // N s_85_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_86_0: read-var ga#12679:u8
        let s_86_0: u8 = fn_state.ga_12679;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 4u16);
        // C s_86_2: const #9u : u8
        let s_86_2: u8 = 9;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 4u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: not s_86_4
        let s_86_5: bool = !s_86_4;
        // N s_86_6: branch s_86_5 b88 b87
        if s_86_5 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_87_0: const #1u : u8
        let s_87_0: bool = true;
        // C s_87_1: const #0u : u8
        let s_87_1: bool = false;
        // C s_87_2: const #0u : u8
        let s_87_2: bool = false;
        // C s_87_3: const #1u : u8
        let s_87_3: bool = true;
        // D s_87_4: create-product struct = ["s_87_0", "s_87_1", "s_87_2", "s_87_3"]
        let s_87_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_87_0,
            _1: s_87_1,
            _2: s_87_2,
            _3: s_87_3,
        };
        // D s_87_5: write-var ga#12689 <= s_87_4
        fn_state.ga_12689 = s_87_4;
        // D s_87_6: read-var ga#12689.0:struct
        let s_87_6: bool = fn_state.ga_12689._0;
        // D s_87_7: read-var ga#12689.1:struct
        let s_87_7: bool = fn_state.ga_12689._1;
        // D s_87_8: read-var ga#12689.2:struct
        let s_87_8: bool = fn_state.ga_12689._2;
        // D s_87_9: read-var ga#12689.3:struct
        let s_87_9: bool = fn_state.ga_12689._3;
        // D s_87_10: write-var ur <= s_87_6
        fn_state.ur = s_87_6;
        // D s_87_11: write-var uw <= s_87_7
        fn_state.uw = s_87_7;
        // D s_87_12: write-var ux <= s_87_8
        fn_state.ux = s_87_8;
        // D s_87_13: write-var ugcs <= s_87_9
        fn_state.ugcs = s_87_9;
        // N s_87_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_88_0: read-var ga#12679:u8
        let s_88_0: u8 = fn_state.ga_12679;
        // D s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 4u16);
        // C s_88_2: const #10u : u8
        let s_88_2: u8 = 10;
        // C s_88_3: cast zx s_88_2 -> bv
        let s_88_3: Bits = Bits::new(s_88_2 as u128, 4u16);
        // D s_88_4: cmp-eq s_88_1 s_88_3
        let s_88_4: bool = ((s_88_1) == (s_88_3));
        // D s_88_5: not s_88_4
        let s_88_5: bool = !s_88_4;
        // N s_88_6: branch s_88_5 b90 b89
        if s_88_5 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_89_0: const #1u : u8
        let s_89_0: bool = true;
        // C s_89_1: const #0u : u8
        let s_89_1: bool = false;
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // C s_89_3: const #0u : u8
        let s_89_3: bool = false;
        // D s_89_4: create-product struct = ["s_89_0", "s_89_1", "s_89_2", "s_89_3"]
        let s_89_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_89_0,
            _1: s_89_1,
            _2: s_89_2,
            _3: s_89_3,
        };
        // D s_89_5: write-var ga#12690 <= s_89_4
        fn_state.ga_12690 = s_89_4;
        // D s_89_6: read-var ga#12690.0:struct
        let s_89_6: bool = fn_state.ga_12690._0;
        // D s_89_7: read-var ga#12690.1:struct
        let s_89_7: bool = fn_state.ga_12690._1;
        // D s_89_8: read-var ga#12690.2:struct
        let s_89_8: bool = fn_state.ga_12690._2;
        // D s_89_9: read-var ga#12690.3:struct
        let s_89_9: bool = fn_state.ga_12690._3;
        // D s_89_10: write-var ur <= s_89_6
        fn_state.ur = s_89_6;
        // D s_89_11: write-var uw <= s_89_7
        fn_state.uw = s_89_7;
        // D s_89_12: write-var ux <= s_89_8
        fn_state.ux = s_89_8;
        // D s_89_13: write-var ugcs <= s_89_9
        fn_state.ugcs = s_89_9;
        // N s_89_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_90_0: read-var ga#12679:u8
        let s_90_0: u8 = fn_state.ga_12679;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 4u16);
        // C s_90_2: const #11u : u8
        let s_90_2: u8 = 11;
        // C s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 4u16);
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: not s_90_4
        let s_90_5: bool = !s_90_4;
        // N s_90_6: branch s_90_5 b92 b91
        if s_90_5 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_91_0: const #0u : u8
        let s_91_0: bool = false;
        // C s_91_1: const #0u : u8
        let s_91_1: bool = false;
        // C s_91_2: const #0u : u8
        let s_91_2: bool = false;
        // C s_91_3: const #0u : u8
        let s_91_3: bool = false;
        // D s_91_4: create-product struct = ["s_91_0", "s_91_1", "s_91_2", "s_91_3"]
        let s_91_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_91_0,
            _1: s_91_1,
            _2: s_91_2,
            _3: s_91_3,
        };
        // D s_91_5: write-var ga#12691 <= s_91_4
        fn_state.ga_12691 = s_91_4;
        // D s_91_6: read-var ga#12691.0:struct
        let s_91_6: bool = fn_state.ga_12691._0;
        // D s_91_7: read-var ga#12691.1:struct
        let s_91_7: bool = fn_state.ga_12691._1;
        // D s_91_8: read-var ga#12691.2:struct
        let s_91_8: bool = fn_state.ga_12691._2;
        // D s_91_9: read-var ga#12691.3:struct
        let s_91_9: bool = fn_state.ga_12691._3;
        // D s_91_10: write-var ur <= s_91_6
        fn_state.ur = s_91_6;
        // D s_91_11: write-var uw <= s_91_7
        fn_state.uw = s_91_7;
        // D s_91_12: write-var ux <= s_91_8
        fn_state.ux = s_91_8;
        // D s_91_13: write-var ugcs <= s_91_9
        fn_state.ugcs = s_91_9;
        // N s_91_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_92_0: read-var ga#12679:u8
        let s_92_0: u8 = fn_state.ga_12679;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 4u16);
        // C s_92_2: const #12u : u8
        let s_92_2: u8 = 12;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 4u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: not s_92_4
        let s_92_5: bool = !s_92_4;
        // N s_92_6: branch s_92_5 b94 b93
        if s_92_5 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // C s_93_1: const #1u : u8
        let s_93_1: bool = true;
        // C s_93_2: const #0u : u8
        let s_93_2: bool = false;
        // C s_93_3: const #0u : u8
        let s_93_3: bool = false;
        // D s_93_4: create-product struct = ["s_93_0", "s_93_1", "s_93_2", "s_93_3"]
        let s_93_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_93_0,
            _1: s_93_1,
            _2: s_93_2,
            _3: s_93_3,
        };
        // D s_93_5: write-var ga#12692 <= s_93_4
        fn_state.ga_12692 = s_93_4;
        // D s_93_6: read-var ga#12692.0:struct
        let s_93_6: bool = fn_state.ga_12692._0;
        // D s_93_7: read-var ga#12692.1:struct
        let s_93_7: bool = fn_state.ga_12692._1;
        // D s_93_8: read-var ga#12692.2:struct
        let s_93_8: bool = fn_state.ga_12692._2;
        // D s_93_9: read-var ga#12692.3:struct
        let s_93_9: bool = fn_state.ga_12692._3;
        // D s_93_10: write-var ur <= s_93_6
        fn_state.ur = s_93_6;
        // D s_93_11: write-var uw <= s_93_7
        fn_state.uw = s_93_7;
        // D s_93_12: write-var ux <= s_93_8
        fn_state.ux = s_93_8;
        // D s_93_13: write-var ugcs <= s_93_9
        fn_state.ugcs = s_93_9;
        // N s_93_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_94_0: read-var ga#12679:u8
        let s_94_0: u8 = fn_state.ga_12679;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 4u16);
        // C s_94_2: const #13u : u8
        let s_94_2: u8 = 13;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 4u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // D s_94_5: not s_94_4
        let s_94_5: bool = !s_94_4;
        // N s_94_6: branch s_94_5 b96 b95
        if s_94_5 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // C s_95_1: const #0u : u8
        let s_95_1: bool = false;
        // C s_95_2: const #0u : u8
        let s_95_2: bool = false;
        // C s_95_3: const #0u : u8
        let s_95_3: bool = false;
        // D s_95_4: create-product struct = ["s_95_0", "s_95_1", "s_95_2", "s_95_3"]
        let s_95_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_95_0,
            _1: s_95_1,
            _2: s_95_2,
            _3: s_95_3,
        };
        // D s_95_5: write-var ga#12693 <= s_95_4
        fn_state.ga_12693 = s_95_4;
        // D s_95_6: read-var ga#12693.0:struct
        let s_95_6: bool = fn_state.ga_12693._0;
        // D s_95_7: read-var ga#12693.1:struct
        let s_95_7: bool = fn_state.ga_12693._1;
        // D s_95_8: read-var ga#12693.2:struct
        let s_95_8: bool = fn_state.ga_12693._2;
        // D s_95_9: read-var ga#12693.3:struct
        let s_95_9: bool = fn_state.ga_12693._3;
        // D s_95_10: write-var ur <= s_95_6
        fn_state.ur = s_95_6;
        // D s_95_11: write-var uw <= s_95_7
        fn_state.uw = s_95_7;
        // D s_95_12: write-var ux <= s_95_8
        fn_state.ux = s_95_8;
        // D s_95_13: write-var ugcs <= s_95_9
        fn_state.ugcs = s_95_9;
        // N s_95_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_96_0: read-var ga#12679:u8
        let s_96_0: u8 = fn_state.ga_12679;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 4u16);
        // C s_96_2: const #14u : u8
        let s_96_2: u8 = 14;
        // C s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 4u16);
        // D s_96_4: cmp-eq s_96_1 s_96_3
        let s_96_4: bool = ((s_96_1) == (s_96_3));
        // D s_96_5: not s_96_4
        let s_96_5: bool = !s_96_4;
        // N s_96_6: branch s_96_5 b98 b97
        if s_96_5 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_97_0: const #1u : u8
        let s_97_0: bool = true;
        // C s_97_1: const #1u : u8
        let s_97_1: bool = true;
        // C s_97_2: const #1u : u8
        let s_97_2: bool = true;
        // C s_97_3: const #0u : u8
        let s_97_3: bool = false;
        // D s_97_4: create-product struct = ["s_97_0", "s_97_1", "s_97_2", "s_97_3"]
        let s_97_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_97_0,
            _1: s_97_1,
            _2: s_97_2,
            _3: s_97_3,
        };
        // D s_97_5: write-var ga#12694 <= s_97_4
        fn_state.ga_12694 = s_97_4;
        // D s_97_6: read-var ga#12694.0:struct
        let s_97_6: bool = fn_state.ga_12694._0;
        // D s_97_7: read-var ga#12694.1:struct
        let s_97_7: bool = fn_state.ga_12694._1;
        // D s_97_8: read-var ga#12694.2:struct
        let s_97_8: bool = fn_state.ga_12694._2;
        // D s_97_9: read-var ga#12694.3:struct
        let s_97_9: bool = fn_state.ga_12694._3;
        // D s_97_10: write-var ur <= s_97_6
        fn_state.ur = s_97_6;
        // D s_97_11: write-var uw <= s_97_7
        fn_state.uw = s_97_7;
        // D s_97_12: write-var ux <= s_97_8
        fn_state.ux = s_97_8;
        // D s_97_13: write-var ugcs <= s_97_9
        fn_state.ugcs = s_97_9;
        // N s_97_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // C s_98_1: const #0u : u8
        let s_98_1: bool = false;
        // C s_98_2: const #0u : u8
        let s_98_2: bool = false;
        // C s_98_3: const #0u : u8
        let s_98_3: bool = false;
        // D s_98_4: create-product struct = ["s_98_0", "s_98_1", "s_98_2", "s_98_3"]
        let s_98_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_98_0,
            _1: s_98_1,
            _2: s_98_2,
            _3: s_98_3,
        };
        // D s_98_5: write-var ga#12695 <= s_98_4
        fn_state.ga_12695 = s_98_4;
        // D s_98_6: read-var ga#12695.0:struct
        let s_98_6: bool = fn_state.ga_12695._0;
        // D s_98_7: read-var ga#12695.1:struct
        let s_98_7: bool = fn_state.ga_12695._1;
        // D s_98_8: read-var ga#12695.2:struct
        let s_98_8: bool = fn_state.ga_12695._2;
        // D s_98_9: read-var ga#12695.3:struct
        let s_98_9: bool = fn_state.ga_12695._3;
        // D s_98_10: write-var ur <= s_98_6
        fn_state.ur = s_98_6;
        // D s_98_11: write-var uw <= s_98_7
        fn_state.uw = s_98_7;
        // D s_98_12: write-var ux <= s_98_8
        fn_state.ux = s_98_8;
        // D s_98_13: write-var ugcs <= s_98_9
        fn_state.ugcs = s_98_9;
        // N s_98_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_99_0: const #1u : u8
        let s_99_0: bool = true;
        // D s_99_1: write-var pwxn <= s_99_0
        fn_state.pwxn = s_99_0;
        // N s_99_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_100_0: read-var ga#12661:u8
        let s_100_0: u8 = fn_state.ga_12661;
        // D s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 4u16);
        // C s_100_2: const #1u : u8
        let s_100_2: u8 = 1;
        // C s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 4u16);
        // D s_100_4: cmp-eq s_100_1 s_100_3
        let s_100_4: bool = ((s_100_1) == (s_100_3));
        // D s_100_5: not s_100_4
        let s_100_5: bool = !s_100_4;
        // N s_100_6: branch s_100_5 b102 b101
        if s_100_5 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_101_0: const #1u : u8
        let s_101_0: bool = true;
        // C s_101_1: const #0u : u8
        let s_101_1: bool = false;
        // C s_101_2: const #0u : u8
        let s_101_2: bool = false;
        // C s_101_3: const #0u : u8
        let s_101_3: bool = false;
        // D s_101_4: create-product struct = ["s_101_0", "s_101_1", "s_101_2", "s_101_3"]
        let s_101_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_101_0,
            _1: s_101_1,
            _2: s_101_2,
            _3: s_101_3,
        };
        // D s_101_5: write-var ga#12663 <= s_101_4
        fn_state.ga_12663 = s_101_4;
        // D s_101_6: read-var ga#12663.0:struct
        let s_101_6: bool = fn_state.ga_12663._0;
        // D s_101_7: read-var ga#12663.1:struct
        let s_101_7: bool = fn_state.ga_12663._1;
        // D s_101_8: read-var ga#12663.2:struct
        let s_101_8: bool = fn_state.ga_12663._2;
        // D s_101_9: read-var ga#12663.3:struct
        let s_101_9: bool = fn_state.ga_12663._3;
        // D s_101_10: write-var pr <= s_101_6
        fn_state.pr = s_101_6;
        // D s_101_11: write-var pw <= s_101_7
        fn_state.pw = s_101_7;
        // D s_101_12: write-var px <= s_101_8
        fn_state.px = s_101_8;
        // D s_101_13: write-var pgcs <= s_101_9
        fn_state.pgcs = s_101_9;
        // N s_101_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_102_0: read-var ga#12661:u8
        let s_102_0: u8 = fn_state.ga_12661;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 4u16);
        // C s_102_2: const #2u : u8
        let s_102_2: u8 = 2;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 4u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: not s_102_4
        let s_102_5: bool = !s_102_4;
        // N s_102_6: branch s_102_5 b104 b103
        if s_102_5 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // C s_103_1: const #0u : u8
        let s_103_1: bool = false;
        // C s_103_2: const #1u : u8
        let s_103_2: bool = true;
        // C s_103_3: const #0u : u8
        let s_103_3: bool = false;
        // D s_103_4: create-product struct = ["s_103_0", "s_103_1", "s_103_2", "s_103_3"]
        let s_103_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_103_0,
            _1: s_103_1,
            _2: s_103_2,
            _3: s_103_3,
        };
        // D s_103_5: write-var ga#12664 <= s_103_4
        fn_state.ga_12664 = s_103_4;
        // D s_103_6: read-var ga#12664.0:struct
        let s_103_6: bool = fn_state.ga_12664._0;
        // D s_103_7: read-var ga#12664.1:struct
        let s_103_7: bool = fn_state.ga_12664._1;
        // D s_103_8: read-var ga#12664.2:struct
        let s_103_8: bool = fn_state.ga_12664._2;
        // D s_103_9: read-var ga#12664.3:struct
        let s_103_9: bool = fn_state.ga_12664._3;
        // D s_103_10: write-var pr <= s_103_6
        fn_state.pr = s_103_6;
        // D s_103_11: write-var pw <= s_103_7
        fn_state.pw = s_103_7;
        // D s_103_12: write-var px <= s_103_8
        fn_state.px = s_103_8;
        // D s_103_13: write-var pgcs <= s_103_9
        fn_state.pgcs = s_103_9;
        // N s_103_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_104_0: read-var ga#12661:u8
        let s_104_0: u8 = fn_state.ga_12661;
        // D s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 4u16);
        // C s_104_2: const #3u : u8
        let s_104_2: u8 = 3;
        // C s_104_3: cast zx s_104_2 -> bv
        let s_104_3: Bits = Bits::new(s_104_2 as u128, 4u16);
        // D s_104_4: cmp-eq s_104_1 s_104_3
        let s_104_4: bool = ((s_104_1) == (s_104_3));
        // D s_104_5: not s_104_4
        let s_104_5: bool = !s_104_4;
        // N s_104_6: branch s_104_5 b106 b105
        if s_104_5 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_105_0: const #1u : u8
        let s_105_0: bool = true;
        // C s_105_1: const #0u : u8
        let s_105_1: bool = false;
        // C s_105_2: const #1u : u8
        let s_105_2: bool = true;
        // C s_105_3: const #0u : u8
        let s_105_3: bool = false;
        // D s_105_4: create-product struct = ["s_105_0", "s_105_1", "s_105_2", "s_105_3"]
        let s_105_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_105_0,
            _1: s_105_1,
            _2: s_105_2,
            _3: s_105_3,
        };
        // D s_105_5: write-var ga#12665 <= s_105_4
        fn_state.ga_12665 = s_105_4;
        // D s_105_6: read-var ga#12665.0:struct
        let s_105_6: bool = fn_state.ga_12665._0;
        // D s_105_7: read-var ga#12665.1:struct
        let s_105_7: bool = fn_state.ga_12665._1;
        // D s_105_8: read-var ga#12665.2:struct
        let s_105_8: bool = fn_state.ga_12665._2;
        // D s_105_9: read-var ga#12665.3:struct
        let s_105_9: bool = fn_state.ga_12665._3;
        // D s_105_10: write-var pr <= s_105_6
        fn_state.pr = s_105_6;
        // D s_105_11: write-var pw <= s_105_7
        fn_state.pw = s_105_7;
        // D s_105_12: write-var px <= s_105_8
        fn_state.px = s_105_8;
        // D s_105_13: write-var pgcs <= s_105_9
        fn_state.pgcs = s_105_9;
        // N s_105_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_106_0: read-var ga#12661:u8
        let s_106_0: u8 = fn_state.ga_12661;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 4u16);
        // C s_106_2: const #4u : u8
        let s_106_2: u8 = 4;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 4u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: not s_106_4
        let s_106_5: bool = !s_106_4;
        // N s_106_6: branch s_106_5 b108 b107
        if s_106_5 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // C s_107_1: const #0u : u8
        let s_107_1: bool = false;
        // C s_107_2: const #0u : u8
        let s_107_2: bool = false;
        // C s_107_3: const #0u : u8
        let s_107_3: bool = false;
        // D s_107_4: create-product struct = ["s_107_0", "s_107_1", "s_107_2", "s_107_3"]
        let s_107_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_107_0,
            _1: s_107_1,
            _2: s_107_2,
            _3: s_107_3,
        };
        // D s_107_5: write-var ga#12666 <= s_107_4
        fn_state.ga_12666 = s_107_4;
        // D s_107_6: read-var ga#12666.0:struct
        let s_107_6: bool = fn_state.ga_12666._0;
        // D s_107_7: read-var ga#12666.1:struct
        let s_107_7: bool = fn_state.ga_12666._1;
        // D s_107_8: read-var ga#12666.2:struct
        let s_107_8: bool = fn_state.ga_12666._2;
        // D s_107_9: read-var ga#12666.3:struct
        let s_107_9: bool = fn_state.ga_12666._3;
        // D s_107_10: write-var pr <= s_107_6
        fn_state.pr = s_107_6;
        // D s_107_11: write-var pw <= s_107_7
        fn_state.pw = s_107_7;
        // D s_107_12: write-var px <= s_107_8
        fn_state.px = s_107_8;
        // D s_107_13: write-var pgcs <= s_107_9
        fn_state.pgcs = s_107_9;
        // N s_107_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_108_0: read-var ga#12661:u8
        let s_108_0: u8 = fn_state.ga_12661;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 4u16);
        // C s_108_2: const #5u : u8
        let s_108_2: u8 = 5;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 4u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: not s_108_4
        let s_108_5: bool = !s_108_4;
        // N s_108_6: branch s_108_5 b110 b109
        if s_108_5 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_109_0: const #1u : u8
        let s_109_0: bool = true;
        // C s_109_1: const #1u : u8
        let s_109_1: bool = true;
        // C s_109_2: const #0u : u8
        let s_109_2: bool = false;
        // C s_109_3: const #0u : u8
        let s_109_3: bool = false;
        // D s_109_4: create-product struct = ["s_109_0", "s_109_1", "s_109_2", "s_109_3"]
        let s_109_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_109_0,
            _1: s_109_1,
            _2: s_109_2,
            _3: s_109_3,
        };
        // D s_109_5: write-var ga#12667 <= s_109_4
        fn_state.ga_12667 = s_109_4;
        // D s_109_6: read-var ga#12667.0:struct
        let s_109_6: bool = fn_state.ga_12667._0;
        // D s_109_7: read-var ga#12667.1:struct
        let s_109_7: bool = fn_state.ga_12667._1;
        // D s_109_8: read-var ga#12667.2:struct
        let s_109_8: bool = fn_state.ga_12667._2;
        // D s_109_9: read-var ga#12667.3:struct
        let s_109_9: bool = fn_state.ga_12667._3;
        // D s_109_10: write-var pr <= s_109_6
        fn_state.pr = s_109_6;
        // D s_109_11: write-var pw <= s_109_7
        fn_state.pw = s_109_7;
        // D s_109_12: write-var px <= s_109_8
        fn_state.px = s_109_8;
        // D s_109_13: write-var pgcs <= s_109_9
        fn_state.pgcs = s_109_9;
        // N s_109_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_110_0: read-var ga#12661:u8
        let s_110_0: u8 = fn_state.ga_12661;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 4u16);
        // C s_110_2: const #6u : u8
        let s_110_2: u8 = 6;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 4u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // D s_110_5: not s_110_4
        let s_110_5: bool = !s_110_4;
        // N s_110_6: branch s_110_5 b112 b111
        if s_110_5 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_111_0: const #1u : u8
        let s_111_0: bool = true;
        // C s_111_1: const #1u : u8
        let s_111_1: bool = true;
        // C s_111_2: const #1u : u8
        let s_111_2: bool = true;
        // C s_111_3: const #0u : u8
        let s_111_3: bool = false;
        // D s_111_4: create-product struct = ["s_111_0", "s_111_1", "s_111_2", "s_111_3"]
        let s_111_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_111_0,
            _1: s_111_1,
            _2: s_111_2,
            _3: s_111_3,
        };
        // D s_111_5: write-var ga#12668 <= s_111_4
        fn_state.ga_12668 = s_111_4;
        // D s_111_6: read-var ga#12668.0:struct
        let s_111_6: bool = fn_state.ga_12668._0;
        // D s_111_7: read-var ga#12668.1:struct
        let s_111_7: bool = fn_state.ga_12668._1;
        // D s_111_8: read-var ga#12668.2:struct
        let s_111_8: bool = fn_state.ga_12668._2;
        // D s_111_9: read-var ga#12668.3:struct
        let s_111_9: bool = fn_state.ga_12668._3;
        // D s_111_10: write-var pr <= s_111_6
        fn_state.pr = s_111_6;
        // D s_111_11: write-var pw <= s_111_7
        fn_state.pw = s_111_7;
        // D s_111_12: write-var px <= s_111_8
        fn_state.px = s_111_8;
        // D s_111_13: write-var pgcs <= s_111_9
        fn_state.pgcs = s_111_9;
        // N s_111_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_112_0: read-var ga#12661:u8
        let s_112_0: u8 = fn_state.ga_12661;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 4u16);
        // C s_112_2: const #7u : u8
        let s_112_2: u8 = 7;
        // C s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 4u16);
        // D s_112_4: cmp-eq s_112_1 s_112_3
        let s_112_4: bool = ((s_112_1) == (s_112_3));
        // D s_112_5: not s_112_4
        let s_112_5: bool = !s_112_4;
        // N s_112_6: branch s_112_5 b114 b113
        if s_112_5 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_113_0: const #1u : u8
        let s_113_0: bool = true;
        // C s_113_1: const #1u : u8
        let s_113_1: bool = true;
        // C s_113_2: const #1u : u8
        let s_113_2: bool = true;
        // C s_113_3: const #0u : u8
        let s_113_3: bool = false;
        // D s_113_4: create-product struct = ["s_113_0", "s_113_1", "s_113_2", "s_113_3"]
        let s_113_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_113_0,
            _1: s_113_1,
            _2: s_113_2,
            _3: s_113_3,
        };
        // D s_113_5: write-var ga#12669 <= s_113_4
        fn_state.ga_12669 = s_113_4;
        // D s_113_6: read-var ga#12669.0:struct
        let s_113_6: bool = fn_state.ga_12669._0;
        // D s_113_7: read-var ga#12669.1:struct
        let s_113_7: bool = fn_state.ga_12669._1;
        // D s_113_8: read-var ga#12669.2:struct
        let s_113_8: bool = fn_state.ga_12669._2;
        // D s_113_9: read-var ga#12669.3:struct
        let s_113_9: bool = fn_state.ga_12669._3;
        // D s_113_10: write-var pr <= s_113_6
        fn_state.pr = s_113_6;
        // D s_113_11: write-var pw <= s_113_7
        fn_state.pw = s_113_7;
        // D s_113_12: write-var px <= s_113_8
        fn_state.px = s_113_8;
        // D s_113_13: write-var pgcs <= s_113_9
        fn_state.pgcs = s_113_9;
        // N s_113_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_114_0: read-var ga#12661:u8
        let s_114_0: u8 = fn_state.ga_12661;
        // D s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 4u16);
        // C s_114_2: const #8u : u8
        let s_114_2: u8 = 8;
        // C s_114_3: cast zx s_114_2 -> bv
        let s_114_3: Bits = Bits::new(s_114_2 as u128, 4u16);
        // D s_114_4: cmp-eq s_114_1 s_114_3
        let s_114_4: bool = ((s_114_1) == (s_114_3));
        // D s_114_5: not s_114_4
        let s_114_5: bool = !s_114_4;
        // N s_114_6: branch s_114_5 b116 b115
        if s_114_5 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_115_0: const #1u : u8
        let s_115_0: bool = true;
        // C s_115_1: const #0u : u8
        let s_115_1: bool = false;
        // C s_115_2: const #0u : u8
        let s_115_2: bool = false;
        // C s_115_3: const #0u : u8
        let s_115_3: bool = false;
        // D s_115_4: create-product struct = ["s_115_0", "s_115_1", "s_115_2", "s_115_3"]
        let s_115_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_115_0,
            _1: s_115_1,
            _2: s_115_2,
            _3: s_115_3,
        };
        // D s_115_5: write-var ga#12670 <= s_115_4
        fn_state.ga_12670 = s_115_4;
        // D s_115_6: read-var ga#12670.0:struct
        let s_115_6: bool = fn_state.ga_12670._0;
        // D s_115_7: read-var ga#12670.1:struct
        let s_115_7: bool = fn_state.ga_12670._1;
        // D s_115_8: read-var ga#12670.2:struct
        let s_115_8: bool = fn_state.ga_12670._2;
        // D s_115_9: read-var ga#12670.3:struct
        let s_115_9: bool = fn_state.ga_12670._3;
        // D s_115_10: write-var pr <= s_115_6
        fn_state.pr = s_115_6;
        // D s_115_11: write-var pw <= s_115_7
        fn_state.pw = s_115_7;
        // D s_115_12: write-var px <= s_115_8
        fn_state.px = s_115_8;
        // D s_115_13: write-var pgcs <= s_115_9
        fn_state.pgcs = s_115_9;
        // N s_115_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_116_0: read-var ga#12661:u8
        let s_116_0: u8 = fn_state.ga_12661;
        // D s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 4u16);
        // C s_116_2: const #9u : u8
        let s_116_2: u8 = 9;
        // C s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 4u16);
        // D s_116_4: cmp-eq s_116_1 s_116_3
        let s_116_4: bool = ((s_116_1) == (s_116_3));
        // D s_116_5: not s_116_4
        let s_116_5: bool = !s_116_4;
        // N s_116_6: branch s_116_5 b118 b117
        if s_116_5 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_117_0: const #1u : u8
        let s_117_0: bool = true;
        // C s_117_1: const #0u : u8
        let s_117_1: bool = false;
        // C s_117_2: const #0u : u8
        let s_117_2: bool = false;
        // C s_117_3: const #1u : u8
        let s_117_3: bool = true;
        // D s_117_4: create-product struct = ["s_117_0", "s_117_1", "s_117_2", "s_117_3"]
        let s_117_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_117_0,
            _1: s_117_1,
            _2: s_117_2,
            _3: s_117_3,
        };
        // D s_117_5: write-var ga#12671 <= s_117_4
        fn_state.ga_12671 = s_117_4;
        // D s_117_6: read-var ga#12671.0:struct
        let s_117_6: bool = fn_state.ga_12671._0;
        // D s_117_7: read-var ga#12671.1:struct
        let s_117_7: bool = fn_state.ga_12671._1;
        // D s_117_8: read-var ga#12671.2:struct
        let s_117_8: bool = fn_state.ga_12671._2;
        // D s_117_9: read-var ga#12671.3:struct
        let s_117_9: bool = fn_state.ga_12671._3;
        // D s_117_10: write-var pr <= s_117_6
        fn_state.pr = s_117_6;
        // D s_117_11: write-var pw <= s_117_7
        fn_state.pw = s_117_7;
        // D s_117_12: write-var px <= s_117_8
        fn_state.px = s_117_8;
        // D s_117_13: write-var pgcs <= s_117_9
        fn_state.pgcs = s_117_9;
        // N s_117_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_118_0: read-var ga#12661:u8
        let s_118_0: u8 = fn_state.ga_12661;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 4u16);
        // C s_118_2: const #10u : u8
        let s_118_2: u8 = 10;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 4u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // D s_118_5: not s_118_4
        let s_118_5: bool = !s_118_4;
        // N s_118_6: branch s_118_5 b120 b119
        if s_118_5 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_119_0: const #1u : u8
        let s_119_0: bool = true;
        // C s_119_1: const #0u : u8
        let s_119_1: bool = false;
        // C s_119_2: const #1u : u8
        let s_119_2: bool = true;
        // C s_119_3: const #0u : u8
        let s_119_3: bool = false;
        // D s_119_4: create-product struct = ["s_119_0", "s_119_1", "s_119_2", "s_119_3"]
        let s_119_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_119_0,
            _1: s_119_1,
            _2: s_119_2,
            _3: s_119_3,
        };
        // D s_119_5: write-var ga#12672 <= s_119_4
        fn_state.ga_12672 = s_119_4;
        // D s_119_6: read-var ga#12672.0:struct
        let s_119_6: bool = fn_state.ga_12672._0;
        // D s_119_7: read-var ga#12672.1:struct
        let s_119_7: bool = fn_state.ga_12672._1;
        // D s_119_8: read-var ga#12672.2:struct
        let s_119_8: bool = fn_state.ga_12672._2;
        // D s_119_9: read-var ga#12672.3:struct
        let s_119_9: bool = fn_state.ga_12672._3;
        // D s_119_10: write-var pr <= s_119_6
        fn_state.pr = s_119_6;
        // D s_119_11: write-var pw <= s_119_7
        fn_state.pw = s_119_7;
        // D s_119_12: write-var px <= s_119_8
        fn_state.px = s_119_8;
        // D s_119_13: write-var pgcs <= s_119_9
        fn_state.pgcs = s_119_9;
        // N s_119_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_120_0: read-var ga#12661:u8
        let s_120_0: u8 = fn_state.ga_12661;
        // D s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 4u16);
        // C s_120_2: const #11u : u8
        let s_120_2: u8 = 11;
        // C s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 4u16);
        // D s_120_4: cmp-eq s_120_1 s_120_3
        let s_120_4: bool = ((s_120_1) == (s_120_3));
        // D s_120_5: not s_120_4
        let s_120_5: bool = !s_120_4;
        // N s_120_6: branch s_120_5 b122 b121
        if s_120_5 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // C s_121_1: const #0u : u8
        let s_121_1: bool = false;
        // C s_121_2: const #0u : u8
        let s_121_2: bool = false;
        // C s_121_3: const #0u : u8
        let s_121_3: bool = false;
        // D s_121_4: create-product struct = ["s_121_0", "s_121_1", "s_121_2", "s_121_3"]
        let s_121_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_121_0,
            _1: s_121_1,
            _2: s_121_2,
            _3: s_121_3,
        };
        // D s_121_5: write-var ga#12673 <= s_121_4
        fn_state.ga_12673 = s_121_4;
        // D s_121_6: read-var ga#12673.0:struct
        let s_121_6: bool = fn_state.ga_12673._0;
        // D s_121_7: read-var ga#12673.1:struct
        let s_121_7: bool = fn_state.ga_12673._1;
        // D s_121_8: read-var ga#12673.2:struct
        let s_121_8: bool = fn_state.ga_12673._2;
        // D s_121_9: read-var ga#12673.3:struct
        let s_121_9: bool = fn_state.ga_12673._3;
        // D s_121_10: write-var pr <= s_121_6
        fn_state.pr = s_121_6;
        // D s_121_11: write-var pw <= s_121_7
        fn_state.pw = s_121_7;
        // D s_121_12: write-var px <= s_121_8
        fn_state.px = s_121_8;
        // D s_121_13: write-var pgcs <= s_121_9
        fn_state.pgcs = s_121_9;
        // N s_121_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_122_0: read-var ga#12661:u8
        let s_122_0: u8 = fn_state.ga_12661;
        // D s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 4u16);
        // C s_122_2: const #12u : u8
        let s_122_2: u8 = 12;
        // C s_122_3: cast zx s_122_2 -> bv
        let s_122_3: Bits = Bits::new(s_122_2 as u128, 4u16);
        // D s_122_4: cmp-eq s_122_1 s_122_3
        let s_122_4: bool = ((s_122_1) == (s_122_3));
        // D s_122_5: not s_122_4
        let s_122_5: bool = !s_122_4;
        // N s_122_6: branch s_122_5 b124 b123
        if s_122_5 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_123_0: const #1u : u8
        let s_123_0: bool = true;
        // C s_123_1: const #1u : u8
        let s_123_1: bool = true;
        // C s_123_2: const #0u : u8
        let s_123_2: bool = false;
        // C s_123_3: const #0u : u8
        let s_123_3: bool = false;
        // D s_123_4: create-product struct = ["s_123_0", "s_123_1", "s_123_2", "s_123_3"]
        let s_123_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_123_0,
            _1: s_123_1,
            _2: s_123_2,
            _3: s_123_3,
        };
        // D s_123_5: write-var ga#12674 <= s_123_4
        fn_state.ga_12674 = s_123_4;
        // D s_123_6: read-var ga#12674.0:struct
        let s_123_6: bool = fn_state.ga_12674._0;
        // D s_123_7: read-var ga#12674.1:struct
        let s_123_7: bool = fn_state.ga_12674._1;
        // D s_123_8: read-var ga#12674.2:struct
        let s_123_8: bool = fn_state.ga_12674._2;
        // D s_123_9: read-var ga#12674.3:struct
        let s_123_9: bool = fn_state.ga_12674._3;
        // D s_123_10: write-var pr <= s_123_6
        fn_state.pr = s_123_6;
        // D s_123_11: write-var pw <= s_123_7
        fn_state.pw = s_123_7;
        // D s_123_12: write-var px <= s_123_8
        fn_state.px = s_123_8;
        // D s_123_13: write-var pgcs <= s_123_9
        fn_state.pgcs = s_123_9;
        // N s_123_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_124_0: read-var ga#12661:u8
        let s_124_0: u8 = fn_state.ga_12661;
        // D s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 4u16);
        // C s_124_2: const #13u : u8
        let s_124_2: u8 = 13;
        // C s_124_3: cast zx s_124_2 -> bv
        let s_124_3: Bits = Bits::new(s_124_2 as u128, 4u16);
        // D s_124_4: cmp-eq s_124_1 s_124_3
        let s_124_4: bool = ((s_124_1) == (s_124_3));
        // D s_124_5: not s_124_4
        let s_124_5: bool = !s_124_4;
        // N s_124_6: branch s_124_5 b126 b125
        if s_124_5 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_125_0: const #0u : u8
        let s_125_0: bool = false;
        // C s_125_1: const #0u : u8
        let s_125_1: bool = false;
        // C s_125_2: const #0u : u8
        let s_125_2: bool = false;
        // C s_125_3: const #0u : u8
        let s_125_3: bool = false;
        // D s_125_4: create-product struct = ["s_125_0", "s_125_1", "s_125_2", "s_125_3"]
        let s_125_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_125_0,
            _1: s_125_1,
            _2: s_125_2,
            _3: s_125_3,
        };
        // D s_125_5: write-var ga#12675 <= s_125_4
        fn_state.ga_12675 = s_125_4;
        // D s_125_6: read-var ga#12675.0:struct
        let s_125_6: bool = fn_state.ga_12675._0;
        // D s_125_7: read-var ga#12675.1:struct
        let s_125_7: bool = fn_state.ga_12675._1;
        // D s_125_8: read-var ga#12675.2:struct
        let s_125_8: bool = fn_state.ga_12675._2;
        // D s_125_9: read-var ga#12675.3:struct
        let s_125_9: bool = fn_state.ga_12675._3;
        // D s_125_10: write-var pr <= s_125_6
        fn_state.pr = s_125_6;
        // D s_125_11: write-var pw <= s_125_7
        fn_state.pw = s_125_7;
        // D s_125_12: write-var px <= s_125_8
        fn_state.px = s_125_8;
        // D s_125_13: write-var pgcs <= s_125_9
        fn_state.pgcs = s_125_9;
        // N s_125_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_126_0: read-var ga#12661:u8
        let s_126_0: u8 = fn_state.ga_12661;
        // D s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 4u16);
        // C s_126_2: const #14u : u8
        let s_126_2: u8 = 14;
        // C s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 4u16);
        // D s_126_4: cmp-eq s_126_1 s_126_3
        let s_126_4: bool = ((s_126_1) == (s_126_3));
        // D s_126_5: not s_126_4
        let s_126_5: bool = !s_126_4;
        // N s_126_6: branch s_126_5 b128 b127
        if s_126_5 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_127_0: const #1u : u8
        let s_127_0: bool = true;
        // C s_127_1: const #1u : u8
        let s_127_1: bool = true;
        // C s_127_2: const #1u : u8
        let s_127_2: bool = true;
        // C s_127_3: const #0u : u8
        let s_127_3: bool = false;
        // D s_127_4: create-product struct = ["s_127_0", "s_127_1", "s_127_2", "s_127_3"]
        let s_127_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_127_0,
            _1: s_127_1,
            _2: s_127_2,
            _3: s_127_3,
        };
        // D s_127_5: write-var ga#12676 <= s_127_4
        fn_state.ga_12676 = s_127_4;
        // D s_127_6: read-var ga#12676.0:struct
        let s_127_6: bool = fn_state.ga_12676._0;
        // D s_127_7: read-var ga#12676.1:struct
        let s_127_7: bool = fn_state.ga_12676._1;
        // D s_127_8: read-var ga#12676.2:struct
        let s_127_8: bool = fn_state.ga_12676._2;
        // D s_127_9: read-var ga#12676.3:struct
        let s_127_9: bool = fn_state.ga_12676._3;
        // D s_127_10: write-var pr <= s_127_6
        fn_state.pr = s_127_6;
        // D s_127_11: write-var pw <= s_127_7
        fn_state.pw = s_127_7;
        // D s_127_12: write-var px <= s_127_8
        fn_state.px = s_127_8;
        // D s_127_13: write-var pgcs <= s_127_9
        fn_state.pgcs = s_127_9;
        // N s_127_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_128_0: const #0u : u8
        let s_128_0: bool = false;
        // C s_128_1: const #0u : u8
        let s_128_1: bool = false;
        // C s_128_2: const #0u : u8
        let s_128_2: bool = false;
        // C s_128_3: const #0u : u8
        let s_128_3: bool = false;
        // D s_128_4: create-product struct = ["s_128_0", "s_128_1", "s_128_2", "s_128_3"]
        let s_128_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_128_0,
            _1: s_128_1,
            _2: s_128_2,
            _3: s_128_3,
        };
        // D s_128_5: write-var ga#12677 <= s_128_4
        fn_state.ga_12677 = s_128_4;
        // D s_128_6: read-var ga#12677.0:struct
        let s_128_6: bool = fn_state.ga_12677._0;
        // D s_128_7: read-var ga#12677.1:struct
        let s_128_7: bool = fn_state.ga_12677._1;
        // D s_128_8: read-var ga#12677.2:struct
        let s_128_8: bool = fn_state.ga_12677._2;
        // D s_128_9: read-var ga#12677.3:struct
        let s_128_9: bool = fn_state.ga_12677._3;
        // D s_128_10: write-var pr <= s_128_6
        fn_state.pr = s_128_6;
        // D s_128_11: write-var pw <= s_128_7
        fn_state.pw = s_128_7;
        // D s_128_12: write-var px <= s_128_8
        fn_state.px = s_128_8;
        // D s_128_13: write-var pgcs <= s_128_9
        fn_state.pgcs = s_128_9;
        // N s_128_14: jump b2
        return block_2(state, tracer, fn_state);
    }
}
