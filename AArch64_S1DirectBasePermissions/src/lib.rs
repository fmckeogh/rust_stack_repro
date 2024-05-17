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
use u__IMPDEF_boolean::*;
use HavePANExt::*;
use HasUnprivileged::*;
use common::*;
pub fn AArch64_S1DirectBasePermissions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    walkstate: ProductType96e7acababe246a1,
    walkparams: ProductTypeef284266e139aee2,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductTypea231b9ca5c98dc3c {
    #[derive(Default)]
    struct FunctionState {
        ga_12577: ProductTypede60d0d1f6e7c94c,
        ga_12628: ProductTyped8f896a024a4e2cb,
        r: bool,
        gs_16872: bool,
        ga_12630: ProductTypeda0231e9dc169f81,
        ga_12593: ProductTypeda0231e9dc169f81,
        ga_12575: ProductTypede60d0d1f6e7c94c,
        gs_16858: bool,
        ga_12617: ProductType8b847afc727d5818,
        ga_12570: ProductTypede60d0d1f6e7c94c,
        gs_16861: bool,
        ga_12569: ProductTypede60d0d1f6e7c94c,
        gs_16859: bool,
        gs_16862: bool,
        ga_12576: ProductTypede60d0d1f6e7c94c,
        gs_16854: bool,
        permissions: ProductTypebf05c51f33174538,
        wxn: bool,
        ga_12644: ProductTypeda0231e9dc169f81,
        w: bool,
        gs_16873: bool,
        gs_16855: bool,
        gs_16875: bool,
        ga_12637: ProductTypeda0231e9dc169f81,
        x: bool,
        ux: bool,
        pw: bool,
        ga_12568: u8,
        ga_12621: ProductType8b847afc727d5818,
        ga_12571: ProductTypede60d0d1f6e7c94c,
        ga_12572: ProductTypede60d0d1f6e7c94c,
        gs_16856: bool,
        gs_16871: bool,
        ga_12616: ProductType8b847afc727d5818,
        px: bool,
        s1perms: ProductTypea231b9ca5c98dc3c,
        pr: bool,
        gs_16860: bool,
        gs_16863: bool,
        gs_16874: bool,
        ga_12574: ProductTypede60d0d1f6e7c94c,
        ur: bool,
        ga_12602: ProductTypeda0231e9dc169f81,
        ga_12573: u8,
        uw: bool,
        ga_12622: ProductType8b847afc727d5818,
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
        // D s_0_2: read-var regime:u32
        let s_0_2: u32 = fn_state.regime;
        // D s_0_3: call HasUnprivileged(s_0_2)
        let s_0_3: bool = HasUnprivileged(state, tracer, s_0_2);
        // N s_0_4: branch s_0_3 b36 b1
        if s_0_3 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_1_0: read-var permissions.0:struct
        let s_1_0: u8 = fn_state.permissions._0;
        // C s_1_1: const #2s : i
        let s_1_1: i128 = 2;
        // D s_1_2: cast zx s_1_0 -> bv
        let s_1_2: Bits = Bits::new(s_1_0 as u128, 3u16);
        // C s_1_3: const #1u : u64
        let s_1_3: u64 = 1;
        // D s_1_4: bit-extract s_1_2 s_1_1 s_1_3
        let s_1_4: Bits = (Bits::new(
            ((s_1_2) >> (s_1_1)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // C s_1_6: const #0s : i
        let s_1_6: i128 = 0;
        // C s_1_7: const #0u : u64
        let s_1_7: u64 = 0;
        // D s_1_8: cast zx s_1_5 -> u64
        let s_1_8: u64 = (s_1_5 as u64);
        // C s_1_9: const #1u : u64
        let s_1_9: u64 = 1;
        // D s_1_10: and s_1_8 s_1_9
        let s_1_10: u64 = ((s_1_8) & (s_1_9));
        // D s_1_11: cmp-eq s_1_10 s_1_9
        let s_1_11: bool = ((s_1_10) == (s_1_9));
        // D s_1_12: lsl s_1_8 s_1_6
        let s_1_12: u64 = s_1_8 << s_1_6;
        // D s_1_13: or s_1_7 s_1_12
        let s_1_13: u64 = ((s_1_7) | (s_1_12));
        // D s_1_14: cmpl s_1_12
        let s_1_14: u64 = !s_1_12;
        // D s_1_15: and s_1_7 s_1_14
        let s_1_15: u64 = ((s_1_7) & (s_1_14));
        // D s_1_16: select s_1_11 s_1_13 s_1_15
        let s_1_16: u64 = if s_1_11 { s_1_13 } else { s_1_15 };
        // D s_1_17: cast trunc s_1_16 -> u8
        let s_1_17: bool = ((s_1_16) != 0);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // C s_1_19: const #0u : u8
        let s_1_19: bool = false;
        // C s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // D s_1_21: cmp-eq s_1_18 s_1_20
        let s_1_21: bool = ((s_1_18) == (s_1_20));
        // D s_1_22: not s_1_21
        let s_1_22: bool = !s_1_21;
        // N s_1_23: branch s_1_22 b35 b2
        if s_1_22 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_2_0: const #1u : u8
        let s_2_0: bool = true;
        // C s_2_1: const #1u : u8
        let s_2_1: bool = true;
        // D s_2_2: create-product struct = ["s_2_0", "s_2_1"]
        let s_2_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_2_0,
            _1: s_2_1,
        };
        // D s_2_3: write-var ga#12616 <= s_2_2
        fn_state.ga_12616 = s_2_2;
        // D s_2_4: read-var ga#12616.0:struct
        let s_2_4: bool = fn_state.ga_12616._0;
        // D s_2_5: read-var ga#12616.1:struct
        let s_2_5: bool = fn_state.ga_12616._1;
        // D s_2_6: write-var pr <= s_2_4
        fn_state.pr = s_2_4;
        // D s_2_7: write-var pw <= s_2_5
        fn_state.pw = s_2_5;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_3_0: read-var permissions.1:struct
        let s_3_0: u8 = fn_state.permissions._1;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: cast zx s_3_0 -> bv
        let s_3_2: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_1 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_1)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // C s_3_19: const #0u : u8
        let s_3_19: bool = false;
        // C s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // D s_3_21: cmp-eq s_3_18 s_3_20
        let s_3_21: bool = ((s_3_18) == (s_3_20));
        // D s_3_22: not s_3_21
        let s_3_22: bool = !s_3_21;
        // N s_3_23: branch s_3_22 b34 b4
        if s_3_22 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_4_0: read-var pr:u8
        let s_4_0: bool = fn_state.pr;
        // D s_4_1: read-var pw:u8
        let s_4_1: bool = fn_state.pw;
        // D s_4_2: create-product struct = ["s_4_0", "s_4_1"]
        let s_4_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_4_0,
            _1: s_4_1,
        };
        // D s_4_3: write-var ga#12621 <= s_4_2
        fn_state.ga_12621 = s_4_2;
        // D s_4_4: read-var ga#12621.0:struct
        let s_4_4: bool = fn_state.ga_12621._0;
        // D s_4_5: read-var ga#12621.1:struct
        let s_4_5: bool = fn_state.ga_12621._1;
        // D s_4_6: write-var pr <= s_4_4
        fn_state.pr = s_4_4;
        // D s_4_7: write-var pw <= s_4_5
        fn_state.pw = s_4_5;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_5_0: read-var permissions.17:struct
        let s_5_0: bool = fn_state.permissions._17;
        // D s_5_1: read-var permissions.18:struct
        let s_5_1: bool = fn_state.permissions._18;
        // D s_5_2: cast zx s_5_0 -> bv
        let s_5_2: Bits = Bits::new(s_5_0 as u128, 1u16);
        // D s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 1u16);
        // D s_5_4: or s_5_2 s_5_3
        let s_5_4: Bits = ((s_5_2) | (s_5_3));
        // D s_5_5: cast reint s_5_4 -> u8
        let s_5_5: bool = ((s_5_4.value()) != 0);
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 1u16);
        // D s_5_7: not s_5_6
        let s_5_7: Bits = !s_5_6;
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: bool = ((s_5_7.value()) != 0);
        // D s_5_9: write-var px <= s_5_8
        fn_state.px = s_5_8;
        // N s_5_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_6_0: read-var accdesc.8:struct
        let s_6_0: u8 = fn_state.accdesc._8;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #448u : u32
        let s_6_2: u32 = 448;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b33 b7
        if s_6_5 {
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
        // D s_7_3: create-product struct = ["s_7_0", "s_7_1", "s_7_2"]
        let s_7_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_7_0,
            _1: s_7_1,
            _2: s_7_2,
        };
        // D s_7_4: write-var ga#12628 <= s_7_3
        fn_state.ga_12628 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_8_0: read-var ga#12628.0:struct
        let s_8_0: bool = fn_state.ga_12628._0;
        // D s_8_1: read-var ga#12628.1:struct
        let s_8_1: bool = fn_state.ga_12628._1;
        // D s_8_2: read-var ga#12628.2:struct
        let s_8_2: bool = fn_state.ga_12628._2;
        // D s_8_3: write-var r <= s_8_0
        fn_state.r = s_8_0;
        // D s_8_4: write-var w <= s_8_1
        fn_state.w = s_8_1;
        // D s_8_5: write-var x <= s_8_2
        fn_state.x = s_8_2;
        // D s_8_6: read-var walkparams.39:struct
        let s_8_6: bool = fn_state.walkparams._39;
        // D s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 1u16);
        // D s_8_8: read-var w:u8
        let s_8_8: bool = fn_state.w;
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 1u16);
        // D s_8_10: and s_8_7 s_8_9
        let s_8_10: Bits = ((s_8_7) & (s_8_9));
        // D s_8_11: cast reint s_8_10 -> u8
        let s_8_11: bool = ((s_8_10.value()) != 0);
        // D s_8_12: cast zx s_8_11 -> bv
        let s_8_12: Bits = Bits::new(s_8_11 as u128, 1u16);
        // D s_8_13: read-var x:u8
        let s_8_13: bool = fn_state.x;
        // D s_8_14: cast zx s_8_13 -> bv
        let s_8_14: Bits = Bits::new(s_8_13 as u128, 1u16);
        // D s_8_15: and s_8_12 s_8_14
        let s_8_15: Bits = ((s_8_12) & (s_8_14));
        // D s_8_16: cast reint s_8_15 -> u8
        let s_8_16: bool = ((s_8_15.value()) != 0);
        // D s_8_17: write-var wxn <= s_8_16
        fn_state.wxn = s_8_16;
        // D s_8_18: read-var accdesc.25:struct
        let s_8_18: u32 = fn_state.accdesc._25;
        // C s_8_19: const #3u : u32
        let s_8_19: u32 = 3;
        // D s_8_20: cmp-eq s_8_18 s_8_19
        let s_8_20: bool = ((s_8_18) == (s_8_19));
        // N s_8_21: branch s_8_20 b32 b9
        if s_8_20 {
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
        // D s_9_1: write-var gs#16871 <= s_9_0
        fn_state.gs_16871 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_10_0: read-var gs#16871:u8
        let s_10_0: bool = fn_state.gs_16871;
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
        // D s_13_1: write-var gs#16872 <= s_13_0
        fn_state.gs_16872 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_14_0: read-var gs#16872:u8
        let s_14_0: bool = fn_state.gs_16872;
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
        // D s_17_1: write-var gs#16874 <= s_17_0
        fn_state.gs_16874 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_18_0: read-var gs#16874:u8
        let s_18_0: bool = fn_state.gs_16874;
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
        // D s_19_1: write-var gs#16875 <= s_19_0
        fn_state.gs_16875 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_20_0: read-var gs#16875:u8
        let s_20_0: bool = fn_state.gs_16875;
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
        // D s_22_1: read-var r:u8
        let s_22_1: bool = fn_state.r;
        // D s_22_2: write-var s1perms.5 <= s_22_1
        fn_state.s1perms._5 = s_22_1;
        // D s_22_3: read-var w:u8
        let s_22_3: bool = fn_state.w;
        // D s_22_4: write-var s1perms.6 <= s_22_3
        fn_state.s1perms._6 = s_22_3;
        // D s_22_5: write-var s1perms.8 <= s_22_0
        fn_state.s1perms._8 = s_22_0;
        // C s_22_6: const #0u : u8
        let s_22_6: bool = false;
        // D s_22_7: write-var s1perms.0 <= s_22_6
        fn_state.s1perms._0 = s_22_6;
        // D s_22_8: read-var wxn:u8
        let s_22_8: bool = fn_state.wxn;
        // D s_22_9: write-var s1perms.7 <= s_22_8
        fn_state.s1perms._7 = s_22_8;
        // C s_22_10: const #1u : u8
        let s_22_10: bool = true;
        // D s_22_11: write-var s1perms.2 <= s_22_10
        fn_state.s1perms._2 = s_22_10;
        // D s_22_12: read-var s1perms:struct
        let s_22_12: ProductTypea231b9ca5c98dc3c = fn_state.s1perms;
        // N s_22_13: return s_22_12
        return s_22_12;
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
        // N s_23_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_24_0: read-var walkstate.0:struct
        let s_24_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_24_1: write-var ga#12644 <= s_24_0
        fn_state.ga_12644 = s_24_0;
        // D s_24_2: read-var ga#12644.1:struct
        let s_24_2: u32 = fn_state.ga_12644._1;
        // C s_24_3: const #3u : u32
        let s_24_3: u32 = 3;
        // D s_24_4: cmp-eq s_24_2 s_24_3
        let s_24_4: bool = ((s_24_2) == (s_24_3));
        // D s_24_5: write-var gs#16875 <= s_24_4
        fn_state.gs_16875 = s_24_4;
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
        // D s_26_3: write-var gs#16873 <= s_26_2
        fn_state.gs_16873 = s_26_2;
        // N s_26_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_27_0: read-var gs#16873:u8
        let s_27_0: bool = fn_state.gs_16873;
        // D s_27_1: write-var gs#16874 <= s_27_0
        fn_state.gs_16874 = s_27_0;
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
        // D s_28_1: write-var gs#16873 <= s_28_0
        fn_state.gs_16873 = s_28_0;
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
        // N s_29_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_30_0: read-var walkstate.0:struct
        let s_30_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_30_1: write-var ga#12637 <= s_30_0
        fn_state.ga_12637 = s_30_0;
        // D s_30_2: read-var ga#12637.1:struct
        let s_30_2: u32 = fn_state.ga_12637._1;
        // C s_30_3: const #2u : u32
        let s_30_3: u32 = 2;
        // D s_30_4: cmp-eq s_30_2 s_30_3
        let s_30_4: bool = ((s_30_2) == (s_30_3));
        // D s_30_5: write-var gs#16872 <= s_30_4
        fn_state.gs_16872 = s_30_4;
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
        // N s_31_10: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_32_0: read-var walkstate.0:struct
        let s_32_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_32_1: write-var ga#12630 <= s_32_0
        fn_state.ga_12630 = s_32_0;
        // D s_32_2: read-var ga#12630.1:struct
        let s_32_2: u32 = fn_state.ga_12630._1;
        // C s_32_3: const #0u : u32
        let s_32_3: u32 = 0;
        // D s_32_4: cmp-eq s_32_2 s_32_3
        let s_32_4: bool = ((s_32_2) == (s_32_3));
        // D s_32_5: write-var gs#16871 <= s_32_4
        fn_state.gs_16871 = s_32_4;
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
        // D s_33_3: create-product struct = ["s_33_0", "s_33_1", "s_33_2"]
        let s_33_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_33_0,
            _1: s_33_1,
            _2: s_33_2,
        };
        // D s_33_4: write-var ga#12628 <= s_33_3
        fn_state.ga_12628 = s_33_3;
        // N s_33_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_34_0: read-var pr:u8
        let s_34_0: bool = fn_state.pr;
        // C s_34_1: const #0u : u8
        let s_34_1: bool = false;
        // D s_34_2: create-product struct = ["s_34_0", "s_34_1"]
        let s_34_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_34_0,
            _1: s_34_1,
        };
        // D s_34_3: write-var ga#12622 <= s_34_2
        fn_state.ga_12622 = s_34_2;
        // D s_34_4: read-var ga#12622.0:struct
        let s_34_4: bool = fn_state.ga_12622._0;
        // D s_34_5: read-var ga#12622.1:struct
        let s_34_5: bool = fn_state.ga_12622._1;
        // D s_34_6: write-var pr <= s_34_4
        fn_state.pr = s_34_4;
        // D s_34_7: write-var pw <= s_34_5
        fn_state.pw = s_34_5;
        // N s_34_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // C s_35_1: const #0u : u8
        let s_35_1: bool = false;
        // D s_35_2: create-product struct = ["s_35_0", "s_35_1"]
        let s_35_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_35_0,
            _1: s_35_1,
        };
        // D s_35_3: write-var ga#12617 <= s_35_2
        fn_state.ga_12617 = s_35_2;
        // D s_35_4: read-var ga#12617.0:struct
        let s_35_4: bool = fn_state.ga_12617._0;
        // D s_35_5: read-var ga#12617.1:struct
        let s_35_5: bool = fn_state.ga_12617._1;
        // D s_35_6: write-var pr <= s_35_4
        fn_state.pr = s_35_4;
        // D s_35_7: write-var pw <= s_35_5
        fn_state.pw = s_35_5;
        // N s_35_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_36_0: read-var permissions.0:struct
        let s_36_0: u8 = fn_state.permissions._0;
        // C s_36_1: const #1s : i
        let s_36_1: i128 = 1;
        // D s_36_2: cast zx s_36_0 -> bv
        let s_36_2: Bits = Bits::new(s_36_0 as u128, 3u16);
        // C s_36_3: const #1s : i64
        let s_36_3: i64 = 1;
        // C s_36_4: cast zx s_36_3 -> i
        let s_36_4: i128 = (i128::try_from(s_36_3).unwrap());
        // C s_36_5: const #1s : i
        let s_36_5: i128 = 1;
        // C s_36_6: add s_36_5 s_36_4
        let s_36_6: i128 = (s_36_5 + s_36_4);
        // D s_36_7: bit-extract s_36_2 s_36_1 s_36_6
        let s_36_7: Bits = (Bits::new(
            ((s_36_2) >> (s_36_1)).value(),
            u16::try_from(s_36_6).unwrap(),
        ));
        // D s_36_8: cast reint s_36_7 -> u8
        let s_36_8: u8 = (s_36_7.value() as u8);
        // D s_36_9: write-var ga#12568 <= s_36_8
        fn_state.ga_12568 = s_36_8;
        // D s_36_10: read-var ga#12568:u8
        let s_36_10: u8 = fn_state.ga_12568;
        // D s_36_11: cast zx s_36_10 -> bv
        let s_36_11: Bits = Bits::new(s_36_10 as u128, 2u16);
        // C s_36_12: const #0u : u8
        let s_36_12: u8 = 0;
        // C s_36_13: cast zx s_36_12 -> bv
        let s_36_13: Bits = Bits::new(s_36_12 as u128, 2u16);
        // D s_36_14: cmp-eq s_36_11 s_36_13
        let s_36_14: bool = ((s_36_11) == (s_36_13));
        // D s_36_15: not s_36_14
        let s_36_15: bool = !s_36_14;
        // N s_36_16: branch s_36_15 b82 b37
        if s_36_15 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // C s_37_1: const #1u : u8
        let s_37_1: bool = true;
        // C s_37_2: const #0u : u8
        let s_37_2: bool = false;
        // C s_37_3: const #0u : u8
        let s_37_3: bool = false;
        // D s_37_4: create-product struct = ["s_37_0", "s_37_1", "s_37_2", "s_37_3"]
        let s_37_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_37_0,
            _1: s_37_1,
            _2: s_37_2,
            _3: s_37_3,
        };
        // D s_37_5: write-var ga#12569 <= s_37_4
        fn_state.ga_12569 = s_37_4;
        // D s_37_6: read-var ga#12569.0:struct
        let s_37_6: bool = fn_state.ga_12569._0;
        // D s_37_7: read-var ga#12569.1:struct
        let s_37_7: bool = fn_state.ga_12569._1;
        // D s_37_8: read-var ga#12569.2:struct
        let s_37_8: bool = fn_state.ga_12569._2;
        // D s_37_9: read-var ga#12569.3:struct
        let s_37_9: bool = fn_state.ga_12569._3;
        // D s_37_10: write-var pr <= s_37_6
        fn_state.pr = s_37_6;
        // D s_37_11: write-var pw <= s_37_7
        fn_state.pw = s_37_7;
        // D s_37_12: write-var ur <= s_37_8
        fn_state.ur = s_37_8;
        // D s_37_13: write-var uw <= s_37_9
        fn_state.uw = s_37_9;
        // N s_37_14: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_38_0: read-var permissions.1:struct
        let s_38_0: u8 = fn_state.permissions._1;
        // D s_38_1: write-var ga#12573 <= s_38_0
        fn_state.ga_12573 = s_38_0;
        // D s_38_2: read-var ga#12573:u8
        let s_38_2: u8 = fn_state.ga_12573;
        // D s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 2u16);
        // C s_38_4: const #0u : u8
        let s_38_4: u8 = 0;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 2u16);
        // D s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // D s_38_7: not s_38_6
        let s_38_7: bool = !s_38_6;
        // N s_38_8: branch s_38_7 b77 b39
        if s_38_7 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_39_0: read-var pr:u8
        let s_39_0: bool = fn_state.pr;
        // D s_39_1: read-var pw:u8
        let s_39_1: bool = fn_state.pw;
        // D s_39_2: read-var ur:u8
        let s_39_2: bool = fn_state.ur;
        // D s_39_3: read-var uw:u8
        let s_39_3: bool = fn_state.uw;
        // D s_39_4: create-product struct = ["s_39_0", "s_39_1", "s_39_2", "s_39_3"]
        let s_39_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_39_0,
            _1: s_39_1,
            _2: s_39_2,
            _3: s_39_3,
        };
        // D s_39_5: write-var ga#12574 <= s_39_4
        fn_state.ga_12574 = s_39_4;
        // D s_39_6: read-var ga#12574.0:struct
        let s_39_6: bool = fn_state.ga_12574._0;
        // D s_39_7: read-var ga#12574.1:struct
        let s_39_7: bool = fn_state.ga_12574._1;
        // D s_39_8: read-var ga#12574.2:struct
        let s_39_8: bool = fn_state.ga_12574._2;
        // D s_39_9: read-var ga#12574.3:struct
        let s_39_9: bool = fn_state.ga_12574._3;
        // D s_39_10: write-var pr <= s_39_6
        fn_state.pr = s_39_6;
        // D s_39_11: write-var pw <= s_39_7
        fn_state.pw = s_39_7;
        // D s_39_12: write-var ur <= s_39_8
        fn_state.ur = s_39_8;
        // D s_39_13: write-var uw <= s_39_9
        fn_state.uw = s_39_9;
        // N s_39_14: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_40_0: read-var permissions.5:struct
        let s_40_0: bool = fn_state.permissions._5;
        // D s_40_1: read-var permissions.6:struct
        let s_40_1: bool = fn_state.permissions._6;
        // D s_40_2: cast zx s_40_0 -> bv
        let s_40_2: Bits = Bits::new(s_40_0 as u128, 1u16);
        // D s_40_3: cast zx s_40_1 -> bv
        let s_40_3: Bits = Bits::new(s_40_1 as u128, 1u16);
        // D s_40_4: or s_40_2 s_40_3
        let s_40_4: Bits = ((s_40_2) | (s_40_3));
        // D s_40_5: cast reint s_40_4 -> u8
        let s_40_5: bool = ((s_40_4.value()) != 0);
        // D s_40_6: cast zx s_40_5 -> bv
        let s_40_6: Bits = Bits::new(s_40_5 as u128, 1u16);
        // D s_40_7: read-var uw:u8
        let s_40_7: bool = fn_state.uw;
        // D s_40_8: cast zx s_40_7 -> bv
        let s_40_8: Bits = Bits::new(s_40_7 as u128, 1u16);
        // D s_40_9: or s_40_6 s_40_8
        let s_40_9: Bits = ((s_40_6) | (s_40_8));
        // D s_40_10: cast reint s_40_9 -> u8
        let s_40_10: bool = ((s_40_9.value()) != 0);
        // D s_40_11: cast zx s_40_10 -> bv
        let s_40_11: Bits = Bits::new(s_40_10 as u128, 1u16);
        // D s_40_12: not s_40_11
        let s_40_12: Bits = !s_40_11;
        // D s_40_13: cast reint s_40_12 -> u8
        let s_40_13: bool = ((s_40_12.value()) != 0);
        // D s_40_14: write-var px <= s_40_13
        fn_state.px = s_40_13;
        // D s_40_15: read-var permissions.15:struct
        let s_40_15: bool = fn_state.permissions._15;
        // D s_40_16: read-var permissions.16:struct
        let s_40_16: bool = fn_state.permissions._16;
        // D s_40_17: cast zx s_40_15 -> bv
        let s_40_17: Bits = Bits::new(s_40_15 as u128, 1u16);
        // D s_40_18: cast zx s_40_16 -> bv
        let s_40_18: Bits = Bits::new(s_40_16 as u128, 1u16);
        // D s_40_19: or s_40_17 s_40_18
        let s_40_19: Bits = ((s_40_17) | (s_40_18));
        // D s_40_20: cast reint s_40_19 -> u8
        let s_40_20: bool = ((s_40_19.value()) != 0);
        // D s_40_21: cast zx s_40_20 -> bv
        let s_40_21: Bits = Bits::new(s_40_20 as u128, 1u16);
        // D s_40_22: not s_40_21
        let s_40_22: Bits = !s_40_21;
        // D s_40_23: cast reint s_40_22 -> u8
        let s_40_23: bool = ((s_40_22.value()) != 0);
        // D s_40_24: write-var ux <= s_40_23
        fn_state.ux = s_40_23;
        // C s_40_25: const #() : ()
        let s_40_25: () = ();
        // S s_40_26: call HavePANExt(s_40_25)
        let s_40_26: bool = HavePANExt(state, tracer, s_40_25);
        // N s_40_27: branch s_40_26 b76 b41
        if s_40_26 {
            return block_76(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#16854 <= s_41_0
        fn_state.gs_16854 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_42_0: read-var gs#16854:u8
        let s_42_0: bool = fn_state.gs_16854;
        // N s_42_1: branch s_42_0 b72 b43
        if s_42_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#16856 <= s_43_0
        fn_state.gs_16856 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_44_0: read-var gs#16856:u8
        let s_44_0: bool = fn_state.gs_16856;
        // N s_44_1: branch s_44_0 b47 b45
        if s_44_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_46_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_47_0: const #"SCR_EL3.SIF affects EPAN" : str
        let s_47_0: &'static str = "SCR_EL3.SIF affects EPAN";
        // S s_47_1: call __IMPDEF_boolean(s_47_0)
        let s_47_1: bool = u__IMPDEF_boolean(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b71 b48
        if s_47_1 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#16858 <= s_48_0
        fn_state.gs_16858 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_49_0: read-var gs#16858:u8
        let s_49_0: bool = fn_state.gs_16858;
        // N s_49_1: branch s_49_0 b70 b50
        if s_49_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#16859 <= s_50_0
        fn_state.gs_16859 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_51_0: read-var gs#16859:u8
        let s_51_0: bool = fn_state.gs_16859;
        // N s_51_1: branch s_51_0 b69 b52
        if s_51_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#16860 <= s_52_0
        fn_state.gs_16860 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_53_0: read-var gs#16860:u8
        let s_53_0: bool = fn_state.gs_16860;
        // N s_53_1: branch s_53_0 b68 b54
        if s_53_0 {
            return block_68(state, tracer, fn_state);
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
        // C s_55_0: const #"Realm EL2&0 regime affects EPAN" : str
        let s_55_0: &'static str = "Realm EL2&0 regime affects EPAN";
        // S s_55_1: call __IMPDEF_boolean(s_55_0)
        let s_55_1: bool = u__IMPDEF_boolean(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b67 b56
        if s_55_1 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#16861 <= s_56_0
        fn_state.gs_16861 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_57_0: read-var gs#16861:u8
        let s_57_0: bool = fn_state.gs_16861;
        // N s_57_1: branch s_57_0 b66 b58
        if s_57_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#16862 <= s_58_0
        fn_state.gs_16862 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_59_0: read-var gs#16862:u8
        let s_59_0: bool = fn_state.gs_16862;
        // N s_59_1: branch s_59_0 b65 b60
        if s_59_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#16863 <= s_60_0
        fn_state.gs_16863 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_61_0: read-var gs#16863:u8
        let s_61_0: bool = fn_state.gs_16863;
        // N s_61_1: branch s_61_0 b64 b62
        if s_61_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_62_0: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_63_0: const #16985u : u32
        let s_63_0: u32 = 16985;
        // D s_63_1: read-reg s_63_0:u8
        let s_63_1: bool = {
            let value = state.read_register::<bool>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: read-var ur:u8
        let s_63_2: bool = fn_state.ur;
        // D s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: read-var uw:u8
        let s_63_4: bool = fn_state.uw;
        // D s_63_5: cast zx s_63_4 -> bv
        let s_63_5: Bits = Bits::new(s_63_4 as u128, 1u16);
        // D s_63_6: or s_63_3 s_63_5
        let s_63_6: Bits = ((s_63_3) | (s_63_5));
        // D s_63_7: cast reint s_63_6 -> u8
        let s_63_7: bool = ((s_63_6.value()) != 0);
        // D s_63_8: read-var walkparams.11:struct
        let s_63_8: bool = fn_state.walkparams._11;
        // D s_63_9: cast zx s_63_8 -> bv
        let s_63_9: Bits = Bits::new(s_63_8 as u128, 1u16);
        // D s_63_10: read-var ux:u8
        let s_63_10: bool = fn_state.ux;
        // D s_63_11: cast zx s_63_10 -> bv
        let s_63_11: Bits = Bits::new(s_63_10 as u128, 1u16);
        // D s_63_12: and s_63_9 s_63_11
        let s_63_12: Bits = ((s_63_9) & (s_63_11));
        // D s_63_13: cast reint s_63_12 -> u8
        let s_63_13: bool = ((s_63_12.value()) != 0);
        // D s_63_14: cast zx s_63_7 -> bv
        let s_63_14: Bits = Bits::new(s_63_7 as u128, 1u16);
        // D s_63_15: cast zx s_63_13 -> bv
        let s_63_15: Bits = Bits::new(s_63_13 as u128, 1u16);
        // D s_63_16: or s_63_14 s_63_15
        let s_63_16: Bits = ((s_63_14) | (s_63_15));
        // D s_63_17: cast reint s_63_16 -> u8
        let s_63_17: bool = ((s_63_16.value()) != 0);
        // D s_63_18: cast zx s_63_1 -> bv
        let s_63_18: Bits = Bits::new(s_63_1 as u128, 1u16);
        // D s_63_19: cast zx s_63_17 -> bv
        let s_63_19: Bits = Bits::new(s_63_17 as u128, 1u16);
        // D s_63_20: and s_63_18 s_63_19
        let s_63_20: Bits = ((s_63_18) & (s_63_19));
        // D s_63_21: cast reint s_63_20 -> u8
        let s_63_21: bool = ((s_63_20.value()) != 0);
        // D s_63_22: cast zx s_63_21 -> bv
        let s_63_22: Bits = Bits::new(s_63_21 as u128, 1u16);
        // D s_63_23: not s_63_22
        let s_63_23: Bits = !s_63_22;
        // D s_63_24: cast reint s_63_23 -> u8
        let s_63_24: bool = ((s_63_23.value()) != 0);
        // D s_63_25: read-var pr:u8
        let s_63_25: bool = fn_state.pr;
        // D s_63_26: cast zx s_63_25 -> bv
        let s_63_26: Bits = Bits::new(s_63_25 as u128, 1u16);
        // D s_63_27: cast zx s_63_24 -> bv
        let s_63_27: Bits = Bits::new(s_63_24 as u128, 1u16);
        // D s_63_28: and s_63_26 s_63_27
        let s_63_28: Bits = ((s_63_26) & (s_63_27));
        // D s_63_29: cast reint s_63_28 -> u8
        let s_63_29: bool = ((s_63_28.value()) != 0);
        // D s_63_30: write-var pr <= s_63_29
        fn_state.pr = s_63_29;
        // D s_63_31: cast zx s_63_21 -> bv
        let s_63_31: Bits = Bits::new(s_63_21 as u128, 1u16);
        // D s_63_32: not s_63_31
        let s_63_32: Bits = !s_63_31;
        // D s_63_33: cast reint s_63_32 -> u8
        let s_63_33: bool = ((s_63_32.value()) != 0);
        // D s_63_34: read-var pw:u8
        let s_63_34: bool = fn_state.pw;
        // D s_63_35: cast zx s_63_34 -> bv
        let s_63_35: Bits = Bits::new(s_63_34 as u128, 1u16);
        // D s_63_36: cast zx s_63_33 -> bv
        let s_63_36: Bits = Bits::new(s_63_33 as u128, 1u16);
        // D s_63_37: and s_63_35 s_63_36
        let s_63_37: Bits = ((s_63_35) & (s_63_36));
        // D s_63_38: cast reint s_63_37 -> u8
        let s_63_38: bool = ((s_63_37.value()) != 0);
        // D s_63_39: write-var pw <= s_63_38
        fn_state.pw = s_63_38;
        // N s_63_40: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var ux <= s_64_0
        fn_state.ux = s_64_0;
        // N s_64_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_65_0: read-var walkstate.0:struct
        let s_65_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_65_1: write-var ga#12602 <= s_65_0
        fn_state.ga_12602 = s_65_0;
        // D s_65_2: read-var ga#12602.1:struct
        let s_65_2: u32 = fn_state.ga_12602._1;
        // C s_65_3: const #3u : u32
        let s_65_3: u32 = 3;
        // D s_65_4: cmp-eq s_65_2 s_65_3
        let s_65_4: bool = ((s_65_2) == (s_65_3));
        // D s_65_5: write-var gs#16863 <= s_65_4
        fn_state.gs_16863 = s_65_4;
        // N s_65_6: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_66_0: read-var regime:u32
        let s_66_0: u32 = fn_state.regime;
        // C s_66_1: const #3u : u32
        let s_66_1: u32 = 3;
        // D s_66_2: cmp-eq s_66_0 s_66_1
        let s_66_2: bool = ((s_66_0) == (s_66_1));
        // D s_66_3: write-var gs#16862 <= s_66_2
        fn_state.gs_16862 = s_66_2;
        // N s_66_4: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_67_0: read-var accdesc.25:struct
        let s_67_0: u32 = fn_state.accdesc._25;
        // C s_67_1: const #2u : u32
        let s_67_1: u32 = 2;
        // D s_67_2: cmp-eq s_67_0 s_67_1
        let s_67_2: bool = ((s_67_0) == (s_67_1));
        // D s_67_3: write-var gs#16861 <= s_67_2
        fn_state.gs_16861 = s_67_2;
        // N s_67_4: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var ux <= s_68_0
        fn_state.ux = s_68_0;
        // N s_68_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_69_0: read-var walkparams.30:struct
        let s_69_0: bool = fn_state.walkparams._30;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#16860 <= s_69_4
        fn_state.gs_16860 = s_69_4;
        // N s_69_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_70_0: read-var walkstate.0:struct
        let s_70_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_70_1: write-var ga#12593 <= s_70_0
        fn_state.ga_12593 = s_70_0;
        // D s_70_2: read-var ga#12593.1:struct
        let s_70_2: u32 = fn_state.ga_12593._1;
        // C s_70_3: const #0u : u32
        let s_70_3: u32 = 0;
        // D s_70_4: cmp-eq s_70_2 s_70_3
        let s_70_4: bool = ((s_70_2) == (s_70_3));
        // D s_70_5: write-var gs#16859 <= s_70_4
        fn_state.gs_16859 = s_70_4;
        // N s_70_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_71_0: read-var accdesc.25:struct
        let s_71_0: u32 = fn_state.accdesc._25;
        // C s_71_1: const #3u : u32
        let s_71_1: u32 = 3;
        // D s_71_2: cmp-eq s_71_0 s_71_1
        let s_71_2: bool = ((s_71_0) == (s_71_1));
        // D s_71_3: write-var gs#16858 <= s_71_2
        fn_state.gs_16858 = s_71_2;
        // N s_71_4: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_72_0: read-var regime:u32
        let s_72_0: u32 = fn_state.regime;
        // C s_72_1: const #4u : u32
        let s_72_1: u32 = 4;
        // D s_72_2: cmp-eq s_72_0 s_72_1
        let s_72_2: bool = ((s_72_0) == (s_72_1));
        // N s_72_3: branch s_72_2 b75 b73
        if s_72_2 {
            return block_75(state, tracer, fn_state);
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
        // D s_73_1: write-var gs#16855 <= s_73_0
        fn_state.gs_16855 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_74_0: read-var gs#16855:u8
        let s_74_0: bool = fn_state.gs_16855;
        // D s_74_1: not s_74_0
        let s_74_1: bool = !s_74_0;
        // D s_74_2: write-var gs#16856 <= s_74_1
        fn_state.gs_16856 = s_74_1;
        // N s_74_3: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_75_0: read-var walkparams.22:struct
        let s_75_0: bool = fn_state.walkparams._22;
        // D s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 1u16);
        // C s_75_2: const #1u : u8
        let s_75_2: bool = true;
        // C s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_4: cmp-eq s_75_1 s_75_3
        let s_75_4: bool = ((s_75_1) == (s_75_3));
        // D s_75_5: write-var gs#16855 <= s_75_4
        fn_state.gs_16855 = s_75_4;
        // N s_75_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_76_0: read-var accdesc.20:struct
        let s_76_0: bool = fn_state.accdesc._20;
        // D s_76_1: write-var gs#16854 <= s_76_0
        fn_state.gs_16854 = s_76_0;
        // N s_76_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_77_0: read-var ga#12573:u8
        let s_77_0: u8 = fn_state.ga_12573;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 2u16);
        // C s_77_2: const #1u : u8
        let s_77_2: u8 = 1;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 2u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: not s_77_4
        let s_77_5: bool = !s_77_4;
        // N s_77_6: branch s_77_5 b79 b78
        if s_77_5 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_78_0: read-var pr:u8
        let s_78_0: bool = fn_state.pr;
        // D s_78_1: read-var pw:u8
        let s_78_1: bool = fn_state.pw;
        // C s_78_2: const #0u : u8
        let s_78_2: bool = false;
        // C s_78_3: const #0u : u8
        let s_78_3: bool = false;
        // D s_78_4: create-product struct = ["s_78_0", "s_78_1", "s_78_2", "s_78_3"]
        let s_78_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_78_0,
            _1: s_78_1,
            _2: s_78_2,
            _3: s_78_3,
        };
        // D s_78_5: write-var ga#12575 <= s_78_4
        fn_state.ga_12575 = s_78_4;
        // D s_78_6: read-var ga#12575.0:struct
        let s_78_6: bool = fn_state.ga_12575._0;
        // D s_78_7: read-var ga#12575.1:struct
        let s_78_7: bool = fn_state.ga_12575._1;
        // D s_78_8: read-var ga#12575.2:struct
        let s_78_8: bool = fn_state.ga_12575._2;
        // D s_78_9: read-var ga#12575.3:struct
        let s_78_9: bool = fn_state.ga_12575._3;
        // D s_78_10: write-var pr <= s_78_6
        fn_state.pr = s_78_6;
        // D s_78_11: write-var pw <= s_78_7
        fn_state.pw = s_78_7;
        // D s_78_12: write-var ur <= s_78_8
        fn_state.ur = s_78_8;
        // D s_78_13: write-var uw <= s_78_9
        fn_state.uw = s_78_9;
        // N s_78_14: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_79_0: read-var ga#12573:u8
        let s_79_0: u8 = fn_state.ga_12573;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 2u16);
        // C s_79_2: const #2u : u8
        let s_79_2: u8 = 2;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 2u16);
        // D s_79_4: cmp-eq s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) == (s_79_3));
        // D s_79_5: not s_79_4
        let s_79_5: bool = !s_79_4;
        // N s_79_6: branch s_79_5 b81 b80
        if s_79_5 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_80_0: read-var pr:u8
        let s_80_0: bool = fn_state.pr;
        // C s_80_1: const #0u : u8
        let s_80_1: bool = false;
        // D s_80_2: read-var ur:u8
        let s_80_2: bool = fn_state.ur;
        // C s_80_3: const #0u : u8
        let s_80_3: bool = false;
        // D s_80_4: create-product struct = ["s_80_0", "s_80_1", "s_80_2", "s_80_3"]
        let s_80_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_80_0,
            _1: s_80_1,
            _2: s_80_2,
            _3: s_80_3,
        };
        // D s_80_5: write-var ga#12576 <= s_80_4
        fn_state.ga_12576 = s_80_4;
        // D s_80_6: read-var ga#12576.0:struct
        let s_80_6: bool = fn_state.ga_12576._0;
        // D s_80_7: read-var ga#12576.1:struct
        let s_80_7: bool = fn_state.ga_12576._1;
        // D s_80_8: read-var ga#12576.2:struct
        let s_80_8: bool = fn_state.ga_12576._2;
        // D s_80_9: read-var ga#12576.3:struct
        let s_80_9: bool = fn_state.ga_12576._3;
        // D s_80_10: write-var pr <= s_80_6
        fn_state.pr = s_80_6;
        // D s_80_11: write-var pw <= s_80_7
        fn_state.pw = s_80_7;
        // D s_80_12: write-var ur <= s_80_8
        fn_state.ur = s_80_8;
        // D s_80_13: write-var uw <= s_80_9
        fn_state.uw = s_80_9;
        // N s_80_14: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_81_0: read-var pr:u8
        let s_81_0: bool = fn_state.pr;
        // C s_81_1: const #0u : u8
        let s_81_1: bool = false;
        // C s_81_2: const #0u : u8
        let s_81_2: bool = false;
        // C s_81_3: const #0u : u8
        let s_81_3: bool = false;
        // D s_81_4: create-product struct = ["s_81_0", "s_81_1", "s_81_2", "s_81_3"]
        let s_81_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_81_0,
            _1: s_81_1,
            _2: s_81_2,
            _3: s_81_3,
        };
        // D s_81_5: write-var ga#12577 <= s_81_4
        fn_state.ga_12577 = s_81_4;
        // D s_81_6: read-var ga#12577.0:struct
        let s_81_6: bool = fn_state.ga_12577._0;
        // D s_81_7: read-var ga#12577.1:struct
        let s_81_7: bool = fn_state.ga_12577._1;
        // D s_81_8: read-var ga#12577.2:struct
        let s_81_8: bool = fn_state.ga_12577._2;
        // D s_81_9: read-var ga#12577.3:struct
        let s_81_9: bool = fn_state.ga_12577._3;
        // D s_81_10: write-var pr <= s_81_6
        fn_state.pr = s_81_6;
        // D s_81_11: write-var pw <= s_81_7
        fn_state.pw = s_81_7;
        // D s_81_12: write-var ur <= s_81_8
        fn_state.ur = s_81_8;
        // D s_81_13: write-var uw <= s_81_9
        fn_state.uw = s_81_9;
        // N s_81_14: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_82_0: read-var ga#12568:u8
        let s_82_0: u8 = fn_state.ga_12568;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 2u16);
        // C s_82_2: const #1u : u8
        let s_82_2: u8 = 1;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 2u16);
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
        // C s_83_3: const #1u : u8
        let s_83_3: bool = true;
        // D s_83_4: create-product struct = ["s_83_0", "s_83_1", "s_83_2", "s_83_3"]
        let s_83_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_83_0,
            _1: s_83_1,
            _2: s_83_2,
            _3: s_83_3,
        };
        // D s_83_5: write-var ga#12570 <= s_83_4
        fn_state.ga_12570 = s_83_4;
        // D s_83_6: read-var ga#12570.0:struct
        let s_83_6: bool = fn_state.ga_12570._0;
        // D s_83_7: read-var ga#12570.1:struct
        let s_83_7: bool = fn_state.ga_12570._1;
        // D s_83_8: read-var ga#12570.2:struct
        let s_83_8: bool = fn_state.ga_12570._2;
        // D s_83_9: read-var ga#12570.3:struct
        let s_83_9: bool = fn_state.ga_12570._3;
        // D s_83_10: write-var pr <= s_83_6
        fn_state.pr = s_83_6;
        // D s_83_11: write-var pw <= s_83_7
        fn_state.pw = s_83_7;
        // D s_83_12: write-var ur <= s_83_8
        fn_state.ur = s_83_8;
        // D s_83_13: write-var uw <= s_83_9
        fn_state.uw = s_83_9;
        // N s_83_14: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_84_0: read-var ga#12568:u8
        let s_84_0: u8 = fn_state.ga_12568;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 2u16);
        // C s_84_2: const #2u : u8
        let s_84_2: u8 = 2;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 2u16);
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
        // D s_85_5: write-var ga#12571 <= s_85_4
        fn_state.ga_12571 = s_85_4;
        // D s_85_6: read-var ga#12571.0:struct
        let s_85_6: bool = fn_state.ga_12571._0;
        // D s_85_7: read-var ga#12571.1:struct
        let s_85_7: bool = fn_state.ga_12571._1;
        // D s_85_8: read-var ga#12571.2:struct
        let s_85_8: bool = fn_state.ga_12571._2;
        // D s_85_9: read-var ga#12571.3:struct
        let s_85_9: bool = fn_state.ga_12571._3;
        // D s_85_10: write-var pr <= s_85_6
        fn_state.pr = s_85_6;
        // D s_85_11: write-var pw <= s_85_7
        fn_state.pw = s_85_7;
        // D s_85_12: write-var ur <= s_85_8
        fn_state.ur = s_85_8;
        // D s_85_13: write-var uw <= s_85_9
        fn_state.uw = s_85_9;
        // N s_85_14: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_86_0: const #1u : u8
        let s_86_0: bool = true;
        // C s_86_1: const #0u : u8
        let s_86_1: bool = false;
        // C s_86_2: const #1u : u8
        let s_86_2: bool = true;
        // C s_86_3: const #0u : u8
        let s_86_3: bool = false;
        // D s_86_4: create-product struct = ["s_86_0", "s_86_1", "s_86_2", "s_86_3"]
        let s_86_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_86_0,
            _1: s_86_1,
            _2: s_86_2,
            _3: s_86_3,
        };
        // D s_86_5: write-var ga#12572 <= s_86_4
        fn_state.ga_12572 = s_86_4;
        // D s_86_6: read-var ga#12572.0:struct
        let s_86_6: bool = fn_state.ga_12572._0;
        // D s_86_7: read-var ga#12572.1:struct
        let s_86_7: bool = fn_state.ga_12572._1;
        // D s_86_8: read-var ga#12572.2:struct
        let s_86_8: bool = fn_state.ga_12572._2;
        // D s_86_9: read-var ga#12572.3:struct
        let s_86_9: bool = fn_state.ga_12572._3;
        // D s_86_10: write-var pr <= s_86_6
        fn_state.pr = s_86_6;
        // D s_86_11: write-var pw <= s_86_7
        fn_state.pw = s_86_7;
        // D s_86_12: write-var ur <= s_86_8
        fn_state.ur = s_86_8;
        // D s_86_13: write-var uw <= s_86_9
        fn_state.uw = s_86_9;
        // N s_86_14: jump b38
        return block_38(state, tracer, fn_state);
    }
}
