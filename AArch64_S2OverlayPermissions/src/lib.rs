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
use common::*;
pub fn AArch64_S2OverlayPermissions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    permissions: ProductTypebf05c51f33174538,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductType2fc9d3588999ac79 {
    #[derive(Default)]
    struct FunctionState {
        r: bool,
        ga_13884: ProductType74f83332f678823c,
        ga_13896: ProductType8b847afc727d5818,
        toplevel0: bool,
        ga_13891: ProductType74f83332f678823c,
        ga_13894: ProductType8b847afc727d5818,
        ga_13877: ProductType74f83332f678823c,
        ga_13892: ProductType74f83332f678823c,
        s2overlay_perms: ProductType2fc9d3588999ac79,
        ga_13895: ProductType8b847afc727d5818,
        toplevel1: bool,
        ga_13886: ProductType74f83332f678823c,
        w: bool,
        ga_13888: ProductType74f83332f678823c,
        ga_13889: ProductType74f83332f678823c,
        ga_13882: ProductType74f83332f678823c,
        x: bool,
        ga_13893: ProductType8b847afc727d5818,
        ga_13881: ProductType74f83332f678823c,
        ux: bool,
        ga_13883: ProductType74f83332f678823c,
        w_rcw: bool,
        ga_13879: ProductType74f83332f678823c,
        ga_13878: ProductType74f83332f678823c,
        w_mmu: bool,
        r_mmu: bool,
        px: bool,
        ga_13897: ProductType8b847afc727d5818,
        s2po: u8,
        ga_13890: ProductType74f83332f678823c,
        ga_13885: ProductType74f83332f678823c,
        ga_13887: ProductType74f83332f678823c,
        ga_13880: ProductType74f83332f678823c,
        r_rcw: bool,
        permissions: ProductTypebf05c51f33174538,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        permissions,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_0_0: read-var permissions.10:struct
        let s_0_0: u8 = fn_state.permissions._10;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // C s_0_4: const #4s : i
        let s_0_4: i128 = 4;
        // D s_0_5: cast zx s_0_3 -> i
        let s_0_5: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_6: mul s_0_4 s_0_5
        let s_0_6: i128 = ((s_0_4) * (s_0_5));
        // D s_0_7: cast reint s_0_6 -> i64
        let s_0_7: i64 = (s_0_6 as i64);
        // C s_0_8: const #101040u : u32
        let s_0_8: u32 = 101040;
        // D s_0_9: read-reg s_0_8:u64
        let s_0_9: u64 = {
            let value = state.read_register::<u64>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // C s_0_10: const #4s : i
        let s_0_10: i128 = 4;
        // D s_0_11: cast zx s_0_9 -> bv
        let s_0_11: Bits = Bits::new(s_0_9 as u128, 64u16);
        // D s_0_12: cast zx s_0_7 -> i
        let s_0_12: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_13: bit-extract s_0_11 s_0_12 s_0_10
        let s_0_13: Bits = (Bits::new(
            ((s_0_11) >> (s_0_12)).value(),
            u16::try_from(s_0_10).unwrap(),
        ));
        // D s_0_14: cast reint s_0_13 -> u8
        let s_0_14: u8 = (s_0_13.value() as u8);
        // D s_0_15: write-var s2po <= s_0_14
        fn_state.s2po = s_0_14;
        // D s_0_16: read-var s2po:u8
        let s_0_16: u8 = fn_state.s2po;
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 4u16);
        // C s_0_18: const #0u : u8
        let s_0_18: u8 = 0;
        // C s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 4u16);
        // D s_0_20: cmp-eq s_0_17 s_0_19
        let s_0_20: bool = ((s_0_17) == (s_0_19));
        // D s_0_21: not s_0_20
        let s_0_21: bool = !s_0_20;
        // N s_0_22: branch s_0_21 b13 b1
        if s_0_21 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // C s_1_1: const #0u : u8
        let s_1_1: bool = false;
        // C s_1_2: const #0u : u8
        let s_1_2: bool = false;
        // C s_1_3: const #0u : u8
        let s_1_3: bool = false;
        // C s_1_4: const #0u : u8
        let s_1_4: bool = false;
        // C s_1_5: const #0u : u8
        let s_1_5: bool = false;
        // D s_1_6: create-product struct = ["s_1_0", "s_1_1", "s_1_2", "s_1_3", "s_1_4", "s_1_5"]
        let s_1_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_1_0,
            _1: s_1_1,
            _2: s_1_2,
            _3: s_1_3,
            _4: s_1_4,
            _5: s_1_5,
        };
        // D s_1_7: write-var ga#13877 <= s_1_6
        fn_state.ga_13877 = s_1_6;
        // D s_1_8: read-var ga#13877.0:struct
        let s_1_8: bool = fn_state.ga_13877._0;
        // D s_1_9: read-var ga#13877.1:struct
        let s_1_9: bool = fn_state.ga_13877._1;
        // D s_1_10: read-var ga#13877.2:struct
        let s_1_10: bool = fn_state.ga_13877._2;
        // D s_1_11: read-var ga#13877.3:struct
        let s_1_11: bool = fn_state.ga_13877._3;
        // D s_1_12: read-var ga#13877.4:struct
        let s_1_12: bool = fn_state.ga_13877._4;
        // D s_1_13: read-var ga#13877.5:struct
        let s_1_13: bool = fn_state.ga_13877._5;
        // D s_1_14: write-var r <= s_1_8
        fn_state.r = s_1_8;
        // D s_1_15: write-var w <= s_1_9
        fn_state.w = s_1_9;
        // D s_1_16: write-var px <= s_1_10
        fn_state.px = s_1_10;
        // D s_1_17: write-var ux <= s_1_11
        fn_state.ux = s_1_11;
        // D s_1_18: write-var w_rcw <= s_1_12
        fn_state.w_rcw = s_1_12;
        // D s_1_19: write-var w_mmu <= s_1_13
        fn_state.w_mmu = s_1_13;
        // N s_1_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_2_0: read-var accdesc.8:struct
        let s_2_0: u8 = fn_state.accdesc._8;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #448u : u32
        let s_2_2: u32 = 448;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // N s_2_6: branch s_2_5 b12 b3
        if s_2_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_3_0: read-var px:u8
        let s_3_0: bool = fn_state.px;
        // D s_3_1: write-var x <= s_3_0
        fn_state.x = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_4_0: read-var r:u8
        let s_4_0: bool = fn_state.r;
        // D s_4_1: read-var r:u8
        let s_4_1: bool = fn_state.r;
        // D s_4_2: create-product struct = ["s_4_0", "s_4_1"]
        let s_4_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_4_0,
            _1: s_4_1,
        };
        // D s_4_3: write-var ga#13893 <= s_4_2
        fn_state.ga_13893 = s_4_2;
        // D s_4_4: read-var ga#13893.0:struct
        let s_4_4: bool = fn_state.ga_13893._0;
        // D s_4_5: read-var ga#13893.1:struct
        let s_4_5: bool = fn_state.ga_13893._1;
        // D s_4_6: write-var r_rcw <= s_4_4
        fn_state.r_rcw = s_4_4;
        // D s_4_7: write-var r_mmu <= s_4_5
        fn_state.r_mmu = s_4_5;
        // D s_4_8: read-var s2po:u8
        let s_4_8: u8 = fn_state.s2po;
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 4u16);
        // C s_4_10: const #6u : u8
        let s_4_10: u8 = 6;
        // C s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 4u16);
        // D s_4_12: cmp-eq s_4_9 s_4_11
        let s_4_12: bool = ((s_4_9) == (s_4_11));
        // D s_4_13: not s_4_12
        let s_4_13: bool = !s_4_12;
        // N s_4_14: branch s_4_13 b7 b5
        if s_4_13 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // C s_5_1: const #0u : u8
        let s_5_1: bool = false;
        // D s_5_2: create-product struct = ["s_5_0", "s_5_1"]
        let s_5_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_5_0,
            _1: s_5_1,
        };
        // D s_5_3: write-var ga#13894 <= s_5_2
        fn_state.ga_13894 = s_5_2;
        // D s_5_4: read-var ga#13894.0:struct
        let s_5_4: bool = fn_state.ga_13894._0;
        // D s_5_5: read-var ga#13894.1:struct
        let s_5_5: bool = fn_state.ga_13894._1;
        // D s_5_6: write-var toplevel0 <= s_5_4
        fn_state.toplevel0 = s_5_4;
        // D s_5_7: write-var toplevel1 <= s_5_5
        fn_state.toplevel1 = s_5_5;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_6_0: read-var r:u8
        let s_6_0: bool = fn_state.r;
        // D s_6_1: write-var s2overlay_perms.0 <= s_6_0
        fn_state.s2overlay_perms._0 = s_6_0;
        // D s_6_2: read-var w:u8
        let s_6_2: bool = fn_state.w;
        // D s_6_3: write-var s2overlay_perms.4 <= s_6_2
        fn_state.s2overlay_perms._4 = s_6_2;
        // D s_6_4: read-var x:u8
        let s_6_4: bool = fn_state.x;
        // D s_6_5: write-var s2overlay_perms.7 <= s_6_4
        fn_state.s2overlay_perms._7 = s_6_4;
        // D s_6_6: read-var r_rcw:u8
        let s_6_6: bool = fn_state.r_rcw;
        // D s_6_7: write-var s2overlay_perms.2 <= s_6_6
        fn_state.s2overlay_perms._2 = s_6_6;
        // D s_6_8: read-var w_rcw:u8
        let s_6_8: bool = fn_state.w_rcw;
        // D s_6_9: write-var s2overlay_perms.6 <= s_6_8
        fn_state.s2overlay_perms._6 = s_6_8;
        // D s_6_10: read-var r_mmu:u8
        let s_6_10: bool = fn_state.r_mmu;
        // D s_6_11: write-var s2overlay_perms.1 <= s_6_10
        fn_state.s2overlay_perms._1 = s_6_10;
        // D s_6_12: read-var w_mmu:u8
        let s_6_12: bool = fn_state.w_mmu;
        // D s_6_13: write-var s2overlay_perms.5 <= s_6_12
        fn_state.s2overlay_perms._5 = s_6_12;
        // D s_6_14: read-var toplevel0:u8
        let s_6_14: bool = fn_state.toplevel0;
        // D s_6_15: write-var s2overlay_perms.11 <= s_6_14
        fn_state.s2overlay_perms._11 = s_6_14;
        // D s_6_16: read-var toplevel1:u8
        let s_6_16: bool = fn_state.toplevel1;
        // D s_6_17: write-var s2overlay_perms.12 <= s_6_16
        fn_state.s2overlay_perms._12 = s_6_16;
        // D s_6_18: read-var s2overlay_perms:struct
        let s_6_18: ProductType2fc9d3588999ac79 = fn_state.s2overlay_perms;
        // N s_6_19: return s_6_18
        return s_6_18;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_7_0: read-var s2po:u8
        let s_7_0: u8 = fn_state.s2po;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 4u16);
        // C s_7_2: const #3u : u8
        let s_7_2: u8 = 3;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 4u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // C s_8_1: const #1u : u8
        let s_8_1: bool = true;
        // D s_8_2: create-product struct = ["s_8_0", "s_8_1"]
        let s_8_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_8_0,
            _1: s_8_1,
        };
        // D s_8_3: write-var ga#13895 <= s_8_2
        fn_state.ga_13895 = s_8_2;
        // D s_8_4: read-var ga#13895.0:struct
        let s_8_4: bool = fn_state.ga_13895._0;
        // D s_8_5: read-var ga#13895.1:struct
        let s_8_5: bool = fn_state.ga_13895._1;
        // D s_8_6: write-var toplevel0 <= s_8_4
        fn_state.toplevel0 = s_8_4;
        // D s_8_7: write-var toplevel1 <= s_8_5
        fn_state.toplevel1 = s_8_5;
        // N s_8_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_9_0: read-var s2po:u8
        let s_9_0: u8 = fn_state.s2po;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 4u16);
        // C s_9_2: const #7u : u8
        let s_9_2: u8 = 7;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 4u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // C s_10_1: const #1u : u8
        let s_10_1: bool = true;
        // D s_10_2: create-product struct = ["s_10_0", "s_10_1"]
        let s_10_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_10_0,
            _1: s_10_1,
        };
        // D s_10_3: write-var ga#13896 <= s_10_2
        fn_state.ga_13896 = s_10_2;
        // D s_10_4: read-var ga#13896.0:struct
        let s_10_4: bool = fn_state.ga_13896._0;
        // D s_10_5: read-var ga#13896.1:struct
        let s_10_5: bool = fn_state.ga_13896._1;
        // D s_10_6: write-var toplevel0 <= s_10_4
        fn_state.toplevel0 = s_10_4;
        // D s_10_7: write-var toplevel1 <= s_10_5
        fn_state.toplevel1 = s_10_5;
        // N s_10_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // C s_11_1: const #0u : u8
        let s_11_1: bool = false;
        // D s_11_2: create-product struct = ["s_11_0", "s_11_1"]
        let s_11_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_11_0,
            _1: s_11_1,
        };
        // D s_11_3: write-var ga#13897 <= s_11_2
        fn_state.ga_13897 = s_11_2;
        // D s_11_4: read-var ga#13897.0:struct
        let s_11_4: bool = fn_state.ga_13897._0;
        // D s_11_5: read-var ga#13897.1:struct
        let s_11_5: bool = fn_state.ga_13897._1;
        // D s_11_6: write-var toplevel0 <= s_11_4
        fn_state.toplevel0 = s_11_4;
        // D s_11_7: write-var toplevel1 <= s_11_5
        fn_state.toplevel1 = s_11_5;
        // N s_11_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_12_0: read-var ux:u8
        let s_12_0: bool = fn_state.ux;
        // D s_12_1: write-var x <= s_12_0
        fn_state.x = s_12_0;
        // N s_12_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_13_0: read-var s2po:u8
        let s_13_0: u8 = fn_state.s2po;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 4u16);
        // C s_13_2: const #1u : u8
        let s_13_2: u8 = 1;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 4u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b15 b14
        if s_13_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // C s_14_1: const #0u : u8
        let s_14_1: bool = false;
        // C s_14_2: const #0u : u8
        let s_14_2: bool = false;
        // C s_14_3: const #0u : u8
        let s_14_3: bool = false;
        // C s_14_4: const #0u : u8
        let s_14_4: bool = false;
        // C s_14_5: const #0u : u8
        let s_14_5: bool = false;
        // D s_14_6: create-product struct = ["s_14_0", "s_14_1", "s_14_2", "s_14_3", "s_14_4", "s_14_5"]
        let s_14_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_14_0,
            _1: s_14_1,
            _2: s_14_2,
            _3: s_14_3,
            _4: s_14_4,
            _5: s_14_5,
        };
        // D s_14_7: write-var ga#13878 <= s_14_6
        fn_state.ga_13878 = s_14_6;
        // D s_14_8: read-var ga#13878.0:struct
        let s_14_8: bool = fn_state.ga_13878._0;
        // D s_14_9: read-var ga#13878.1:struct
        let s_14_9: bool = fn_state.ga_13878._1;
        // D s_14_10: read-var ga#13878.2:struct
        let s_14_10: bool = fn_state.ga_13878._2;
        // D s_14_11: read-var ga#13878.3:struct
        let s_14_11: bool = fn_state.ga_13878._3;
        // D s_14_12: read-var ga#13878.4:struct
        let s_14_12: bool = fn_state.ga_13878._4;
        // D s_14_13: read-var ga#13878.5:struct
        let s_14_13: bool = fn_state.ga_13878._5;
        // D s_14_14: write-var r <= s_14_8
        fn_state.r = s_14_8;
        // D s_14_15: write-var w <= s_14_9
        fn_state.w = s_14_9;
        // D s_14_16: write-var px <= s_14_10
        fn_state.px = s_14_10;
        // D s_14_17: write-var ux <= s_14_11
        fn_state.ux = s_14_11;
        // D s_14_18: write-var w_rcw <= s_14_12
        fn_state.w_rcw = s_14_12;
        // D s_14_19: write-var w_mmu <= s_14_13
        fn_state.w_mmu = s_14_13;
        // N s_14_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_15_0: read-var s2po:u8
        let s_15_0: u8 = fn_state.s2po;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 4u16);
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 4u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b17 b16
        if s_15_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // C s_16_1: const #0u : u8
        let s_16_1: bool = false;
        // C s_16_2: const #0u : u8
        let s_16_2: bool = false;
        // C s_16_3: const #0u : u8
        let s_16_3: bool = false;
        // C s_16_4: const #1u : u8
        let s_16_4: bool = true;
        // C s_16_5: const #1u : u8
        let s_16_5: bool = true;
        // D s_16_6: create-product struct = ["s_16_0", "s_16_1", "s_16_2", "s_16_3", "s_16_4", "s_16_5"]
        let s_16_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_16_0,
            _1: s_16_1,
            _2: s_16_2,
            _3: s_16_3,
            _4: s_16_4,
            _5: s_16_5,
        };
        // D s_16_7: write-var ga#13879 <= s_16_6
        fn_state.ga_13879 = s_16_6;
        // D s_16_8: read-var ga#13879.0:struct
        let s_16_8: bool = fn_state.ga_13879._0;
        // D s_16_9: read-var ga#13879.1:struct
        let s_16_9: bool = fn_state.ga_13879._1;
        // D s_16_10: read-var ga#13879.2:struct
        let s_16_10: bool = fn_state.ga_13879._2;
        // D s_16_11: read-var ga#13879.3:struct
        let s_16_11: bool = fn_state.ga_13879._3;
        // D s_16_12: read-var ga#13879.4:struct
        let s_16_12: bool = fn_state.ga_13879._4;
        // D s_16_13: read-var ga#13879.5:struct
        let s_16_13: bool = fn_state.ga_13879._5;
        // D s_16_14: write-var r <= s_16_8
        fn_state.r = s_16_8;
        // D s_16_15: write-var w <= s_16_9
        fn_state.w = s_16_9;
        // D s_16_16: write-var px <= s_16_10
        fn_state.px = s_16_10;
        // D s_16_17: write-var ux <= s_16_11
        fn_state.ux = s_16_11;
        // D s_16_18: write-var w_rcw <= s_16_12
        fn_state.w_rcw = s_16_12;
        // D s_16_19: write-var w_mmu <= s_16_13
        fn_state.w_mmu = s_16_13;
        // N s_16_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_17_0: read-var s2po:u8
        let s_17_0: u8 = fn_state.s2po;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 4u16);
        // C s_17_2: const #3u : u8
        let s_17_2: u8 = 3;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 4u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b19 b18
        if s_17_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // C s_18_1: const #0u : u8
        let s_18_1: bool = false;
        // C s_18_2: const #0u : u8
        let s_18_2: bool = false;
        // C s_18_3: const #0u : u8
        let s_18_3: bool = false;
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: const #1u : u8
        let s_18_5: bool = true;
        // D s_18_6: create-product struct = ["s_18_0", "s_18_1", "s_18_2", "s_18_3", "s_18_4", "s_18_5"]
        let s_18_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_18_0,
            _1: s_18_1,
            _2: s_18_2,
            _3: s_18_3,
            _4: s_18_4,
            _5: s_18_5,
        };
        // D s_18_7: write-var ga#13880 <= s_18_6
        fn_state.ga_13880 = s_18_6;
        // D s_18_8: read-var ga#13880.0:struct
        let s_18_8: bool = fn_state.ga_13880._0;
        // D s_18_9: read-var ga#13880.1:struct
        let s_18_9: bool = fn_state.ga_13880._1;
        // D s_18_10: read-var ga#13880.2:struct
        let s_18_10: bool = fn_state.ga_13880._2;
        // D s_18_11: read-var ga#13880.3:struct
        let s_18_11: bool = fn_state.ga_13880._3;
        // D s_18_12: read-var ga#13880.4:struct
        let s_18_12: bool = fn_state.ga_13880._4;
        // D s_18_13: read-var ga#13880.5:struct
        let s_18_13: bool = fn_state.ga_13880._5;
        // D s_18_14: write-var r <= s_18_8
        fn_state.r = s_18_8;
        // D s_18_15: write-var w <= s_18_9
        fn_state.w = s_18_9;
        // D s_18_16: write-var px <= s_18_10
        fn_state.px = s_18_10;
        // D s_18_17: write-var ux <= s_18_11
        fn_state.ux = s_18_11;
        // D s_18_18: write-var w_rcw <= s_18_12
        fn_state.w_rcw = s_18_12;
        // D s_18_19: write-var w_mmu <= s_18_13
        fn_state.w_mmu = s_18_13;
        // N s_18_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_19_0: read-var s2po:u8
        let s_19_0: u8 = fn_state.s2po;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 4u16);
        // C s_19_2: const #4u : u8
        let s_19_2: u8 = 4;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 4u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: not s_19_4
        let s_19_5: bool = !s_19_4;
        // N s_19_6: branch s_19_5 b21 b20
        if s_19_5 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // C s_20_1: const #1u : u8
        let s_20_1: bool = true;
        // C s_20_2: const #0u : u8
        let s_20_2: bool = false;
        // C s_20_3: const #0u : u8
        let s_20_3: bool = false;
        // C s_20_4: const #0u : u8
        let s_20_4: bool = false;
        // C s_20_5: const #0u : u8
        let s_20_5: bool = false;
        // D s_20_6: create-product struct = ["s_20_0", "s_20_1", "s_20_2", "s_20_3", "s_20_4", "s_20_5"]
        let s_20_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_20_0,
            _1: s_20_1,
            _2: s_20_2,
            _3: s_20_3,
            _4: s_20_4,
            _5: s_20_5,
        };
        // D s_20_7: write-var ga#13881 <= s_20_6
        fn_state.ga_13881 = s_20_6;
        // D s_20_8: read-var ga#13881.0:struct
        let s_20_8: bool = fn_state.ga_13881._0;
        // D s_20_9: read-var ga#13881.1:struct
        let s_20_9: bool = fn_state.ga_13881._1;
        // D s_20_10: read-var ga#13881.2:struct
        let s_20_10: bool = fn_state.ga_13881._2;
        // D s_20_11: read-var ga#13881.3:struct
        let s_20_11: bool = fn_state.ga_13881._3;
        // D s_20_12: read-var ga#13881.4:struct
        let s_20_12: bool = fn_state.ga_13881._4;
        // D s_20_13: read-var ga#13881.5:struct
        let s_20_13: bool = fn_state.ga_13881._5;
        // D s_20_14: write-var r <= s_20_8
        fn_state.r = s_20_8;
        // D s_20_15: write-var w <= s_20_9
        fn_state.w = s_20_9;
        // D s_20_16: write-var px <= s_20_10
        fn_state.px = s_20_10;
        // D s_20_17: write-var ux <= s_20_11
        fn_state.ux = s_20_11;
        // D s_20_18: write-var w_rcw <= s_20_12
        fn_state.w_rcw = s_20_12;
        // D s_20_19: write-var w_mmu <= s_20_13
        fn_state.w_mmu = s_20_13;
        // N s_20_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_21_0: read-var s2po:u8
        let s_21_0: u8 = fn_state.s2po;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 4u16);
        // C s_21_2: const #5u : u8
        let s_21_2: u8 = 5;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 4u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: not s_21_4
        let s_21_5: bool = !s_21_4;
        // N s_21_6: branch s_21_5 b23 b22
        if s_21_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // C s_22_1: const #0u : u8
        let s_22_1: bool = false;
        // C s_22_2: const #0u : u8
        let s_22_2: bool = false;
        // C s_22_3: const #0u : u8
        let s_22_3: bool = false;
        // C s_22_4: const #0u : u8
        let s_22_4: bool = false;
        // C s_22_5: const #0u : u8
        let s_22_5: bool = false;
        // D s_22_6: create-product struct = ["s_22_0", "s_22_1", "s_22_2", "s_22_3", "s_22_4", "s_22_5"]
        let s_22_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_22_0,
            _1: s_22_1,
            _2: s_22_2,
            _3: s_22_3,
            _4: s_22_4,
            _5: s_22_5,
        };
        // D s_22_7: write-var ga#13882 <= s_22_6
        fn_state.ga_13882 = s_22_6;
        // D s_22_8: read-var ga#13882.0:struct
        let s_22_8: bool = fn_state.ga_13882._0;
        // D s_22_9: read-var ga#13882.1:struct
        let s_22_9: bool = fn_state.ga_13882._1;
        // D s_22_10: read-var ga#13882.2:struct
        let s_22_10: bool = fn_state.ga_13882._2;
        // D s_22_11: read-var ga#13882.3:struct
        let s_22_11: bool = fn_state.ga_13882._3;
        // D s_22_12: read-var ga#13882.4:struct
        let s_22_12: bool = fn_state.ga_13882._4;
        // D s_22_13: read-var ga#13882.5:struct
        let s_22_13: bool = fn_state.ga_13882._5;
        // D s_22_14: write-var r <= s_22_8
        fn_state.r = s_22_8;
        // D s_22_15: write-var w <= s_22_9
        fn_state.w = s_22_9;
        // D s_22_16: write-var px <= s_22_10
        fn_state.px = s_22_10;
        // D s_22_17: write-var ux <= s_22_11
        fn_state.ux = s_22_11;
        // D s_22_18: write-var w_rcw <= s_22_12
        fn_state.w_rcw = s_22_12;
        // D s_22_19: write-var w_mmu <= s_22_13
        fn_state.w_mmu = s_22_13;
        // N s_22_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_23_0: read-var s2po:u8
        let s_23_0: u8 = fn_state.s2po;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 4u16);
        // C s_23_2: const #6u : u8
        let s_23_2: u8 = 6;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 4u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: not s_23_4
        let s_23_5: bool = !s_23_4;
        // N s_23_6: branch s_23_5 b25 b24
        if s_23_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // C s_24_1: const #0u : u8
        let s_24_1: bool = false;
        // C s_24_2: const #0u : u8
        let s_24_2: bool = false;
        // C s_24_3: const #0u : u8
        let s_24_3: bool = false;
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: const #1u : u8
        let s_24_5: bool = true;
        // D s_24_6: create-product struct = ["s_24_0", "s_24_1", "s_24_2", "s_24_3", "s_24_4", "s_24_5"]
        let s_24_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_24_0,
            _1: s_24_1,
            _2: s_24_2,
            _3: s_24_3,
            _4: s_24_4,
            _5: s_24_5,
        };
        // D s_24_7: write-var ga#13883 <= s_24_6
        fn_state.ga_13883 = s_24_6;
        // D s_24_8: read-var ga#13883.0:struct
        let s_24_8: bool = fn_state.ga_13883._0;
        // D s_24_9: read-var ga#13883.1:struct
        let s_24_9: bool = fn_state.ga_13883._1;
        // D s_24_10: read-var ga#13883.2:struct
        let s_24_10: bool = fn_state.ga_13883._2;
        // D s_24_11: read-var ga#13883.3:struct
        let s_24_11: bool = fn_state.ga_13883._3;
        // D s_24_12: read-var ga#13883.4:struct
        let s_24_12: bool = fn_state.ga_13883._4;
        // D s_24_13: read-var ga#13883.5:struct
        let s_24_13: bool = fn_state.ga_13883._5;
        // D s_24_14: write-var r <= s_24_8
        fn_state.r = s_24_8;
        // D s_24_15: write-var w <= s_24_9
        fn_state.w = s_24_9;
        // D s_24_16: write-var px <= s_24_10
        fn_state.px = s_24_10;
        // D s_24_17: write-var ux <= s_24_11
        fn_state.ux = s_24_11;
        // D s_24_18: write-var w_rcw <= s_24_12
        fn_state.w_rcw = s_24_12;
        // D s_24_19: write-var w_mmu <= s_24_13
        fn_state.w_mmu = s_24_13;
        // N s_24_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_25_0: read-var s2po:u8
        let s_25_0: u8 = fn_state.s2po;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 4u16);
        // C s_25_2: const #7u : u8
        let s_25_2: u8 = 7;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 4u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: not s_25_4
        let s_25_5: bool = !s_25_4;
        // N s_25_6: branch s_25_5 b27 b26
        if s_25_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // C s_26_1: const #0u : u8
        let s_26_1: bool = false;
        // C s_26_2: const #0u : u8
        let s_26_2: bool = false;
        // C s_26_3: const #0u : u8
        let s_26_3: bool = false;
        // C s_26_4: const #1u : u8
        let s_26_4: bool = true;
        // C s_26_5: const #1u : u8
        let s_26_5: bool = true;
        // D s_26_6: create-product struct = ["s_26_0", "s_26_1", "s_26_2", "s_26_3", "s_26_4", "s_26_5"]
        let s_26_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_26_0,
            _1: s_26_1,
            _2: s_26_2,
            _3: s_26_3,
            _4: s_26_4,
            _5: s_26_5,
        };
        // D s_26_7: write-var ga#13884 <= s_26_6
        fn_state.ga_13884 = s_26_6;
        // D s_26_8: read-var ga#13884.0:struct
        let s_26_8: bool = fn_state.ga_13884._0;
        // D s_26_9: read-var ga#13884.1:struct
        let s_26_9: bool = fn_state.ga_13884._1;
        // D s_26_10: read-var ga#13884.2:struct
        let s_26_10: bool = fn_state.ga_13884._2;
        // D s_26_11: read-var ga#13884.3:struct
        let s_26_11: bool = fn_state.ga_13884._3;
        // D s_26_12: read-var ga#13884.4:struct
        let s_26_12: bool = fn_state.ga_13884._4;
        // D s_26_13: read-var ga#13884.5:struct
        let s_26_13: bool = fn_state.ga_13884._5;
        // D s_26_14: write-var r <= s_26_8
        fn_state.r = s_26_8;
        // D s_26_15: write-var w <= s_26_9
        fn_state.w = s_26_9;
        // D s_26_16: write-var px <= s_26_10
        fn_state.px = s_26_10;
        // D s_26_17: write-var ux <= s_26_11
        fn_state.ux = s_26_11;
        // D s_26_18: write-var w_rcw <= s_26_12
        fn_state.w_rcw = s_26_12;
        // D s_26_19: write-var w_mmu <= s_26_13
        fn_state.w_mmu = s_26_13;
        // N s_26_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_27_0: read-var s2po:u8
        let s_27_0: u8 = fn_state.s2po;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 4u16);
        // C s_27_2: const #8u : u8
        let s_27_2: u8 = 8;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 4u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: not s_27_4
        let s_27_5: bool = !s_27_4;
        // N s_27_6: branch s_27_5 b29 b28
        if s_27_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // C s_28_1: const #0u : u8
        let s_28_1: bool = false;
        // C s_28_2: const #0u : u8
        let s_28_2: bool = false;
        // C s_28_3: const #0u : u8
        let s_28_3: bool = false;
        // C s_28_4: const #0u : u8
        let s_28_4: bool = false;
        // C s_28_5: const #0u : u8
        let s_28_5: bool = false;
        // D s_28_6: create-product struct = ["s_28_0", "s_28_1", "s_28_2", "s_28_3", "s_28_4", "s_28_5"]
        let s_28_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_28_0,
            _1: s_28_1,
            _2: s_28_2,
            _3: s_28_3,
            _4: s_28_4,
            _5: s_28_5,
        };
        // D s_28_7: write-var ga#13885 <= s_28_6
        fn_state.ga_13885 = s_28_6;
        // D s_28_8: read-var ga#13885.0:struct
        let s_28_8: bool = fn_state.ga_13885._0;
        // D s_28_9: read-var ga#13885.1:struct
        let s_28_9: bool = fn_state.ga_13885._1;
        // D s_28_10: read-var ga#13885.2:struct
        let s_28_10: bool = fn_state.ga_13885._2;
        // D s_28_11: read-var ga#13885.3:struct
        let s_28_11: bool = fn_state.ga_13885._3;
        // D s_28_12: read-var ga#13885.4:struct
        let s_28_12: bool = fn_state.ga_13885._4;
        // D s_28_13: read-var ga#13885.5:struct
        let s_28_13: bool = fn_state.ga_13885._5;
        // D s_28_14: write-var r <= s_28_8
        fn_state.r = s_28_8;
        // D s_28_15: write-var w <= s_28_9
        fn_state.w = s_28_9;
        // D s_28_16: write-var px <= s_28_10
        fn_state.px = s_28_10;
        // D s_28_17: write-var ux <= s_28_11
        fn_state.ux = s_28_11;
        // D s_28_18: write-var w_rcw <= s_28_12
        fn_state.w_rcw = s_28_12;
        // D s_28_19: write-var w_mmu <= s_28_13
        fn_state.w_mmu = s_28_13;
        // N s_28_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_29_0: read-var s2po:u8
        let s_29_0: u8 = fn_state.s2po;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 4u16);
        // C s_29_2: const #9u : u8
        let s_29_2: u8 = 9;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 4u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: not s_29_4
        let s_29_5: bool = !s_29_4;
        // N s_29_6: branch s_29_5 b31 b30
        if s_29_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // C s_30_1: const #0u : u8
        let s_30_1: bool = false;
        // C s_30_2: const #0u : u8
        let s_30_2: bool = false;
        // C s_30_3: const #1u : u8
        let s_30_3: bool = true;
        // C s_30_4: const #0u : u8
        let s_30_4: bool = false;
        // C s_30_5: const #0u : u8
        let s_30_5: bool = false;
        // D s_30_6: create-product struct = ["s_30_0", "s_30_1", "s_30_2", "s_30_3", "s_30_4", "s_30_5"]
        let s_30_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_30_0,
            _1: s_30_1,
            _2: s_30_2,
            _3: s_30_3,
            _4: s_30_4,
            _5: s_30_5,
        };
        // D s_30_7: write-var ga#13886 <= s_30_6
        fn_state.ga_13886 = s_30_6;
        // D s_30_8: read-var ga#13886.0:struct
        let s_30_8: bool = fn_state.ga_13886._0;
        // D s_30_9: read-var ga#13886.1:struct
        let s_30_9: bool = fn_state.ga_13886._1;
        // D s_30_10: read-var ga#13886.2:struct
        let s_30_10: bool = fn_state.ga_13886._2;
        // D s_30_11: read-var ga#13886.3:struct
        let s_30_11: bool = fn_state.ga_13886._3;
        // D s_30_12: read-var ga#13886.4:struct
        let s_30_12: bool = fn_state.ga_13886._4;
        // D s_30_13: read-var ga#13886.5:struct
        let s_30_13: bool = fn_state.ga_13886._5;
        // D s_30_14: write-var r <= s_30_8
        fn_state.r = s_30_8;
        // D s_30_15: write-var w <= s_30_9
        fn_state.w = s_30_9;
        // D s_30_16: write-var px <= s_30_10
        fn_state.px = s_30_10;
        // D s_30_17: write-var ux <= s_30_11
        fn_state.ux = s_30_11;
        // D s_30_18: write-var w_rcw <= s_30_12
        fn_state.w_rcw = s_30_12;
        // D s_30_19: write-var w_mmu <= s_30_13
        fn_state.w_mmu = s_30_13;
        // N s_30_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_31_0: read-var s2po:u8
        let s_31_0: u8 = fn_state.s2po;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 4u16);
        // C s_31_2: const #10u : u8
        let s_31_2: u8 = 10;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 4u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: not s_31_4
        let s_31_5: bool = !s_31_4;
        // N s_31_6: branch s_31_5 b33 b32
        if s_31_5 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // C s_32_1: const #0u : u8
        let s_32_1: bool = false;
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: const #0u : u8
        let s_32_3: bool = false;
        // C s_32_4: const #0u : u8
        let s_32_4: bool = false;
        // C s_32_5: const #0u : u8
        let s_32_5: bool = false;
        // D s_32_6: create-product struct = ["s_32_0", "s_32_1", "s_32_2", "s_32_3", "s_32_4", "s_32_5"]
        let s_32_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_32_0,
            _1: s_32_1,
            _2: s_32_2,
            _3: s_32_3,
            _4: s_32_4,
            _5: s_32_5,
        };
        // D s_32_7: write-var ga#13887 <= s_32_6
        fn_state.ga_13887 = s_32_6;
        // D s_32_8: read-var ga#13887.0:struct
        let s_32_8: bool = fn_state.ga_13887._0;
        // D s_32_9: read-var ga#13887.1:struct
        let s_32_9: bool = fn_state.ga_13887._1;
        // D s_32_10: read-var ga#13887.2:struct
        let s_32_10: bool = fn_state.ga_13887._2;
        // D s_32_11: read-var ga#13887.3:struct
        let s_32_11: bool = fn_state.ga_13887._3;
        // D s_32_12: read-var ga#13887.4:struct
        let s_32_12: bool = fn_state.ga_13887._4;
        // D s_32_13: read-var ga#13887.5:struct
        let s_32_13: bool = fn_state.ga_13887._5;
        // D s_32_14: write-var r <= s_32_8
        fn_state.r = s_32_8;
        // D s_32_15: write-var w <= s_32_9
        fn_state.w = s_32_9;
        // D s_32_16: write-var px <= s_32_10
        fn_state.px = s_32_10;
        // D s_32_17: write-var ux <= s_32_11
        fn_state.ux = s_32_11;
        // D s_32_18: write-var w_rcw <= s_32_12
        fn_state.w_rcw = s_32_12;
        // D s_32_19: write-var w_mmu <= s_32_13
        fn_state.w_mmu = s_32_13;
        // N s_32_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_33_0: read-var s2po:u8
        let s_33_0: u8 = fn_state.s2po;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 4u16);
        // C s_33_2: const #11u : u8
        let s_33_2: u8 = 11;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 4u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: not s_33_4
        let s_33_5: bool = !s_33_4;
        // N s_33_6: branch s_33_5 b35 b34
        if s_33_5 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // C s_34_1: const #0u : u8
        let s_34_1: bool = false;
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: const #1u : u8
        let s_34_3: bool = true;
        // C s_34_4: const #0u : u8
        let s_34_4: bool = false;
        // C s_34_5: const #0u : u8
        let s_34_5: bool = false;
        // D s_34_6: create-product struct = ["s_34_0", "s_34_1", "s_34_2", "s_34_3", "s_34_4", "s_34_5"]
        let s_34_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_34_0,
            _1: s_34_1,
            _2: s_34_2,
            _3: s_34_3,
            _4: s_34_4,
            _5: s_34_5,
        };
        // D s_34_7: write-var ga#13888 <= s_34_6
        fn_state.ga_13888 = s_34_6;
        // D s_34_8: read-var ga#13888.0:struct
        let s_34_8: bool = fn_state.ga_13888._0;
        // D s_34_9: read-var ga#13888.1:struct
        let s_34_9: bool = fn_state.ga_13888._1;
        // D s_34_10: read-var ga#13888.2:struct
        let s_34_10: bool = fn_state.ga_13888._2;
        // D s_34_11: read-var ga#13888.3:struct
        let s_34_11: bool = fn_state.ga_13888._3;
        // D s_34_12: read-var ga#13888.4:struct
        let s_34_12: bool = fn_state.ga_13888._4;
        // D s_34_13: read-var ga#13888.5:struct
        let s_34_13: bool = fn_state.ga_13888._5;
        // D s_34_14: write-var r <= s_34_8
        fn_state.r = s_34_8;
        // D s_34_15: write-var w <= s_34_9
        fn_state.w = s_34_9;
        // D s_34_16: write-var px <= s_34_10
        fn_state.px = s_34_10;
        // D s_34_17: write-var ux <= s_34_11
        fn_state.ux = s_34_11;
        // D s_34_18: write-var w_rcw <= s_34_12
        fn_state.w_rcw = s_34_12;
        // D s_34_19: write-var w_mmu <= s_34_13
        fn_state.w_mmu = s_34_13;
        // N s_34_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_35_0: read-var s2po:u8
        let s_35_0: u8 = fn_state.s2po;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 4u16);
        // C s_35_2: const #12u : u8
        let s_35_2: u8 = 12;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 4u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // D s_35_5: not s_35_4
        let s_35_5: bool = !s_35_4;
        // N s_35_6: branch s_35_5 b37 b36
        if s_35_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // C s_36_1: const #1u : u8
        let s_36_1: bool = true;
        // C s_36_2: const #0u : u8
        let s_36_2: bool = false;
        // C s_36_3: const #0u : u8
        let s_36_3: bool = false;
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: const #1u : u8
        let s_36_5: bool = true;
        // D s_36_6: create-product struct = ["s_36_0", "s_36_1", "s_36_2", "s_36_3", "s_36_4", "s_36_5"]
        let s_36_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_36_0,
            _1: s_36_1,
            _2: s_36_2,
            _3: s_36_3,
            _4: s_36_4,
            _5: s_36_5,
        };
        // D s_36_7: write-var ga#13889 <= s_36_6
        fn_state.ga_13889 = s_36_6;
        // D s_36_8: read-var ga#13889.0:struct
        let s_36_8: bool = fn_state.ga_13889._0;
        // D s_36_9: read-var ga#13889.1:struct
        let s_36_9: bool = fn_state.ga_13889._1;
        // D s_36_10: read-var ga#13889.2:struct
        let s_36_10: bool = fn_state.ga_13889._2;
        // D s_36_11: read-var ga#13889.3:struct
        let s_36_11: bool = fn_state.ga_13889._3;
        // D s_36_12: read-var ga#13889.4:struct
        let s_36_12: bool = fn_state.ga_13889._4;
        // D s_36_13: read-var ga#13889.5:struct
        let s_36_13: bool = fn_state.ga_13889._5;
        // D s_36_14: write-var r <= s_36_8
        fn_state.r = s_36_8;
        // D s_36_15: write-var w <= s_36_9
        fn_state.w = s_36_9;
        // D s_36_16: write-var px <= s_36_10
        fn_state.px = s_36_10;
        // D s_36_17: write-var ux <= s_36_11
        fn_state.ux = s_36_11;
        // D s_36_18: write-var w_rcw <= s_36_12
        fn_state.w_rcw = s_36_12;
        // D s_36_19: write-var w_mmu <= s_36_13
        fn_state.w_mmu = s_36_13;
        // N s_36_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_37_0: read-var s2po:u8
        let s_37_0: u8 = fn_state.s2po;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 4u16);
        // C s_37_2: const #13u : u8
        let s_37_2: u8 = 13;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 4u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: not s_37_4
        let s_37_5: bool = !s_37_4;
        // N s_37_6: branch s_37_5 b39 b38
        if s_37_5 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // C s_38_1: const #1u : u8
        let s_38_1: bool = true;
        // C s_38_2: const #0u : u8
        let s_38_2: bool = false;
        // C s_38_3: const #1u : u8
        let s_38_3: bool = true;
        // C s_38_4: const #1u : u8
        let s_38_4: bool = true;
        // C s_38_5: const #1u : u8
        let s_38_5: bool = true;
        // D s_38_6: create-product struct = ["s_38_0", "s_38_1", "s_38_2", "s_38_3", "s_38_4", "s_38_5"]
        let s_38_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_38_0,
            _1: s_38_1,
            _2: s_38_2,
            _3: s_38_3,
            _4: s_38_4,
            _5: s_38_5,
        };
        // D s_38_7: write-var ga#13890 <= s_38_6
        fn_state.ga_13890 = s_38_6;
        // D s_38_8: read-var ga#13890.0:struct
        let s_38_8: bool = fn_state.ga_13890._0;
        // D s_38_9: read-var ga#13890.1:struct
        let s_38_9: bool = fn_state.ga_13890._1;
        // D s_38_10: read-var ga#13890.2:struct
        let s_38_10: bool = fn_state.ga_13890._2;
        // D s_38_11: read-var ga#13890.3:struct
        let s_38_11: bool = fn_state.ga_13890._3;
        // D s_38_12: read-var ga#13890.4:struct
        let s_38_12: bool = fn_state.ga_13890._4;
        // D s_38_13: read-var ga#13890.5:struct
        let s_38_13: bool = fn_state.ga_13890._5;
        // D s_38_14: write-var r <= s_38_8
        fn_state.r = s_38_8;
        // D s_38_15: write-var w <= s_38_9
        fn_state.w = s_38_9;
        // D s_38_16: write-var px <= s_38_10
        fn_state.px = s_38_10;
        // D s_38_17: write-var ux <= s_38_11
        fn_state.ux = s_38_11;
        // D s_38_18: write-var w_rcw <= s_38_12
        fn_state.w_rcw = s_38_12;
        // D s_38_19: write-var w_mmu <= s_38_13
        fn_state.w_mmu = s_38_13;
        // N s_38_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_39_0: read-var s2po:u8
        let s_39_0: u8 = fn_state.s2po;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 4u16);
        // C s_39_2: const #14u : u8
        let s_39_2: u8 = 14;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 4u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: not s_39_4
        let s_39_5: bool = !s_39_4;
        // N s_39_6: branch s_39_5 b41 b40
        if s_39_5 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // C s_40_1: const #1u : u8
        let s_40_1: bool = true;
        // C s_40_2: const #1u : u8
        let s_40_2: bool = true;
        // C s_40_3: const #0u : u8
        let s_40_3: bool = false;
        // C s_40_4: const #1u : u8
        let s_40_4: bool = true;
        // C s_40_5: const #1u : u8
        let s_40_5: bool = true;
        // D s_40_6: create-product struct = ["s_40_0", "s_40_1", "s_40_2", "s_40_3", "s_40_4", "s_40_5"]
        let s_40_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_40_0,
            _1: s_40_1,
            _2: s_40_2,
            _3: s_40_3,
            _4: s_40_4,
            _5: s_40_5,
        };
        // D s_40_7: write-var ga#13891 <= s_40_6
        fn_state.ga_13891 = s_40_6;
        // D s_40_8: read-var ga#13891.0:struct
        let s_40_8: bool = fn_state.ga_13891._0;
        // D s_40_9: read-var ga#13891.1:struct
        let s_40_9: bool = fn_state.ga_13891._1;
        // D s_40_10: read-var ga#13891.2:struct
        let s_40_10: bool = fn_state.ga_13891._2;
        // D s_40_11: read-var ga#13891.3:struct
        let s_40_11: bool = fn_state.ga_13891._3;
        // D s_40_12: read-var ga#13891.4:struct
        let s_40_12: bool = fn_state.ga_13891._4;
        // D s_40_13: read-var ga#13891.5:struct
        let s_40_13: bool = fn_state.ga_13891._5;
        // D s_40_14: write-var r <= s_40_8
        fn_state.r = s_40_8;
        // D s_40_15: write-var w <= s_40_9
        fn_state.w = s_40_9;
        // D s_40_16: write-var px <= s_40_10
        fn_state.px = s_40_10;
        // D s_40_17: write-var ux <= s_40_11
        fn_state.ux = s_40_11;
        // D s_40_18: write-var w_rcw <= s_40_12
        fn_state.w_rcw = s_40_12;
        // D s_40_19: write-var w_mmu <= s_40_13
        fn_state.w_mmu = s_40_13;
        // N s_40_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // C s_41_1: const #1u : u8
        let s_41_1: bool = true;
        // C s_41_2: const #1u : u8
        let s_41_2: bool = true;
        // C s_41_3: const #1u : u8
        let s_41_3: bool = true;
        // C s_41_4: const #1u : u8
        let s_41_4: bool = true;
        // C s_41_5: const #1u : u8
        let s_41_5: bool = true;
        // D s_41_6: create-product struct = ["s_41_0", "s_41_1", "s_41_2", "s_41_3", "s_41_4", "s_41_5"]
        let s_41_6: ProductType74f83332f678823c = ProductType74f83332f678823c {
            _0: s_41_0,
            _1: s_41_1,
            _2: s_41_2,
            _3: s_41_3,
            _4: s_41_4,
            _5: s_41_5,
        };
        // D s_41_7: write-var ga#13892 <= s_41_6
        fn_state.ga_13892 = s_41_6;
        // D s_41_8: read-var ga#13892.0:struct
        let s_41_8: bool = fn_state.ga_13892._0;
        // D s_41_9: read-var ga#13892.1:struct
        let s_41_9: bool = fn_state.ga_13892._1;
        // D s_41_10: read-var ga#13892.2:struct
        let s_41_10: bool = fn_state.ga_13892._2;
        // D s_41_11: read-var ga#13892.3:struct
        let s_41_11: bool = fn_state.ga_13892._3;
        // D s_41_12: read-var ga#13892.4:struct
        let s_41_12: bool = fn_state.ga_13892._4;
        // D s_41_13: read-var ga#13892.5:struct
        let s_41_13: bool = fn_state.ga_13892._5;
        // D s_41_14: write-var r <= s_41_8
        fn_state.r = s_41_8;
        // D s_41_15: write-var w <= s_41_9
        fn_state.w = s_41_9;
        // D s_41_16: write-var px <= s_41_10
        fn_state.px = s_41_10;
        // D s_41_17: write-var ux <= s_41_11
        fn_state.ux = s_41_11;
        // D s_41_18: write-var w_rcw <= s_41_12
        fn_state.w_rcw = s_41_12;
        // D s_41_19: write-var w_mmu <= s_41_13
        fn_state.w_mmu = s_41_13;
        // N s_41_20: jump b2
        return block_2(state, tracer, fn_state);
    }
}
