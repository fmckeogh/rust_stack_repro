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
use ConstrainUnpredictableBits::*;
use u__IMPDEF_MemoryAttributes::*;
use DecodeSDFAttr::*;
use Unreachable::*;
use common::*;
pub fn AArch32_DefaultTEXDecode<T: Tracer>(
    state: &mut State,
    tracer: &T,
    TEX_in: u8,
    C_in: bool,
    B_in: bool,
    s: bool,
) -> ProductTypef170cab34335b70c {
    #[derive(Default)]
    struct FunctionState {
        ga_22168: u8,
        ga_22176: ProductType183e6678e5239c85,
        ga_22174: ProductType183e6678e5239c85,
        ga_22184: ProductType183e6678e5239c85,
        ga_22182: ProductType183e6678e5239c85,
        C: bool,
        gs_453910: ProductType690b94b58c91cec7,
        memattrs: ProductTypef170cab34335b70c,
        gs_28575: bool,
        B: bool,
        TEX: u8,
        gs_28621: bool,
        gs_28574: bool,
        gs_28577: bool,
        gs_28576: bool,
        gs_28618: bool,
        TEX_in: u8,
        C_in: bool,
        B_in: bool,
        s: bool,
    }
    let fn_state = FunctionState {
        TEX_in,
        C_in,
        B_in,
        s,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_0_0: read-var TEX_in:u8
        let s_0_0: u8 = fn_state.TEX_in;
        // D s_0_1: write-var TEX <= s_0_0
        fn_state.TEX = s_0_0;
        // D s_0_2: read-var C_in:u8
        let s_0_2: bool = fn_state.C_in;
        // D s_0_3: write-var C <= s_0_2
        fn_state.C = s_0_2;
        // D s_0_4: read-var B_in:u8
        let s_0_4: bool = fn_state.B_in;
        // D s_0_5: write-var B <= s_0_4
        fn_state.B = s_0_4;
        // D s_0_6: read-var TEX:u8
        let s_0_6: u8 = fn_state.TEX;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 3u16);
        // C s_0_8: const #1u : u8
        let s_0_8: u8 = 1;
        // C s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 3u16);
        // D s_0_10: cmp-eq s_0_7 s_0_9
        let s_0_10: bool = ((s_0_7) == (s_0_9));
        // N s_0_11: branch s_0_10 b57 b1
        if s_0_10 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#28574 <= s_1_0
        fn_state.gs_28574 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_2_0: read-var gs#28574:u8
        let s_2_0: bool = fn_state.gs_28574;
        // N s_2_1: branch s_2_0 b56 b3
        if s_2_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_3_0: read-var TEX:u8
        let s_3_0: u8 = fn_state.TEX;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 3u16);
        // C s_3_2: const #2u : u8
        let s_3_2: u8 = 2;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 3u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b55 b4
        if s_3_4 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#28575 <= s_4_0
        fn_state.gs_28575 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_5_0: read-var gs#28575:u8
        let s_5_0: bool = fn_state.gs_28575;
        // D s_5_1: write-var gs#28576 <= s_5_0
        fn_state.gs_28576 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_6_0: read-var gs#28576:u8
        let s_6_0: bool = fn_state.gs_28576;
        // N s_6_1: branch s_6_0 b54 b7
        if s_6_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_7_0: read-var TEX:u8
        let s_7_0: u8 = fn_state.TEX;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 3u16);
        // C s_7_2: const #3u : u8
        let s_7_2: u8 = 3;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 3u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: write-var gs#28577 <= s_7_4
        fn_state.gs_28577 = s_7_4;
        // N s_7_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_8_0: read-var gs#28577:u8
        let s_8_0: bool = fn_state.gs_28577;
        // N s_8_1: branch s_8_0 b53 b9
        if s_8_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_10_0: read-var TEX:u8
        let s_10_0: u8 = fn_state.TEX;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 3u16);
        // D s_10_2: read-var C:u8
        let s_10_2: bool = fn_state.C;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cast reint s_10_1 -> u128
        let s_10_4: u128 = (s_10_1.value() as u128);
        // D s_10_5: size-of s_10_1
        let s_10_5: u16 = s_10_1.length();
        // D s_10_6: cast reint s_10_3 -> u128
        let s_10_6: u128 = (s_10_3.value() as u128);
        // D s_10_7: size-of s_10_3
        let s_10_7: u16 = s_10_3.length();
        // D s_10_8: lsl s_10_4 s_10_7
        let s_10_8: u128 = s_10_4 << s_10_7;
        // D s_10_9: or s_10_8 s_10_6
        let s_10_9: u128 = ((s_10_8) | (s_10_6));
        // D s_10_10: add s_10_5 s_10_7
        let s_10_10: u16 = (s_10_5 + s_10_7);
        // D s_10_11: create-bits s_10_9 s_10_10
        let s_10_11: Bits = Bits::new(s_10_9, s_10_10);
        // D s_10_12: cast reint s_10_11 -> u8
        let s_10_12: u8 = (s_10_11.value() as u8);
        // D s_10_13: cast zx s_10_12 -> bv
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 4u16);
        // D s_10_14: read-var B:u8
        let s_10_14: bool = fn_state.B;
        // D s_10_15: cast zx s_10_14 -> bv
        let s_10_15: Bits = Bits::new(s_10_14 as u128, 1u16);
        // D s_10_16: cast reint s_10_13 -> u128
        let s_10_16: u128 = (s_10_13.value() as u128);
        // D s_10_17: size-of s_10_13
        let s_10_17: u16 = s_10_13.length();
        // D s_10_18: cast reint s_10_15 -> u128
        let s_10_18: u128 = (s_10_15.value() as u128);
        // D s_10_19: size-of s_10_15
        let s_10_19: u16 = s_10_15.length();
        // D s_10_20: lsl s_10_16 s_10_19
        let s_10_20: u128 = s_10_16 << s_10_19;
        // D s_10_21: or s_10_20 s_10_18
        let s_10_21: u128 = ((s_10_20) | (s_10_18));
        // D s_10_22: add s_10_17 s_10_19
        let s_10_22: u16 = (s_10_17 + s_10_19);
        // D s_10_23: create-bits s_10_21 s_10_22
        let s_10_23: Bits = Bits::new(s_10_21, s_10_22);
        // D s_10_24: cast reint s_10_23 -> u8
        let s_10_24: u8 = (s_10_23.value() as u8);
        // D s_10_25: write-var ga#22168 <= s_10_24
        fn_state.ga_22168 = s_10_24;
        // D s_10_26: read-var ga#22168:u8
        let s_10_26: u8 = fn_state.ga_22168;
        // D s_10_27: cast zx s_10_26 -> bv
        let s_10_27: Bits = Bits::new(s_10_26 as u128, 5u16);
        // C s_10_28: const #0u : u8
        let s_10_28: u8 = 0;
        // C s_10_29: cast zx s_10_28 -> bv
        let s_10_29: Bits = Bits::new(s_10_28 as u128, 5u16);
        // D s_10_30: cmp-eq s_10_27 s_10_29
        let s_10_30: bool = ((s_10_27) == (s_10_29));
        // D s_10_31: not s_10_30
        let s_10_31: bool = !s_10_30;
        // N s_10_32: branch s_10_31 b19 b11
        if s_10_31 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_11_0: const #1u : u32
        let s_11_0: u32 = 1;
        // D s_11_1: write-var memattrs.2 <= s_11_0
        fn_state.memattrs._2 = s_11_0;
        // C s_11_2: const #3u : u32
        let s_11_2: u32 = 3;
        // D s_11_3: write-var memattrs.0 <= s_11_2
        fn_state.memattrs._0 = s_11_2;
        // C s_11_4: const #2u : u32
        let s_11_4: u32 = 2;
        // D s_11_5: write-var memattrs.5 <= s_11_4
        fn_state.memattrs._5 = s_11_4;
        // N s_11_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var memattrs.1.2 <= s_12_0
        fn_state.memattrs._1._2 = s_12_0;
        // C s_12_2: const #0u : u8
        let s_12_2: bool = false;
        // D s_12_3: write-var memattrs.4.2 <= s_12_2
        fn_state.memattrs._4._2 = s_12_2;
        // C s_12_4: const #0u : u32
        let s_12_4: u32 = 0;
        // D s_12_5: write-var memattrs.6 <= s_12_4
        fn_state.memattrs._6 = s_12_4;
        // D s_12_6: read-var memattrs.1:struct
        let s_12_6: ProductType183e6678e5239c85 = fn_state.memattrs._1;
        // D s_12_7: write-var ga#22182 <= s_12_6
        fn_state.ga_22182 = s_12_6;
        // D s_12_8: read-var ga#22182.0:struct
        let s_12_8: u8 = fn_state.ga_22182._0;
        // D s_12_9: cast zx s_12_8 -> bv
        let s_12_9: Bits = Bits::new(s_12_8 as u128, 2u16);
        // C s_12_10: const #480u : u32
        let s_12_10: u32 = 480;
        // D s_12_11: read-reg s_12_10:u8
        let s_12_11: u8 = {
            let value = state.read_register::<u8>(s_12_10 as isize);
            tracer.read_register(s_12_10 as isize, value);
            value
        };
        // D s_12_12: cast zx s_12_11 -> bv
        let s_12_12: Bits = Bits::new(s_12_11 as u128, 2u16);
        // D s_12_13: cmp-eq s_12_9 s_12_12
        let s_12_13: bool = ((s_12_9) == (s_12_12));
        // N s_12_14: branch s_12_13 b18 b13
        if s_12_13 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#28621 <= s_13_0
        fn_state.gs_28621 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_14_0: read-var gs#28621:u8
        let s_14_0: bool = fn_state.gs_28621;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var memattrs.7 <= s_15_0
        fn_state.memattrs._7 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_16_0: read-var memattrs:struct
        let s_16_0: ProductTypef170cab34335b70c = fn_state.memattrs;
        // N s_16_1: return s_16_0
        return s_16_0;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var memattrs.7 <= s_17_0
        fn_state.memattrs._7 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_18_0: read-var memattrs.4:struct
        let s_18_0: ProductType183e6678e5239c85 = fn_state.memattrs._4;
        // D s_18_1: write-var ga#22184 <= s_18_0
        fn_state.ga_22184 = s_18_0;
        // D s_18_2: read-var ga#22184.0:struct
        let s_18_2: u8 = fn_state.ga_22184._0;
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 2u16);
        // C s_18_4: const #480u : u32
        let s_18_4: u32 = 480;
        // D s_18_5: read-reg s_18_4:u8
        let s_18_5: u8 = {
            let value = state.read_register::<u8>(s_18_4 as isize);
            tracer.read_register(s_18_4 as isize, value);
            value
        };
        // D s_18_6: cast zx s_18_5 -> bv
        let s_18_6: Bits = Bits::new(s_18_5 as u128, 2u16);
        // D s_18_7: cmp-eq s_18_3 s_18_6
        let s_18_7: bool = ((s_18_3) == (s_18_6));
        // D s_18_8: write-var gs#28621 <= s_18_7
        fn_state.gs_28621 = s_18_7;
        // N s_18_9: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_19_0: read-var ga#22168:u8
        let s_19_0: u8 = fn_state.ga_22168;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 5u16);
        // C s_19_2: const #1u : u8
        let s_19_2: u8 = 1;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 5u16);
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
    ) -> ProductTypef170cab34335b70c {
        // C s_20_0: const #1u : u32
        let s_20_0: u32 = 1;
        // D s_20_1: write-var memattrs.2 <= s_20_0
        fn_state.memattrs._2 = s_20_0;
        // C s_20_2: const #2u : u32
        let s_20_2: u32 = 2;
        // D s_20_3: write-var memattrs.0 <= s_20_2
        fn_state.memattrs._0 = s_20_2;
        // C s_20_4: const #2u : u32
        let s_20_4: u32 = 2;
        // D s_20_5: write-var memattrs.5 <= s_20_4
        fn_state.memattrs._5 = s_20_4;
        // N s_20_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_21_0: read-var ga#22168:u8
        let s_21_0: u8 = fn_state.ga_22168;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 5u16);
        // C s_21_2: const #8u : u8
        let s_21_2: u8 = 8;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 5u16);
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
    ) -> ProductTypef170cab34335b70c {
        // C s_22_0: const #1u : u32
        let s_22_0: u32 = 1;
        // D s_22_1: write-var memattrs.2 <= s_22_0
        fn_state.memattrs._2 = s_22_0;
        // C s_22_2: const #2u : u32
        let s_22_2: u32 = 2;
        // D s_22_3: write-var memattrs.0 <= s_22_2
        fn_state.memattrs._0 = s_22_2;
        // C s_22_4: const #2u : u32
        let s_22_4: u32 = 2;
        // D s_22_5: write-var memattrs.5 <= s_22_4
        fn_state.memattrs._5 = s_22_4;
        // N s_22_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_23_0: read-var ga#22168:u8
        let s_23_0: u8 = fn_state.ga_22168;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 5u16);
        // C s_23_2: const #2u : u8
        let s_23_2: u8 = 2;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 5u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: not s_23_4
        let s_23_5: bool = !s_23_4;
        // N s_23_6: branch s_23_5 b28 b24
        if s_23_5 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_24_0: const #0u : u32
        let s_24_0: u32 = 0;
        // D s_24_1: write-var memattrs.2 <= s_24_0
        fn_state.memattrs._2 = s_24_0;
        // C s_24_2: const #472u : u32
        let s_24_2: u32 = 472;
        // D s_24_3: read-reg s_24_2:u8
        let s_24_3: u8 = {
            let value = state.read_register::<u8>(s_24_2 as isize);
            tracer.read_register(s_24_2 as isize, value);
            value
        };
        // D s_24_4: write-var memattrs.1.0 <= s_24_3
        fn_state.memattrs._1._0 = s_24_3;
        // C s_24_5: const #504u : u32
        let s_24_5: u32 = 504;
        // D s_24_6: read-reg s_24_5:u8
        let s_24_6: u8 = {
            let value = state.read_register::<u8>(s_24_5 as isize);
            tracer.read_register(s_24_5 as isize, value);
            value
        };
        // D s_24_7: write-var memattrs.1.1 <= s_24_6
        fn_state.memattrs._1._1 = s_24_6;
        // C s_24_8: const #472u : u32
        let s_24_8: u32 = 472;
        // D s_24_9: read-reg s_24_8:u8
        let s_24_9: u8 = {
            let value = state.read_register::<u8>(s_24_8 as isize);
            tracer.read_register(s_24_8 as isize, value);
            value
        };
        // D s_24_10: write-var memattrs.4.0 <= s_24_9
        fn_state.memattrs._4._0 = s_24_9;
        // C s_24_11: const #504u : u32
        let s_24_11: u32 = 504;
        // D s_24_12: read-reg s_24_11:u8
        let s_24_12: u8 = {
            let value = state.read_register::<u8>(s_24_11 as isize);
            tracer.read_register(s_24_11 as isize, value);
            value
        };
        // D s_24_13: write-var memattrs.4.1 <= s_24_12
        fn_state.memattrs._4._1 = s_24_12;
        // D s_24_14: read-var s:u8
        let s_24_14: bool = fn_state.s;
        // D s_24_15: cast zx s_24_14 -> bv
        let s_24_15: Bits = Bits::new(s_24_14 as u128, 1u16);
        // C s_24_16: const #1u : u8
        let s_24_16: bool = true;
        // C s_24_17: cast zx s_24_16 -> bv
        let s_24_17: Bits = Bits::new(s_24_16 as u128, 1u16);
        // D s_24_18: cmp-eq s_24_15 s_24_17
        let s_24_18: bool = ((s_24_15) == (s_24_17));
        // N s_24_19: branch s_24_18 b27 b25
        if s_24_18 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_25_0: const #0u : u32
        let s_25_0: u32 = 0;
        // D s_25_1: write-var memattrs.5 <= s_25_0
        fn_state.memattrs._5 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // N s_26_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_27_0: const #2u : u32
        let s_27_0: u32 = 2;
        // D s_27_1: write-var memattrs.5 <= s_27_0
        fn_state.memattrs._5 = s_27_0;
        // N s_27_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_28_0: read-var ga#22168:u8
        let s_28_0: u8 = fn_state.ga_22168;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 5u16);
        // C s_28_2: const #3u : u8
        let s_28_2: u8 = 3;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 5u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: not s_28_4
        let s_28_5: bool = !s_28_4;
        // N s_28_6: branch s_28_5 b33 b29
        if s_28_5 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_29_0: const #0u : u32
        let s_29_0: u32 = 0;
        // D s_29_1: write-var memattrs.2 <= s_29_0
        fn_state.memattrs._2 = s_29_0;
        // C s_29_2: const #480u : u32
        let s_29_2: u32 = 480;
        // D s_29_3: read-reg s_29_2:u8
        let s_29_3: u8 = {
            let value = state.read_register::<u8>(s_29_2 as isize);
            tracer.read_register(s_29_2 as isize, value);
            value
        };
        // D s_29_4: write-var memattrs.1.0 <= s_29_3
        fn_state.memattrs._1._0 = s_29_3;
        // C s_29_5: const #504u : u32
        let s_29_5: u32 = 504;
        // D s_29_6: read-reg s_29_5:u8
        let s_29_6: u8 = {
            let value = state.read_register::<u8>(s_29_5 as isize);
            tracer.read_register(s_29_5 as isize, value);
            value
        };
        // D s_29_7: write-var memattrs.1.1 <= s_29_6
        fn_state.memattrs._1._1 = s_29_6;
        // C s_29_8: const #480u : u32
        let s_29_8: u32 = 480;
        // D s_29_9: read-reg s_29_8:u8
        let s_29_9: u8 = {
            let value = state.read_register::<u8>(s_29_8 as isize);
            tracer.read_register(s_29_8 as isize, value);
            value
        };
        // D s_29_10: write-var memattrs.4.0 <= s_29_9
        fn_state.memattrs._4._0 = s_29_9;
        // C s_29_11: const #504u : u32
        let s_29_11: u32 = 504;
        // D s_29_12: read-reg s_29_11:u8
        let s_29_12: u8 = {
            let value = state.read_register::<u8>(s_29_11 as isize);
            tracer.read_register(s_29_11 as isize, value);
            value
        };
        // D s_29_13: write-var memattrs.4.1 <= s_29_12
        fn_state.memattrs._4._1 = s_29_12;
        // D s_29_14: read-var s:u8
        let s_29_14: bool = fn_state.s;
        // D s_29_15: cast zx s_29_14 -> bv
        let s_29_15: Bits = Bits::new(s_29_14 as u128, 1u16);
        // C s_29_16: const #1u : u8
        let s_29_16: bool = true;
        // C s_29_17: cast zx s_29_16 -> bv
        let s_29_17: Bits = Bits::new(s_29_16 as u128, 1u16);
        // D s_29_18: cmp-eq s_29_15 s_29_17
        let s_29_18: bool = ((s_29_15) == (s_29_17));
        // N s_29_19: branch s_29_18 b32 b30
        if s_29_18 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_30_0: const #0u : u32
        let s_30_0: u32 = 0;
        // D s_30_1: write-var memattrs.5 <= s_30_0
        fn_state.memattrs._5 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // N s_31_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_32_0: const #2u : u32
        let s_32_0: u32 = 2;
        // D s_32_1: write-var memattrs.5 <= s_32_0
        fn_state.memattrs._5 = s_32_0;
        // N s_32_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_33_0: read-var ga#22168:u8
        let s_33_0: u8 = fn_state.ga_22168;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 5u16);
        // C s_33_2: const #4u : u8
        let s_33_2: u8 = 4;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 5u16);
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
    ) -> ProductTypef170cab34335b70c {
        // C s_34_0: const #0u : u32
        let s_34_0: u32 = 0;
        // D s_34_1: write-var memattrs.2 <= s_34_0
        fn_state.memattrs._2 = s_34_0;
        // C s_34_2: const #464u : u32
        let s_34_2: u32 = 464;
        // D s_34_3: read-reg s_34_2:u8
        let s_34_3: u8 = {
            let value = state.read_register::<u8>(s_34_2 as isize);
            tracer.read_register(s_34_2 as isize, value);
            value
        };
        // D s_34_4: write-var memattrs.1.0 <= s_34_3
        fn_state.memattrs._1._0 = s_34_3;
        // C s_34_5: const #464u : u32
        let s_34_5: u32 = 464;
        // D s_34_6: read-reg s_34_5:u8
        let s_34_6: u8 = {
            let value = state.read_register::<u8>(s_34_5 as isize);
            tracer.read_register(s_34_5 as isize, value);
            value
        };
        // D s_34_7: write-var memattrs.4.0 <= s_34_6
        fn_state.memattrs._4._0 = s_34_6;
        // C s_34_8: const #2u : u32
        let s_34_8: u32 = 2;
        // D s_34_9: write-var memattrs.5 <= s_34_8
        fn_state.memattrs._5 = s_34_8;
        // N s_34_10: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_35_0: read-var ga#22168:u8
        let s_35_0: u8 = fn_state.ga_22168;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 5u16);
        // C s_35_2: const #6u : u8
        let s_35_2: u8 = 6;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 5u16);
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
    ) -> ProductTypef170cab34335b70c {
        // C s_36_0: const #"" : str
        let s_36_0: &'static str = "";
        // S s_36_1: call __IMPDEF_MemoryAttributes(s_36_0)
        let s_36_1: ProductTypef170cab34335b70c = u__IMPDEF_MemoryAttributes(
            state,
            tracer,
            s_36_0,
        );
        // D s_36_2: write-var memattrs <= s_36_1
        fn_state.memattrs = s_36_1;
        // N s_36_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_37_0: read-var ga#22168:u8
        let s_37_0: u8 = fn_state.ga_22168;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 5u16);
        // C s_37_2: const #7u : u8
        let s_37_2: u8 = 7;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 5u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: not s_37_4
        let s_37_5: bool = !s_37_4;
        // N s_37_6: branch s_37_5 b42 b38
        if s_37_5 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_38_0: const #0u : u32
        let s_38_0: u32 = 0;
        // D s_38_1: write-var memattrs.2 <= s_38_0
        fn_state.memattrs._2 = s_38_0;
        // C s_38_2: const #480u : u32
        let s_38_2: u32 = 480;
        // D s_38_3: read-reg s_38_2:u8
        let s_38_3: u8 = {
            let value = state.read_register::<u8>(s_38_2 as isize);
            tracer.read_register(s_38_2 as isize, value);
            value
        };
        // D s_38_4: write-var memattrs.1.0 <= s_38_3
        fn_state.memattrs._1._0 = s_38_3;
        // C s_38_5: const #512u : u32
        let s_38_5: u32 = 512;
        // D s_38_6: read-reg s_38_5:u8
        let s_38_6: u8 = {
            let value = state.read_register::<u8>(s_38_5 as isize);
            tracer.read_register(s_38_5 as isize, value);
            value
        };
        // D s_38_7: write-var memattrs.1.1 <= s_38_6
        fn_state.memattrs._1._1 = s_38_6;
        // C s_38_8: const #480u : u32
        let s_38_8: u32 = 480;
        // D s_38_9: read-reg s_38_8:u8
        let s_38_9: u8 = {
            let value = state.read_register::<u8>(s_38_8 as isize);
            tracer.read_register(s_38_8 as isize, value);
            value
        };
        // D s_38_10: write-var memattrs.4.0 <= s_38_9
        fn_state.memattrs._4._0 = s_38_9;
        // C s_38_11: const #512u : u32
        let s_38_11: u32 = 512;
        // D s_38_12: read-reg s_38_11:u8
        let s_38_12: u8 = {
            let value = state.read_register::<u8>(s_38_11 as isize);
            tracer.read_register(s_38_11 as isize, value);
            value
        };
        // D s_38_13: write-var memattrs.4.1 <= s_38_12
        fn_state.memattrs._4._1 = s_38_12;
        // D s_38_14: read-var s:u8
        let s_38_14: bool = fn_state.s;
        // D s_38_15: cast zx s_38_14 -> bv
        let s_38_15: Bits = Bits::new(s_38_14 as u128, 1u16);
        // C s_38_16: const #1u : u8
        let s_38_16: bool = true;
        // C s_38_17: cast zx s_38_16 -> bv
        let s_38_17: Bits = Bits::new(s_38_16 as u128, 1u16);
        // D s_38_18: cmp-eq s_38_15 s_38_17
        let s_38_18: bool = ((s_38_15) == (s_38_17));
        // N s_38_19: branch s_38_18 b41 b39
        if s_38_18 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_39_0: const #0u : u32
        let s_39_0: u32 = 0;
        // D s_39_1: write-var memattrs.5 <= s_39_0
        fn_state.memattrs._5 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // N s_40_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_41_0: const #2u : u32
        let s_41_0: u32 = 2;
        // D s_41_1: write-var memattrs.5 <= s_41_0
        fn_state.memattrs._5 = s_41_0;
        // N s_41_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_42_0: read-var ga#22168:u8
        let s_42_0: u8 = fn_state.ga_22168;
        // C s_42_1: const #4s : i
        let s_42_1: i128 = 4;
        // D s_42_2: cast zx s_42_0 -> bv
        let s_42_2: Bits = Bits::new(s_42_0 as u128, 5u16);
        // C s_42_3: const #1s : i64
        let s_42_3: i64 = 1;
        // C s_42_4: cast zx s_42_3 -> i
        let s_42_4: i128 = (i128::try_from(s_42_3).unwrap());
        // C s_42_5: const #0s : i
        let s_42_5: i128 = 0;
        // C s_42_6: add s_42_5 s_42_4
        let s_42_6: i128 = (s_42_5 + s_42_4);
        // D s_42_7: bit-extract s_42_2 s_42_1 s_42_6
        let s_42_7: Bits = (Bits::new(
            ((s_42_2) >> (s_42_1)).value(),
            u16::try_from(s_42_6).unwrap(),
        ));
        // D s_42_8: cast reint s_42_7 -> u8
        let s_42_8: bool = ((s_42_7.value()) != 0);
        // D s_42_9: cast zx s_42_8 -> bv
        let s_42_9: Bits = Bits::new(s_42_8 as u128, 1u16);
        // C s_42_10: const #1u : u8
        let s_42_10: bool = true;
        // C s_42_11: cast zx s_42_10 -> bv
        let s_42_11: Bits = Bits::new(s_42_10 as u128, 1u16);
        // D s_42_12: cmp-eq s_42_9 s_42_11
        let s_42_12: bool = ((s_42_9) == (s_42_11));
        // D s_42_13: not s_42_12
        let s_42_13: bool = !s_42_12;
        // N s_42_14: branch s_42_13 b52 b43
        if s_42_13 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_43_0: const #0u : u32
        let s_43_0: u32 = 0;
        // D s_43_1: write-var memattrs.2 <= s_43_0
        fn_state.memattrs._2 = s_43_0;
        // D s_43_2: read-var C:u8
        let s_43_2: bool = fn_state.C;
        // D s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: read-var B:u8
        let s_43_4: bool = fn_state.B;
        // D s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 1u16);
        // D s_43_6: cast reint s_43_3 -> u128
        let s_43_6: u128 = (s_43_3.value() as u128);
        // D s_43_7: size-of s_43_3
        let s_43_7: u16 = s_43_3.length();
        // D s_43_8: cast reint s_43_5 -> u128
        let s_43_8: u128 = (s_43_5.value() as u128);
        // D s_43_9: size-of s_43_5
        let s_43_9: u16 = s_43_5.length();
        // D s_43_10: lsl s_43_6 s_43_9
        let s_43_10: u128 = s_43_6 << s_43_9;
        // D s_43_11: or s_43_10 s_43_8
        let s_43_11: u128 = ((s_43_10) | (s_43_8));
        // D s_43_12: add s_43_7 s_43_9
        let s_43_12: u16 = (s_43_7 + s_43_9);
        // D s_43_13: create-bits s_43_11 s_43_12
        let s_43_13: Bits = Bits::new(s_43_11, s_43_12);
        // D s_43_14: cast reint s_43_13 -> u8
        let s_43_14: u8 = (s_43_13.value() as u8);
        // D s_43_15: call DecodeSDFAttr(s_43_14)
        let s_43_15: ProductType183e6678e5239c85 = DecodeSDFAttr(state, tracer, s_43_14);
        // D s_43_16: write-var memattrs.1 <= s_43_15
        fn_state.memattrs._1 = s_43_15;
        // C s_43_17: const #0s : i
        let s_43_17: i128 = 0;
        // D s_43_18: read-var TEX:u8
        let s_43_18: u8 = fn_state.TEX;
        // D s_43_19: cast zx s_43_18 -> bv
        let s_43_19: Bits = Bits::new(s_43_18 as u128, 3u16);
        // C s_43_20: const #1s : i64
        let s_43_20: i64 = 1;
        // C s_43_21: cast zx s_43_20 -> i
        let s_43_21: i128 = (i128::try_from(s_43_20).unwrap());
        // C s_43_22: const #1s : i
        let s_43_22: i128 = 1;
        // C s_43_23: add s_43_22 s_43_21
        let s_43_23: i128 = (s_43_22 + s_43_21);
        // D s_43_24: bit-extract s_43_19 s_43_17 s_43_23
        let s_43_24: Bits = (Bits::new(
            ((s_43_19) >> (s_43_17)).value(),
            u16::try_from(s_43_23).unwrap(),
        ));
        // D s_43_25: cast reint s_43_24 -> u8
        let s_43_25: u8 = (s_43_24.value() as u8);
        // D s_43_26: call DecodeSDFAttr(s_43_25)
        let s_43_26: ProductType183e6678e5239c85 = DecodeSDFAttr(state, tracer, s_43_25);
        // D s_43_27: write-var memattrs.4 <= s_43_26
        fn_state.memattrs._4 = s_43_26;
        // D s_43_28: read-var memattrs.1:struct
        let s_43_28: ProductType183e6678e5239c85 = fn_state.memattrs._1;
        // D s_43_29: write-var ga#22174 <= s_43_28
        fn_state.ga_22174 = s_43_28;
        // D s_43_30: read-var ga#22174.0:struct
        let s_43_30: u8 = fn_state.ga_22174._0;
        // D s_43_31: cast zx s_43_30 -> bv
        let s_43_31: Bits = Bits::new(s_43_30 as u128, 2u16);
        // C s_43_32: const #464u : u32
        let s_43_32: u32 = 464;
        // D s_43_33: read-reg s_43_32:u8
        let s_43_33: u8 = {
            let value = state.read_register::<u8>(s_43_32 as isize);
            tracer.read_register(s_43_32 as isize, value);
            value
        };
        // D s_43_34: cast zx s_43_33 -> bv
        let s_43_34: Bits = Bits::new(s_43_33 as u128, 2u16);
        // D s_43_35: cmp-eq s_43_31 s_43_34
        let s_43_35: bool = ((s_43_31) == (s_43_34));
        // N s_43_36: branch s_43_35 b51 b44
        if s_43_35 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#28618 <= s_44_0
        fn_state.gs_28618 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_45_0: read-var gs#28618:u8
        let s_45_0: bool = fn_state.gs_28618;
        // N s_45_1: branch s_45_0 b50 b46
        if s_45_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_46_0: read-var s:u8
        let s_46_0: bool = fn_state.s;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // N s_46_5: branch s_46_4 b49 b47
        if s_46_4 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_47_0: const #0u : u32
        let s_47_0: u32 = 0;
        // D s_47_1: write-var memattrs.5 <= s_47_0
        fn_state.memattrs._5 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // N s_48_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_49_0: const #2u : u32
        let s_49_0: u32 = 2;
        // D s_49_1: write-var memattrs.5 <= s_49_0
        fn_state.memattrs._5 = s_49_0;
        // N s_49_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_50_0: const #2u : u32
        let s_50_0: u32 = 2;
        // D s_50_1: write-var memattrs.5 <= s_50_0
        fn_state.memattrs._5 = s_50_0;
        // N s_50_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_51_0: read-var memattrs.4:struct
        let s_51_0: ProductType183e6678e5239c85 = fn_state.memattrs._4;
        // D s_51_1: write-var ga#22176 <= s_51_0
        fn_state.ga_22176 = s_51_0;
        // D s_51_2: read-var ga#22176.0:struct
        let s_51_2: u8 = fn_state.ga_22176._0;
        // D s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 2u16);
        // C s_51_4: const #464u : u32
        let s_51_4: u32 = 464;
        // D s_51_5: read-reg s_51_4:u8
        let s_51_5: u8 = {
            let value = state.read_register::<u8>(s_51_4 as isize);
            tracer.read_register(s_51_4 as isize, value);
            value
        };
        // D s_51_6: cast zx s_51_5 -> bv
        let s_51_6: Bits = Bits::new(s_51_5 as u128, 2u16);
        // D s_51_7: cmp-eq s_51_3 s_51_6
        let s_51_7: bool = ((s_51_3) == (s_51_6));
        // D s_51_8: write-var gs#28618 <= s_51_7
        fn_state.gs_28618 = s_51_7;
        // N s_51_9: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call Unreachable(s_52_0)
        let s_52_1: () = Unreachable(state, tracer, s_52_0);
        // N s_52_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_53_0: const #5s : i
        let s_53_0: i128 = 5;
        // C s_53_1: const #12u : u32
        let s_53_1: u32 = 12;
        // S s_53_2: call ConstrainUnpredictableBits(s_53_1, s_53_0)
        let s_53_2: ProductType690b94b58c91cec7 = ConstrainUnpredictableBits(
            state,
            tracer,
            s_53_1,
            s_53_0,
        );
        // D s_53_3: write-var gs#453910 <= s_53_2
        fn_state.gs_453910 = s_53_2;
        // D s_53_4: read-var gs#453910.1:struct
        let s_53_4: Bits = fn_state.gs_453910._1;
        // D s_53_5: cast reint s_53_4 -> u8
        let s_53_5: u8 = (s_53_4.value() as u8);
        // C s_53_6: const #2s : i
        let s_53_6: i128 = 2;
        // D s_53_7: cast zx s_53_5 -> bv
        let s_53_7: Bits = Bits::new(s_53_5 as u128, 5u16);
        // C s_53_8: const #1s : i64
        let s_53_8: i64 = 1;
        // C s_53_9: cast zx s_53_8 -> i
        let s_53_9: i128 = (i128::try_from(s_53_8).unwrap());
        // C s_53_10: const #2s : i
        let s_53_10: i128 = 2;
        // C s_53_11: add s_53_10 s_53_9
        let s_53_11: i128 = (s_53_10 + s_53_9);
        // D s_53_12: bit-extract s_53_7 s_53_6 s_53_11
        let s_53_12: Bits = (Bits::new(
            ((s_53_7) >> (s_53_6)).value(),
            u16::try_from(s_53_11).unwrap(),
        ));
        // D s_53_13: cast reint s_53_12 -> u8
        let s_53_13: u8 = (s_53_12.value() as u8);
        // D s_53_14: write-var TEX <= s_53_13
        fn_state.TEX = s_53_13;
        // C s_53_15: const #1s : i
        let s_53_15: i128 = 1;
        // D s_53_16: cast zx s_53_5 -> bv
        let s_53_16: Bits = Bits::new(s_53_5 as u128, 5u16);
        // C s_53_17: const #1u : u64
        let s_53_17: u64 = 1;
        // D s_53_18: bit-extract s_53_16 s_53_15 s_53_17
        let s_53_18: Bits = (Bits::new(
            ((s_53_16) >> (s_53_15)).value(),
            u16::try_from(s_53_17).unwrap(),
        ));
        // D s_53_19: cast reint s_53_18 -> u8
        let s_53_19: bool = ((s_53_18.value()) != 0);
        // C s_53_20: const #0s : i
        let s_53_20: i128 = 0;
        // C s_53_21: const #0u : u64
        let s_53_21: u64 = 0;
        // D s_53_22: cast zx s_53_19 -> u64
        let s_53_22: u64 = (s_53_19 as u64);
        // C s_53_23: const #1u : u64
        let s_53_23: u64 = 1;
        // D s_53_24: and s_53_22 s_53_23
        let s_53_24: u64 = ((s_53_22) & (s_53_23));
        // D s_53_25: cmp-eq s_53_24 s_53_23
        let s_53_25: bool = ((s_53_24) == (s_53_23));
        // D s_53_26: lsl s_53_22 s_53_20
        let s_53_26: u64 = s_53_22 << s_53_20;
        // D s_53_27: or s_53_21 s_53_26
        let s_53_27: u64 = ((s_53_21) | (s_53_26));
        // D s_53_28: cmpl s_53_26
        let s_53_28: u64 = !s_53_26;
        // D s_53_29: and s_53_21 s_53_28
        let s_53_29: u64 = ((s_53_21) & (s_53_28));
        // D s_53_30: select s_53_25 s_53_27 s_53_29
        let s_53_30: u64 = if s_53_25 { s_53_27 } else { s_53_29 };
        // D s_53_31: cast trunc s_53_30 -> u8
        let s_53_31: bool = ((s_53_30) != 0);
        // D s_53_32: write-var C <= s_53_31
        fn_state.C = s_53_31;
        // C s_53_33: const #0s : i
        let s_53_33: i128 = 0;
        // D s_53_34: cast zx s_53_5 -> bv
        let s_53_34: Bits = Bits::new(s_53_5 as u128, 5u16);
        // C s_53_35: const #1u : u64
        let s_53_35: u64 = 1;
        // D s_53_36: bit-extract s_53_34 s_53_33 s_53_35
        let s_53_36: Bits = (Bits::new(
            ((s_53_34) >> (s_53_33)).value(),
            u16::try_from(s_53_35).unwrap(),
        ));
        // D s_53_37: cast reint s_53_36 -> u8
        let s_53_37: bool = ((s_53_36.value()) != 0);
        // C s_53_38: const #0s : i
        let s_53_38: i128 = 0;
        // C s_53_39: const #0u : u64
        let s_53_39: u64 = 0;
        // D s_53_40: cast zx s_53_37 -> u64
        let s_53_40: u64 = (s_53_37 as u64);
        // C s_53_41: const #1u : u64
        let s_53_41: u64 = 1;
        // D s_53_42: and s_53_40 s_53_41
        let s_53_42: u64 = ((s_53_40) & (s_53_41));
        // D s_53_43: cmp-eq s_53_42 s_53_41
        let s_53_43: bool = ((s_53_42) == (s_53_41));
        // D s_53_44: lsl s_53_40 s_53_38
        let s_53_44: u64 = s_53_40 << s_53_38;
        // D s_53_45: or s_53_39 s_53_44
        let s_53_45: u64 = ((s_53_39) | (s_53_44));
        // D s_53_46: cmpl s_53_44
        let s_53_46: u64 = !s_53_44;
        // D s_53_47: and s_53_39 s_53_46
        let s_53_47: u64 = ((s_53_39) & (s_53_46));
        // D s_53_48: select s_53_43 s_53_45 s_53_47
        let s_53_48: u64 = if s_53_43 { s_53_45 } else { s_53_47 };
        // D s_53_49: cast trunc s_53_48 -> u8
        let s_53_49: bool = ((s_53_48) != 0);
        // D s_53_50: write-var B <= s_53_49
        fn_state.B = s_53_49;
        // N s_53_51: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#28577 <= s_54_0
        fn_state.gs_28577 = s_54_0;
        // N s_54_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_55_0: read-var C:u8
        let s_55_0: bool = fn_state.C;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // D s_55_2: read-var B:u8
        let s_55_2: bool = fn_state.B;
        // D s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cast reint s_55_1 -> u128
        let s_55_4: u128 = (s_55_1.value() as u128);
        // D s_55_5: size-of s_55_1
        let s_55_5: u16 = s_55_1.length();
        // D s_55_6: cast reint s_55_3 -> u128
        let s_55_6: u128 = (s_55_3.value() as u128);
        // D s_55_7: size-of s_55_3
        let s_55_7: u16 = s_55_3.length();
        // D s_55_8: lsl s_55_4 s_55_7
        let s_55_8: u128 = s_55_4 << s_55_7;
        // D s_55_9: or s_55_8 s_55_6
        let s_55_9: u128 = ((s_55_8) | (s_55_6));
        // D s_55_10: add s_55_5 s_55_7
        let s_55_10: u16 = (s_55_5 + s_55_7);
        // D s_55_11: create-bits s_55_9 s_55_10
        let s_55_11: Bits = Bits::new(s_55_9, s_55_10);
        // D s_55_12: cast reint s_55_11 -> u8
        let s_55_12: u8 = (s_55_11.value() as u8);
        // D s_55_13: cast zx s_55_12 -> bv
        let s_55_13: Bits = Bits::new(s_55_12 as u128, 2u16);
        // C s_55_14: const #0u : u8
        let s_55_14: u8 = 0;
        // C s_55_15: cast zx s_55_14 -> bv
        let s_55_15: Bits = Bits::new(s_55_14 as u128, 2u16);
        // D s_55_16: cmp-ne s_55_13 s_55_15
        let s_55_16: bool = ((s_55_13) != (s_55_15));
        // D s_55_17: write-var gs#28575 <= s_55_16
        fn_state.gs_28575 = s_55_16;
        // N s_55_18: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#28576 <= s_56_0
        fn_state.gs_28576 = s_56_0;
        // N s_56_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_57_0: read-var C:u8
        let s_57_0: bool = fn_state.C;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 1u16);
        // D s_57_2: read-var B:u8
        let s_57_2: bool = fn_state.B;
        // D s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // D s_57_4: cast reint s_57_1 -> u128
        let s_57_4: u128 = (s_57_1.value() as u128);
        // D s_57_5: size-of s_57_1
        let s_57_5: u16 = s_57_1.length();
        // D s_57_6: cast reint s_57_3 -> u128
        let s_57_6: u128 = (s_57_3.value() as u128);
        // D s_57_7: size-of s_57_3
        let s_57_7: u16 = s_57_3.length();
        // D s_57_8: lsl s_57_4 s_57_7
        let s_57_8: u128 = s_57_4 << s_57_7;
        // D s_57_9: or s_57_8 s_57_6
        let s_57_9: u128 = ((s_57_8) | (s_57_6));
        // D s_57_10: add s_57_5 s_57_7
        let s_57_10: u16 = (s_57_5 + s_57_7);
        // D s_57_11: create-bits s_57_9 s_57_10
        let s_57_11: Bits = Bits::new(s_57_9, s_57_10);
        // D s_57_12: cast reint s_57_11 -> u8
        let s_57_12: u8 = (s_57_11.value() as u8);
        // D s_57_13: cast zx s_57_12 -> bv
        let s_57_13: Bits = Bits::new(s_57_12 as u128, 2u16);
        // C s_57_14: const #1u : u8
        let s_57_14: u8 = 1;
        // C s_57_15: cast zx s_57_14 -> bv
        let s_57_15: Bits = Bits::new(s_57_14 as u128, 2u16);
        // D s_57_16: cmp-eq s_57_13 s_57_15
        let s_57_16: bool = ((s_57_13) == (s_57_15));
        // D s_57_17: write-var gs#28574 <= s_57_16
        fn_state.gs_28574 = s_57_16;
        // N s_57_18: jump b2
        return block_2(state, tracer, fn_state);
    }
}
