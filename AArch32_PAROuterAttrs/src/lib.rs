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
use u__UNKNOWN_bits::*;
use common::*;
pub fn AArch32_PAROuterAttrs<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memattrs: ProductTypef170cab34335b70c,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        gs_30402: bool,
        gs_30405: bool,
        result: u8,
        gs_30399: bool,
        outer: ProductType183e6678e5239c85,
        memattrs: ProductTypef170cab34335b70c,
    }
    let fn_state = FunctionState {
        memattrs,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_0_0: read-var memattrs.2:struct
        let s_0_0: u32 = fn_state.memattrs._2;
        // C s_0_1: const #1u : u32
        let s_0_1: u32 = 1;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b20 b1
        if s_0_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_1_0: read-var memattrs.4:struct
        let s_1_0: ProductType183e6678e5239c85 = fn_state.memattrs._4;
        // D s_1_1: write-var outer <= s_1_0
        fn_state.outer = s_1_0;
        // D s_1_2: read-var outer.0:struct
        let s_1_2: u8 = fn_state.outer._0;
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // C s_1_4: const #464u : u32
        let s_1_4: u32 = 464;
        // D s_1_5: read-reg s_1_4:u8
        let s_1_5: u8 = {
            let value = state.read_register::<u8>(s_1_4 as isize);
            tracer.read_register(s_1_4 as isize, value);
            value
        };
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 2u16);
        // D s_1_7: cmp-eq s_1_3 s_1_6
        let s_1_7: bool = ((s_1_3) == (s_1_6));
        // N s_1_8: branch s_1_7 b19 b2
        if s_1_7 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var outer.0:struct
        let s_2_0: u8 = fn_state.outer._0;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #480u : u32
        let s_2_2: u32 = 480;
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
        // N s_2_6: branch s_2_5 b18 b3
        if s_2_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#30399 <= s_3_0
        fn_state.gs_30399 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_4_0: read-var gs#30399:u8
        let s_4_0: bool = fn_state.gs_30399;
        // N s_4_1: branch s_4_0 b17 b5
        if s_4_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_5_0: read-var outer.0:struct
        let s_5_0: u8 = fn_state.outer._0;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #472u : u32
        let s_5_2: u32 = 472;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // N s_5_6: branch s_5_5 b16 b6
        if s_5_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#30402 <= s_6_0
        fn_state.gs_30402 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_7_0: read-var gs#30402:u8
        let s_7_0: bool = fn_state.gs_30402;
        // N s_7_1: branch s_7_0 b15 b8
        if s_7_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_8_0: read-var outer.0:struct
        let s_8_0: u8 = fn_state.outer._0;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #480u : u32
        let s_8_2: u32 = 480;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 2u16);
        // D s_8_5: cmp-eq s_8_1 s_8_4
        let s_8_5: bool = ((s_8_1) == (s_8_4));
        // N s_8_6: branch s_8_5 b14 b9
        if s_8_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#30405 <= s_9_0
        fn_state.gs_30405 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_10_0: read-var gs#30405:u8
        let s_10_0: bool = fn_state.gs_30405;
        // N s_10_1: branch s_10_0 b13 b11
        if s_10_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_12_0: read-var result:u8
        let s_12_0: u8 = fn_state.result;
        // N s_12_1: return s_12_0
        return s_12_0;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_13_0: const #3u : u8
        let s_13_0: u8 = 3;
        // D s_13_1: write-var result <= s_13_0
        fn_state.result = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_14_0: read-var outer.1:struct
        let s_14_0: u8 = fn_state.outer._1;
        // C s_14_1: const #0s : i
        let s_14_1: i128 = 0;
        // D s_14_2: cast zx s_14_0 -> bv
        let s_14_2: Bits = Bits::new(s_14_0 as u128, 2u16);
        // C s_14_3: const #1u : u64
        let s_14_3: u64 = 1;
        // D s_14_4: bit-extract s_14_2 s_14_1 s_14_3
        let s_14_4: Bits = (Bits::new(
            ((s_14_2) >> (s_14_1)).value(),
            u16::try_from(s_14_3).unwrap(),
        ));
        // D s_14_5: cast reint s_14_4 -> u8
        let s_14_5: bool = ((s_14_4.value()) != 0);
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // C s_14_7: const #0u : u64
        let s_14_7: u64 = 0;
        // D s_14_8: cast zx s_14_5 -> u64
        let s_14_8: u64 = (s_14_5 as u64);
        // C s_14_9: const #1u : u64
        let s_14_9: u64 = 1;
        // D s_14_10: and s_14_8 s_14_9
        let s_14_10: u64 = ((s_14_8) & (s_14_9));
        // D s_14_11: cmp-eq s_14_10 s_14_9
        let s_14_11: bool = ((s_14_10) == (s_14_9));
        // D s_14_12: lsl s_14_8 s_14_6
        let s_14_12: u64 = s_14_8 << s_14_6;
        // D s_14_13: or s_14_7 s_14_12
        let s_14_13: u64 = ((s_14_7) | (s_14_12));
        // D s_14_14: cmpl s_14_12
        let s_14_14: u64 = !s_14_12;
        // D s_14_15: and s_14_7 s_14_14
        let s_14_15: u64 = ((s_14_7) & (s_14_14));
        // D s_14_16: select s_14_11 s_14_13 s_14_15
        let s_14_16: u64 = if s_14_11 { s_14_13 } else { s_14_15 };
        // D s_14_17: cast trunc s_14_16 -> u8
        let s_14_17: bool = ((s_14_16) != 0);
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 1u16);
        // C s_14_19: const #0u : u8
        let s_14_19: bool = false;
        // C s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 1u16);
        // D s_14_21: cmp-eq s_14_18 s_14_20
        let s_14_21: bool = ((s_14_18) == (s_14_20));
        // D s_14_22: write-var gs#30405 <= s_14_21
        fn_state.gs_30405 = s_14_21;
        // N s_14_23: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_15_0: const #2u : u8
        let s_15_0: u8 = 2;
        // D s_15_1: write-var result <= s_15_0
        fn_state.result = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_16_0: read-var outer.1:struct
        let s_16_0: u8 = fn_state.outer._1;
        // C s_16_1: const #0s : i
        let s_16_1: i128 = 0;
        // D s_16_2: cast zx s_16_0 -> bv
        let s_16_2: Bits = Bits::new(s_16_0 as u128, 2u16);
        // C s_16_3: const #1u : u64
        let s_16_3: u64 = 1;
        // D s_16_4: bit-extract s_16_2 s_16_1 s_16_3
        let s_16_4: Bits = (Bits::new(
            ((s_16_2) >> (s_16_1)).value(),
            u16::try_from(s_16_3).unwrap(),
        ));
        // D s_16_5: cast reint s_16_4 -> u8
        let s_16_5: bool = ((s_16_4.value()) != 0);
        // C s_16_6: const #0s : i
        let s_16_6: i128 = 0;
        // C s_16_7: const #0u : u64
        let s_16_7: u64 = 0;
        // D s_16_8: cast zx s_16_5 -> u64
        let s_16_8: u64 = (s_16_5 as u64);
        // C s_16_9: const #1u : u64
        let s_16_9: u64 = 1;
        // D s_16_10: and s_16_8 s_16_9
        let s_16_10: u64 = ((s_16_8) & (s_16_9));
        // D s_16_11: cmp-eq s_16_10 s_16_9
        let s_16_11: bool = ((s_16_10) == (s_16_9));
        // D s_16_12: lsl s_16_8 s_16_6
        let s_16_12: u64 = s_16_8 << s_16_6;
        // D s_16_13: or s_16_7 s_16_12
        let s_16_13: u64 = ((s_16_7) | (s_16_12));
        // D s_16_14: cmpl s_16_12
        let s_16_14: u64 = !s_16_12;
        // D s_16_15: and s_16_7 s_16_14
        let s_16_15: u64 = ((s_16_7) & (s_16_14));
        // D s_16_16: select s_16_11 s_16_13 s_16_15
        let s_16_16: u64 = if s_16_11 { s_16_13 } else { s_16_15 };
        // D s_16_17: cast trunc s_16_16 -> u8
        let s_16_17: bool = ((s_16_16) != 0);
        // D s_16_18: cast zx s_16_17 -> bv
        let s_16_18: Bits = Bits::new(s_16_17 as u128, 1u16);
        // C s_16_19: const #0u : u8
        let s_16_19: bool = false;
        // C s_16_20: cast zx s_16_19 -> bv
        let s_16_20: Bits = Bits::new(s_16_19 as u128, 1u16);
        // D s_16_21: cmp-eq s_16_18 s_16_20
        let s_16_21: bool = ((s_16_18) == (s_16_20));
        // D s_16_22: write-var gs#30402 <= s_16_21
        fn_state.gs_30402 = s_16_21;
        // N s_16_23: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_17_0: const #1u : u8
        let s_17_0: u8 = 1;
        // D s_17_1: write-var result <= s_17_0
        fn_state.result = s_17_0;
        // N s_17_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_18_0: read-var outer.1:struct
        let s_18_0: u8 = fn_state.outer._1;
        // C s_18_1: const #0s : i
        let s_18_1: i128 = 0;
        // D s_18_2: cast zx s_18_0 -> bv
        let s_18_2: Bits = Bits::new(s_18_0 as u128, 2u16);
        // C s_18_3: const #1u : u64
        let s_18_3: u64 = 1;
        // D s_18_4: bit-extract s_18_2 s_18_1 s_18_3
        let s_18_4: Bits = (Bits::new(
            ((s_18_2) >> (s_18_1)).value(),
            u16::try_from(s_18_3).unwrap(),
        ));
        // D s_18_5: cast reint s_18_4 -> u8
        let s_18_5: bool = ((s_18_4.value()) != 0);
        // C s_18_6: const #0s : i
        let s_18_6: i128 = 0;
        // C s_18_7: const #0u : u64
        let s_18_7: u64 = 0;
        // D s_18_8: cast zx s_18_5 -> u64
        let s_18_8: u64 = (s_18_5 as u64);
        // C s_18_9: const #1u : u64
        let s_18_9: u64 = 1;
        // D s_18_10: and s_18_8 s_18_9
        let s_18_10: u64 = ((s_18_8) & (s_18_9));
        // D s_18_11: cmp-eq s_18_10 s_18_9
        let s_18_11: bool = ((s_18_10) == (s_18_9));
        // D s_18_12: lsl s_18_8 s_18_6
        let s_18_12: u64 = s_18_8 << s_18_6;
        // D s_18_13: or s_18_7 s_18_12
        let s_18_13: u64 = ((s_18_7) | (s_18_12));
        // D s_18_14: cmpl s_18_12
        let s_18_14: u64 = !s_18_12;
        // D s_18_15: and s_18_7 s_18_14
        let s_18_15: u64 = ((s_18_7) & (s_18_14));
        // D s_18_16: select s_18_11 s_18_13 s_18_15
        let s_18_16: u64 = if s_18_11 { s_18_13 } else { s_18_15 };
        // D s_18_17: cast trunc s_18_16 -> u8
        let s_18_17: bool = ((s_18_16) != 0);
        // D s_18_18: cast zx s_18_17 -> bv
        let s_18_18: Bits = Bits::new(s_18_17 as u128, 1u16);
        // C s_18_19: const #1u : u8
        let s_18_19: bool = true;
        // C s_18_20: cast zx s_18_19 -> bv
        let s_18_20: Bits = Bits::new(s_18_19 as u128, 1u16);
        // D s_18_21: cmp-eq s_18_18 s_18_20
        let s_18_21: bool = ((s_18_18) == (s_18_20));
        // D s_18_22: write-var gs#30399 <= s_18_21
        fn_state.gs_30399 = s_18_21;
        // N s_18_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_19_0: const #0u : u8
        let s_19_0: u8 = 0;
        // D s_19_1: write-var result <= s_19_0
        fn_state.result = s_19_0;
        // N s_19_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_20_0: const #2s : i64
        let s_20_0: i64 = 2;
        // C s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // S s_20_2: call __UNKNOWN_bits(s_20_1)
        let s_20_2: Bits = u__UNKNOWN_bits(state, tracer, s_20_1);
        // S s_20_3: cast reint s_20_2 -> u8
        let s_20_3: u8 = (s_20_2.value() as u8);
        // D s_20_4: write-var result <= s_20_3
        fn_state.result = s_20_3;
        // N s_20_5: jump b12
        return block_12(state, tracer, fn_state);
    }
}
