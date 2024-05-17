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
pub fn AArch64_S2DirectBasePermissions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    permissions: ProductTypebf05c51f33174538,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductType2fc9d3588999ac79 {
    #[derive(Default)]
    struct FunctionState {
        ux: bool,
        r: bool,
        ga_13841: ProductType8b847afc727d5818,
        ga_13842: ProductType8b847afc727d5818,
        ga_13840: u8,
        ga_13844: ProductType8b847afc727d5818,
        w: bool,
        s2perms: ProductType2fc9d3588999ac79,
        px: bool,
        ga_13843: ProductType8b847afc727d5818,
        x: bool,
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
        // D s_0_0: read-var permissions.7:struct
        let s_0_0: u8 = fn_state.permissions._7;
        // C s_0_1: const #0s : i
        let s_0_1: i128 = 0;
        // D s_0_2: cast zx s_0_0 -> bv
        let s_0_2: Bits = Bits::new(s_0_0 as u128, 2u16);
        // C s_0_3: const #1u : u64
        let s_0_3: u64 = 1;
        // D s_0_4: bit-extract s_0_2 s_0_1 s_0_3
        let s_0_4: Bits = (Bits::new(
            ((s_0_2) >> (s_0_1)).value(),
            u16::try_from(s_0_3).unwrap(),
        ));
        // D s_0_5: cast reint s_0_4 -> u8
        let s_0_5: bool = ((s_0_4.value()) != 0);
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // C s_0_7: const #0u : u64
        let s_0_7: u64 = 0;
        // D s_0_8: cast zx s_0_5 -> u64
        let s_0_8: u64 = (s_0_5 as u64);
        // C s_0_9: const #1u : u64
        let s_0_9: u64 = 1;
        // D s_0_10: and s_0_8 s_0_9
        let s_0_10: u64 = ((s_0_8) & (s_0_9));
        // D s_0_11: cmp-eq s_0_10 s_0_9
        let s_0_11: bool = ((s_0_10) == (s_0_9));
        // D s_0_12: lsl s_0_8 s_0_6
        let s_0_12: u64 = s_0_8 << s_0_6;
        // D s_0_13: or s_0_7 s_0_12
        let s_0_13: u64 = ((s_0_7) | (s_0_12));
        // D s_0_14: cmpl s_0_12
        let s_0_14: u64 = !s_0_12;
        // D s_0_15: and s_0_7 s_0_14
        let s_0_15: u64 = ((s_0_7) & (s_0_14));
        // D s_0_16: select s_0_11 s_0_13 s_0_15
        let s_0_16: u64 = if s_0_11 { s_0_13 } else { s_0_15 };
        // D s_0_17: cast trunc s_0_16 -> u8
        let s_0_17: bool = ((s_0_16) != 0);
        // D s_0_18: write-var r <= s_0_17
        fn_state.r = s_0_17;
        // D s_0_19: read-var permissions.7:struct
        let s_0_19: u8 = fn_state.permissions._7;
        // C s_0_20: const #1s : i
        let s_0_20: i128 = 1;
        // D s_0_21: cast zx s_0_19 -> bv
        let s_0_21: Bits = Bits::new(s_0_19 as u128, 2u16);
        // C s_0_22: const #1u : u64
        let s_0_22: u64 = 1;
        // D s_0_23: bit-extract s_0_21 s_0_20 s_0_22
        let s_0_23: Bits = (Bits::new(
            ((s_0_21) >> (s_0_20)).value(),
            u16::try_from(s_0_22).unwrap(),
        ));
        // D s_0_24: cast reint s_0_23 -> u8
        let s_0_24: bool = ((s_0_23.value()) != 0);
        // C s_0_25: const #0s : i
        let s_0_25: i128 = 0;
        // C s_0_26: const #0u : u64
        let s_0_26: u64 = 0;
        // D s_0_27: cast zx s_0_24 -> u64
        let s_0_27: u64 = (s_0_24 as u64);
        // C s_0_28: const #1u : u64
        let s_0_28: u64 = 1;
        // D s_0_29: and s_0_27 s_0_28
        let s_0_29: u64 = ((s_0_27) & (s_0_28));
        // D s_0_30: cmp-eq s_0_29 s_0_28
        let s_0_30: bool = ((s_0_29) == (s_0_28));
        // D s_0_31: lsl s_0_27 s_0_25
        let s_0_31: u64 = s_0_27 << s_0_25;
        // D s_0_32: or s_0_26 s_0_31
        let s_0_32: u64 = ((s_0_26) | (s_0_31));
        // D s_0_33: cmpl s_0_31
        let s_0_33: u64 = !s_0_31;
        // D s_0_34: and s_0_26 s_0_33
        let s_0_34: u64 = ((s_0_26) & (s_0_33));
        // D s_0_35: select s_0_30 s_0_32 s_0_34
        let s_0_35: u64 = if s_0_30 { s_0_32 } else { s_0_34 };
        // D s_0_36: cast trunc s_0_35 -> u8
        let s_0_36: bool = ((s_0_35) != 0);
        // D s_0_37: write-var w <= s_0_36
        fn_state.w = s_0_36;
        // D s_0_38: read-var permissions.12:struct
        let s_0_38: bool = fn_state.permissions._12;
        // D s_0_39: read-var permissions.13:struct
        let s_0_39: bool = fn_state.permissions._13;
        // D s_0_40: cast zx s_0_38 -> bv
        let s_0_40: Bits = Bits::new(s_0_38 as u128, 1u16);
        // D s_0_41: cast zx s_0_39 -> bv
        let s_0_41: Bits = Bits::new(s_0_39 as u128, 1u16);
        // D s_0_42: cast reint s_0_40 -> u128
        let s_0_42: u128 = (s_0_40.value() as u128);
        // D s_0_43: size-of s_0_40
        let s_0_43: u16 = s_0_40.length();
        // D s_0_44: cast reint s_0_41 -> u128
        let s_0_44: u128 = (s_0_41.value() as u128);
        // D s_0_45: size-of s_0_41
        let s_0_45: u16 = s_0_41.length();
        // D s_0_46: lsl s_0_42 s_0_45
        let s_0_46: u128 = s_0_42 << s_0_45;
        // D s_0_47: or s_0_46 s_0_44
        let s_0_47: u128 = ((s_0_46) | (s_0_44));
        // D s_0_48: add s_0_43 s_0_45
        let s_0_48: u16 = (s_0_43 + s_0_45);
        // D s_0_49: create-bits s_0_47 s_0_48
        let s_0_49: Bits = Bits::new(s_0_47, s_0_48);
        // D s_0_50: cast reint s_0_49 -> u8
        let s_0_50: u8 = (s_0_49.value() as u8);
        // D s_0_51: write-var ga#13840 <= s_0_50
        fn_state.ga_13840 = s_0_50;
        // D s_0_52: read-var ga#13840:u8
        let s_0_52: u8 = fn_state.ga_13840;
        // D s_0_53: cast zx s_0_52 -> bv
        let s_0_53: Bits = Bits::new(s_0_52 as u128, 2u16);
        // C s_0_54: const #0u : u8
        let s_0_54: u8 = 0;
        // C s_0_55: cast zx s_0_54 -> bv
        let s_0_55: Bits = Bits::new(s_0_54 as u128, 2u16);
        // D s_0_56: cmp-eq s_0_53 s_0_55
        let s_0_56: bool = ((s_0_53) == (s_0_55));
        // D s_0_57: not s_0_56
        let s_0_57: bool = !s_0_56;
        // N s_0_58: branch s_0_57 b6 b1
        if s_0_57 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_1_0: const #1u : u8
        let s_1_0: bool = true;
        // C s_1_1: const #1u : u8
        let s_1_1: bool = true;
        // D s_1_2: create-product struct = ["s_1_0", "s_1_1"]
        let s_1_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_1_0,
            _1: s_1_1,
        };
        // D s_1_3: write-var ga#13841 <= s_1_2
        fn_state.ga_13841 = s_1_2;
        // D s_1_4: read-var ga#13841.0:struct
        let s_1_4: bool = fn_state.ga_13841._0;
        // D s_1_5: read-var ga#13841.1:struct
        let s_1_5: bool = fn_state.ga_13841._1;
        // D s_1_6: write-var px <= s_1_4
        fn_state.px = s_1_4;
        // D s_1_7: write-var ux <= s_1_5
        fn_state.ux = s_1_5;
        // N s_1_8: jump b2
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
        // N s_2_6: branch s_2_5 b5 b3
        if s_2_5 {
            return block_5(state, tracer, fn_state);
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
        // D s_4_1: write-var s2perms.8 <= s_4_0
        fn_state.s2perms._8 = s_4_0;
        // D s_4_2: read-var w:u8
        let s_4_2: bool = fn_state.w;
        // D s_4_3: write-var s2perms.13 <= s_4_2
        fn_state.s2perms._13 = s_4_2;
        // D s_4_4: read-var x:u8
        let s_4_4: bool = fn_state.x;
        // D s_4_5: write-var s2perms.16 <= s_4_4
        fn_state.s2perms._16 = s_4_4;
        // D s_4_6: read-var r:u8
        let s_4_6: bool = fn_state.r;
        // D s_4_7: write-var s2perms.10 <= s_4_6
        fn_state.s2perms._10 = s_4_6;
        // D s_4_8: read-var w:u8
        let s_4_8: bool = fn_state.w;
        // D s_4_9: write-var s2perms.15 <= s_4_8
        fn_state.s2perms._15 = s_4_8;
        // D s_4_10: read-var r:u8
        let s_4_10: bool = fn_state.r;
        // D s_4_11: write-var s2perms.9 <= s_4_10
        fn_state.s2perms._9 = s_4_10;
        // D s_4_12: read-var w:u8
        let s_4_12: bool = fn_state.w;
        // D s_4_13: write-var s2perms.14 <= s_4_12
        fn_state.s2perms._14 = s_4_12;
        // D s_4_14: read-var s2perms:struct
        let s_4_14: ProductType2fc9d3588999ac79 = fn_state.s2perms;
        // N s_4_15: return s_4_14
        return s_4_14;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_5_0: read-var ux:u8
        let s_5_0: bool = fn_state.ux;
        // D s_5_1: write-var x <= s_5_0
        fn_state.x = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_6_0: read-var ga#13840:u8
        let s_6_0: u8 = fn_state.ga_13840;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #1u : u8
        let s_6_2: u8 = 1;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // C s_7_1: const #1u : u8
        let s_7_1: bool = true;
        // D s_7_2: create-product struct = ["s_7_0", "s_7_1"]
        let s_7_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_7_0,
            _1: s_7_1,
        };
        // D s_7_3: write-var ga#13842 <= s_7_2
        fn_state.ga_13842 = s_7_2;
        // D s_7_4: read-var ga#13842.0:struct
        let s_7_4: bool = fn_state.ga_13842._0;
        // D s_7_5: read-var ga#13842.1:struct
        let s_7_5: bool = fn_state.ga_13842._1;
        // D s_7_6: write-var px <= s_7_4
        fn_state.px = s_7_4;
        // D s_7_7: write-var ux <= s_7_5
        fn_state.ux = s_7_5;
        // N s_7_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_8_0: read-var ga#13840:u8
        let s_8_0: u8 = fn_state.ga_13840;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // C s_9_1: const #0u : u8
        let s_9_1: bool = false;
        // D s_9_2: create-product struct = ["s_9_0", "s_9_1"]
        let s_9_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_9_0,
            _1: s_9_1,
        };
        // D s_9_3: write-var ga#13843 <= s_9_2
        fn_state.ga_13843 = s_9_2;
        // D s_9_4: read-var ga#13843.0:struct
        let s_9_4: bool = fn_state.ga_13843._0;
        // D s_9_5: read-var ga#13843.1:struct
        let s_9_5: bool = fn_state.ga_13843._1;
        // D s_9_6: write-var px <= s_9_4
        fn_state.px = s_9_4;
        // D s_9_7: write-var ux <= s_9_5
        fn_state.ux = s_9_5;
        // N s_9_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // C s_10_1: const #0u : u8
        let s_10_1: bool = false;
        // D s_10_2: create-product struct = ["s_10_0", "s_10_1"]
        let s_10_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_10_0,
            _1: s_10_1,
        };
        // D s_10_3: write-var ga#13844 <= s_10_2
        fn_state.ga_13844 = s_10_2;
        // D s_10_4: read-var ga#13844.0:struct
        let s_10_4: bool = fn_state.ga_13844._0;
        // D s_10_5: read-var ga#13844.1:struct
        let s_10_5: bool = fn_state.ga_13844._1;
        // D s_10_6: write-var px <= s_10_4
        fn_state.px = s_10_4;
        // D s_10_7: write-var ux <= s_10_5
        fn_state.ux = s_10_5;
        // N s_10_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
