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
use integer_access::*;
use common::*;
pub fn EncodeGPCSC<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gpcf: ProductType396b95aa74979079,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        return_value: u8,
        gs_9892: bool,
        ga_7150: u32,
        ga_7167: u8,
        gpcf: ProductType396b95aa74979079,
    }
    let fn_state = FunctionState {
        gpcf,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_0_0: read-var gpcf.1:struct
        let s_0_0: i128 = fn_state.gpcf._1;
        // C s_0_1: const #0s : i
        let s_0_1: i128 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b12 b1
        if s_0_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_1_0: read-var gpcf.1:struct
        let s_1_0: i128 = fn_state.gpcf._1;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // D s_1_3: write-var gs#9892 <= s_1_2
        fn_state.gs_9892 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var gs#9892:u8
        let s_2_0: bool = fn_state.gs_9892;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var gpcf.0:struct
        let s_2_2: u32 = fn_state.gpcf._0;
        // D s_2_3: write-var ga#7150 <= s_2_2
        fn_state.ga_7150 = s_2_2;
        // C s_2_4: const #1u : u32
        let s_2_4: u32 = 1;
        // D s_2_5: read-var ga#7150:u32
        let s_2_5: u32 = fn_state.ga_7150;
        // D s_2_6: cmp-eq s_2_4 s_2_5
        let s_2_6: bool = ((s_2_4) == (s_2_5));
        // D s_2_7: not s_2_6
        let s_2_7: bool = !s_2_6;
        // N s_2_8: branch s_2_7 b5 b3
        if s_2_7 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_3_0: read-var gpcf.1:struct
        let s_3_0: i128 = fn_state.gpcf._1;
        // C s_3_1: const #0s : i
        let s_3_1: i128 = 0;
        // D s_3_2: call integer_access(s_3_0, s_3_1)
        let s_3_2: bool = integer_access(state, tracer, s_3_0, s_3_1);
        // C s_3_3: const #0s : i
        let s_3_3: i128 = 0;
        // C s_3_4: const #0u : u64
        let s_3_4: u64 = 0;
        // D s_3_5: cast zx s_3_2 -> u64
        let s_3_5: u64 = (s_3_2 as u64);
        // C s_3_6: const #1u : u64
        let s_3_6: u64 = 1;
        // D s_3_7: and s_3_5 s_3_6
        let s_3_7: u64 = ((s_3_5) & (s_3_6));
        // D s_3_8: cmp-eq s_3_7 s_3_6
        let s_3_8: bool = ((s_3_7) == (s_3_6));
        // D s_3_9: lsl s_3_5 s_3_3
        let s_3_9: u64 = s_3_5 << s_3_3;
        // D s_3_10: or s_3_4 s_3_9
        let s_3_10: u64 = ((s_3_4) | (s_3_9));
        // D s_3_11: cmpl s_3_9
        let s_3_11: u64 = !s_3_9;
        // D s_3_12: and s_3_4 s_3_11
        let s_3_12: u64 = ((s_3_4) & (s_3_11));
        // D s_3_13: select s_3_8 s_3_10 s_3_12
        let s_3_13: u64 = if s_3_8 { s_3_10 } else { s_3_12 };
        // D s_3_14: cast trunc s_3_13 -> u8
        let s_3_14: bool = ((s_3_13) != 0);
        // C s_3_15: const #0u : u8
        let s_3_15: u8 = 0;
        // C s_3_16: cast zx s_3_15 -> bv
        let s_3_16: Bits = Bits::new(s_3_15 as u128, 5u16);
        // D s_3_17: cast zx s_3_14 -> bv
        let s_3_17: Bits = Bits::new(s_3_14 as u128, 1u16);
        // C s_3_18: cast reint s_3_16 -> u128
        let s_3_18: u128 = (s_3_16.value() as u128);
        // D s_3_19: size-of s_3_16
        let s_3_19: u16 = s_3_16.length();
        // D s_3_20: cast reint s_3_17 -> u128
        let s_3_20: u128 = (s_3_17.value() as u128);
        // D s_3_21: size-of s_3_17
        let s_3_21: u16 = s_3_17.length();
        // D s_3_22: lsl s_3_18 s_3_21
        let s_3_22: u128 = s_3_18 << s_3_21;
        // D s_3_23: or s_3_22 s_3_20
        let s_3_23: u128 = ((s_3_22) | (s_3_20));
        // D s_3_24: add s_3_19 s_3_21
        let s_3_24: u16 = (s_3_19 + s_3_21);
        // D s_3_25: create-bits s_3_23 s_3_24
        let s_3_25: Bits = Bits::new(s_3_23, s_3_24);
        // D s_3_26: cast reint s_3_25 -> u8
        let s_3_26: u8 = (s_3_25.value() as u8);
        // D s_3_27: write-var return_value <= s_3_26
        fn_state.return_value = s_3_26;
        // N s_3_28: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_4_0: read-var return_value:u8
        let s_4_0: u8 = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_5_0: const #2u : u32
        let s_5_0: u32 = 2;
        // D s_5_1: read-var ga#7150:u32
        let s_5_1: u32 = fn_state.ga_7150;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_6_0: read-var gpcf.1:struct
        let s_6_0: i128 = fn_state.gpcf._1;
        // C s_6_1: const #0s : i
        let s_6_1: i128 = 0;
        // D s_6_2: call integer_access(s_6_0, s_6_1)
        let s_6_2: bool = integer_access(state, tracer, s_6_0, s_6_1);
        // C s_6_3: const #0s : i
        let s_6_3: i128 = 0;
        // C s_6_4: const #0u : u64
        let s_6_4: u64 = 0;
        // D s_6_5: cast zx s_6_2 -> u64
        let s_6_5: u64 = (s_6_2 as u64);
        // C s_6_6: const #1u : u64
        let s_6_6: u64 = 1;
        // D s_6_7: and s_6_5 s_6_6
        let s_6_7: u64 = ((s_6_5) & (s_6_6));
        // D s_6_8: cmp-eq s_6_7 s_6_6
        let s_6_8: bool = ((s_6_7) == (s_6_6));
        // D s_6_9: lsl s_6_5 s_6_3
        let s_6_9: u64 = s_6_5 << s_6_3;
        // D s_6_10: or s_6_4 s_6_9
        let s_6_10: u64 = ((s_6_4) | (s_6_9));
        // D s_6_11: cmpl s_6_9
        let s_6_11: u64 = !s_6_9;
        // D s_6_12: and s_6_4 s_6_11
        let s_6_12: u64 = ((s_6_4) & (s_6_11));
        // D s_6_13: select s_6_8 s_6_10 s_6_12
        let s_6_13: u64 = if s_6_8 { s_6_10 } else { s_6_12 };
        // D s_6_14: cast trunc s_6_13 -> u8
        let s_6_14: bool = ((s_6_13) != 0);
        // C s_6_15: const #2u : u8
        let s_6_15: u8 = 2;
        // C s_6_16: cast zx s_6_15 -> bv
        let s_6_16: Bits = Bits::new(s_6_15 as u128, 5u16);
        // D s_6_17: cast zx s_6_14 -> bv
        let s_6_17: Bits = Bits::new(s_6_14 as u128, 1u16);
        // C s_6_18: cast reint s_6_16 -> u128
        let s_6_18: u128 = (s_6_16.value() as u128);
        // D s_6_19: size-of s_6_16
        let s_6_19: u16 = s_6_16.length();
        // D s_6_20: cast reint s_6_17 -> u128
        let s_6_20: u128 = (s_6_17.value() as u128);
        // D s_6_21: size-of s_6_17
        let s_6_21: u16 = s_6_17.length();
        // D s_6_22: lsl s_6_18 s_6_21
        let s_6_22: u128 = s_6_18 << s_6_21;
        // D s_6_23: or s_6_22 s_6_20
        let s_6_23: u128 = ((s_6_22) | (s_6_20));
        // D s_6_24: add s_6_19 s_6_21
        let s_6_24: u16 = (s_6_19 + s_6_21);
        // D s_6_25: create-bits s_6_23 s_6_24
        let s_6_25: Bits = Bits::new(s_6_23, s_6_24);
        // D s_6_26: cast reint s_6_25 -> u8
        let s_6_26: u8 = (s_6_25.value() as u8);
        // D s_6_27: write-var return_value <= s_6_26
        fn_state.return_value = s_6_26;
        // N s_6_28: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_7_0: const #4u : u32
        let s_7_0: u32 = 4;
        // D s_7_1: read-var ga#7150:u32
        let s_7_1: u32 = fn_state.ga_7150;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_8_0: read-var gpcf.1:struct
        let s_8_0: i128 = fn_state.gpcf._1;
        // C s_8_1: const #0s : i
        let s_8_1: i128 = 0;
        // D s_8_2: call integer_access(s_8_0, s_8_1)
        let s_8_2: bool = integer_access(state, tracer, s_8_0, s_8_1);
        // C s_8_3: const #0s : i
        let s_8_3: i128 = 0;
        // C s_8_4: const #0u : u64
        let s_8_4: u64 = 0;
        // D s_8_5: cast zx s_8_2 -> u64
        let s_8_5: u64 = (s_8_2 as u64);
        // C s_8_6: const #1u : u64
        let s_8_6: u64 = 1;
        // D s_8_7: and s_8_5 s_8_6
        let s_8_7: u64 = ((s_8_5) & (s_8_6));
        // D s_8_8: cmp-eq s_8_7 s_8_6
        let s_8_8: bool = ((s_8_7) == (s_8_6));
        // D s_8_9: lsl s_8_5 s_8_3
        let s_8_9: u64 = s_8_5 << s_8_3;
        // D s_8_10: or s_8_4 s_8_9
        let s_8_10: u64 = ((s_8_4) | (s_8_9));
        // D s_8_11: cmpl s_8_9
        let s_8_11: u64 = !s_8_9;
        // D s_8_12: and s_8_4 s_8_11
        let s_8_12: u64 = ((s_8_4) & (s_8_11));
        // D s_8_13: select s_8_8 s_8_10 s_8_12
        let s_8_13: u64 = if s_8_8 { s_8_10 } else { s_8_12 };
        // D s_8_14: cast trunc s_8_13 -> u8
        let s_8_14: bool = ((s_8_13) != 0);
        // C s_8_15: const #6u : u8
        let s_8_15: u8 = 6;
        // C s_8_16: cast zx s_8_15 -> bv
        let s_8_16: Bits = Bits::new(s_8_15 as u128, 5u16);
        // D s_8_17: cast zx s_8_14 -> bv
        let s_8_17: Bits = Bits::new(s_8_14 as u128, 1u16);
        // C s_8_18: cast reint s_8_16 -> u128
        let s_8_18: u128 = (s_8_16.value() as u128);
        // D s_8_19: size-of s_8_16
        let s_8_19: u16 = s_8_16.length();
        // D s_8_20: cast reint s_8_17 -> u128
        let s_8_20: u128 = (s_8_17.value() as u128);
        // D s_8_21: size-of s_8_17
        let s_8_21: u16 = s_8_17.length();
        // D s_8_22: lsl s_8_18 s_8_21
        let s_8_22: u128 = s_8_18 << s_8_21;
        // D s_8_23: or s_8_22 s_8_20
        let s_8_23: u128 = ((s_8_22) | (s_8_20));
        // D s_8_24: add s_8_19 s_8_21
        let s_8_24: u16 = (s_8_19 + s_8_21);
        // D s_8_25: create-bits s_8_23 s_8_24
        let s_8_25: Bits = Bits::new(s_8_23, s_8_24);
        // D s_8_26: cast reint s_8_25 -> u8
        let s_8_26: u8 = (s_8_25.value() as u8);
        // D s_8_27: write-var return_value <= s_8_26
        fn_state.return_value = s_8_26;
        // N s_8_28: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_9_0: const #3u : u32
        let s_9_0: u32 = 3;
        // D s_9_1: read-var ga#7150:u32
        let s_9_1: u32 = fn_state.ga_7150;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b11 b10
        if s_9_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_10_0: read-var gpcf.1:struct
        let s_10_0: i128 = fn_state.gpcf._1;
        // C s_10_1: const #0s : i
        let s_10_1: i128 = 0;
        // D s_10_2: call integer_access(s_10_0, s_10_1)
        let s_10_2: bool = integer_access(state, tracer, s_10_0, s_10_1);
        // C s_10_3: const #0s : i
        let s_10_3: i128 = 0;
        // C s_10_4: const #0u : u64
        let s_10_4: u64 = 0;
        // D s_10_5: cast zx s_10_2 -> u64
        let s_10_5: u64 = (s_10_2 as u64);
        // C s_10_6: const #1u : u64
        let s_10_6: u64 = 1;
        // D s_10_7: and s_10_5 s_10_6
        let s_10_7: u64 = ((s_10_5) & (s_10_6));
        // D s_10_8: cmp-eq s_10_7 s_10_6
        let s_10_8: bool = ((s_10_7) == (s_10_6));
        // D s_10_9: lsl s_10_5 s_10_3
        let s_10_9: u64 = s_10_5 << s_10_3;
        // D s_10_10: or s_10_4 s_10_9
        let s_10_10: u64 = ((s_10_4) | (s_10_9));
        // D s_10_11: cmpl s_10_9
        let s_10_11: u64 = !s_10_9;
        // D s_10_12: and s_10_4 s_10_11
        let s_10_12: u64 = ((s_10_4) & (s_10_11));
        // D s_10_13: select s_10_8 s_10_10 s_10_12
        let s_10_13: u64 = if s_10_8 { s_10_10 } else { s_10_12 };
        // D s_10_14: cast trunc s_10_13 -> u8
        let s_10_14: bool = ((s_10_13) != 0);
        // C s_10_15: const #10u : u8
        let s_10_15: u8 = 10;
        // C s_10_16: cast zx s_10_15 -> bv
        let s_10_16: Bits = Bits::new(s_10_15 as u128, 5u16);
        // D s_10_17: cast zx s_10_14 -> bv
        let s_10_17: Bits = Bits::new(s_10_14 as u128, 1u16);
        // C s_10_18: cast reint s_10_16 -> u128
        let s_10_18: u128 = (s_10_16.value() as u128);
        // D s_10_19: size-of s_10_16
        let s_10_19: u16 = s_10_16.length();
        // D s_10_20: cast reint s_10_17 -> u128
        let s_10_20: u128 = (s_10_17.value() as u128);
        // D s_10_21: size-of s_10_17
        let s_10_21: u16 = s_10_17.length();
        // D s_10_22: lsl s_10_18 s_10_21
        let s_10_22: u128 = s_10_18 << s_10_21;
        // D s_10_23: or s_10_22 s_10_20
        let s_10_23: u128 = ((s_10_22) | (s_10_20));
        // D s_10_24: add s_10_19 s_10_21
        let s_10_24: u16 = (s_10_19 + s_10_21);
        // D s_10_25: create-bits s_10_23 s_10_24
        let s_10_25: Bits = Bits::new(s_10_23, s_10_24);
        // D s_10_26: cast reint s_10_25 -> u8
        let s_10_26: u8 = (s_10_25.value() as u8);
        // D s_10_27: write-var return_value <= s_10_26
        fn_state.return_value = s_10_26;
        // N s_10_28: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_11_0: read-var ga#7167:u8
        let s_11_0: u8 = fn_state.ga_7167;
        // D s_11_1: write-var return_value <= s_11_0
        fn_state.return_value = s_11_0;
        // N s_11_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#9892 <= s_12_0
        fn_state.gs_9892 = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
