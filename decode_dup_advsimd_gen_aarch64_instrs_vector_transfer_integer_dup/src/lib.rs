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
use u__id::*;
use LowestSetBit::*;
use u_shl_int_general::*;
use execute_aarch64_instrs_vector_transfer_integer_dup::*;
use fdiv_int::*;
use common::*;
pub fn decode_dup_advsimd_gen_aarch64_instrs_vector_transfer_integer_dup<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    imm5: u8,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_148060: bool,
        gs_148058: bool,
        gs_148047: bool,
        esize: i128,
        n: i64,
        gs_148066: bool,
        d: i64,
        size: i128,
        elements: i128,
        datasize: i64,
        gs_148064: bool,
        ga_252524: i64,
        gs_148062: bool,
        Rd: u8,
        Rn: u8,
        imm5: u8,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        imm5,
        Q,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var d <= s_0_3
        fn_state.d = s_0_3;
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // D s_0_10: read-var imm5:u8
        let s_0_10: u8 = fn_state.imm5;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: call LowestSetBit(s_0_11)
        let s_0_12: i128 = LowestSetBit(state, tracer, s_0_11);
        // D s_0_13: write-var size <= s_0_12
        fn_state.size = s_0_12;
        // C s_0_14: const #3s : i
        let s_0_14: i128 = 3;
        // D s_0_15: read-var size:i
        let s_0_15: i128 = fn_state.size;
        // D s_0_16: cmp-gt s_0_15 s_0_14
        let s_0_16: bool = ((s_0_15) > (s_0_14));
        // N s_0_17: branch s_0_16 b25 b1
        if s_0_16 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #3s : i
        let s_1_0: i128 = 3;
        // D s_1_1: read-var size:i
        let s_1_1: i128 = fn_state.size;
        // D s_1_2: cmp-eq s_1_1 s_1_0
        let s_1_2: bool = ((s_1_1) == (s_1_0));
        // N s_1_3: branch s_1_2 b24 b2
        if s_1_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#148047 <= s_2_0
        fn_state.gs_148047 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#148047:u8
        let s_3_0: bool = fn_state.gs_148047;
        // N s_3_1: branch s_3_0 b23 b4
        if s_3_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #8s : i
        let s_4_0: i128 = 8;
        // D s_4_1: read-var size:i
        let s_4_1: i128 = fn_state.size;
        // D s_4_2: call _shl_int_general(s_4_0, s_4_1)
        let s_4_2: i128 = u_shl_int_general(state, tracer, s_4_0, s_4_1);
        // D s_4_3: write-var esize <= s_4_2
        fn_state.esize = s_4_2;
        // D s_4_4: read-var Q:u8
        let s_4_4: bool = fn_state.Q;
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // C s_4_6: const #1u : u8
        let s_4_6: bool = true;
        // C s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 1u16);
        // D s_4_8: cmp-eq s_4_5 s_4_7
        let s_4_8: bool = ((s_4_5) == (s_4_7));
        // N s_4_9: branch s_4_8 b22 b5
        if s_4_8 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: write-var ga#252524 <= s_5_0
        fn_state.ga_252524 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#252524:i64
        let s_6_0: i64 = fn_state.ga_252524;
        // D s_6_1: write-var datasize <= s_6_0
        fn_state.datasize = s_6_0;
        // D s_6_2: read-var datasize:i64
        let s_6_2: i64 = fn_state.datasize;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-var esize:i
        let s_6_4: i128 = fn_state.esize;
        // D s_6_5: call fdiv_int(s_6_3, s_6_4)
        let s_6_5: i128 = fdiv_int(state, tracer, s_6_3, s_6_4);
        // D s_6_6: write-var elements <= s_6_5
        fn_state.elements = s_6_5;
        // D s_6_7: read-var esize:i
        let s_6_7: i128 = fn_state.esize;
        // D s_6_8: call __id(s_6_7)
        let s_6_8: i128 = u__id(state, tracer, s_6_7);
        // C s_6_9: const #8s : i
        let s_6_9: i128 = 8;
        // D s_6_10: cmp-eq s_6_8 s_6_9
        let s_6_10: bool = ((s_6_8) == (s_6_9));
        // N s_6_11: branch s_6_10 b21 b7
        if s_6_10 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esize:i
        let s_7_0: i128 = fn_state.esize;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // C s_7_2: const #16s : i
        let s_7_2: i128 = 16;
        // D s_7_3: cmp-eq s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) == (s_7_2));
        // D s_7_4: write-var gs#148058 <= s_7_3
        fn_state.gs_148058 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#148058:u8
        let s_8_0: bool = fn_state.gs_148058;
        // N s_8_1: branch s_8_0 b20 b9
        if s_8_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esize:i
        let s_9_0: i128 = fn_state.esize;
        // D s_9_1: call __id(s_9_0)
        let s_9_1: i128 = u__id(state, tracer, s_9_0);
        // C s_9_2: const #32s : i
        let s_9_2: i128 = 32;
        // D s_9_3: cmp-eq s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) == (s_9_2));
        // D s_9_4: write-var gs#148060 <= s_9_3
        fn_state.gs_148060 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#148060:u8
        let s_10_0: bool = fn_state.gs_148060;
        // N s_10_1: branch s_10_0 b19 b11
        if s_10_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esize:i
        let s_11_0: i128 = fn_state.esize;
        // D s_11_1: call __id(s_11_0)
        let s_11_1: i128 = u__id(state, tracer, s_11_0);
        // C s_11_2: const #64s : i
        let s_11_2: i128 = 64;
        // D s_11_3: cmp-eq s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) == (s_11_2));
        // D s_11_4: write-var gs#148062 <= s_11_3
        fn_state.gs_148062 = s_11_3;
        // N s_11_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#148062:u8
        let s_12_0: bool = fn_state.gs_148062;
        // N s_12_1: branch s_12_0 b18 b13
        if s_12_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esize:i
        let s_13_0: i128 = fn_state.esize;
        // D s_13_1: call __id(s_13_0)
        let s_13_1: i128 = u__id(state, tracer, s_13_0);
        // C s_13_2: const #128s : i
        let s_13_2: i128 = 128;
        // D s_13_3: cmp-eq s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) == (s_13_2));
        // D s_13_4: write-var gs#148064 <= s_13_3
        fn_state.gs_148064 = s_13_3;
        // N s_13_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#148064:u8
        let s_14_0: bool = fn_state.gs_148064;
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
    ) -> () {
        // D s_15_0: read-var esize:i
        let s_15_0: i128 = fn_state.esize;
        // D s_15_1: call __id(s_15_0)
        let s_15_1: i128 = u__id(state, tracer, s_15_0);
        // C s_15_2: const #256s : i
        let s_15_2: i128 = 256;
        // D s_15_3: cmp-eq s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) == (s_15_2));
        // D s_15_4: write-var gs#148066 <= s_15_3
        fn_state.gs_148066 = s_15_3;
        // N s_15_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#148066:u8
        let s_16_0: bool = fn_state.gs_148066;
        // N s_16_1: assert s_16_0
        let s_16_1: () = assert!(s_16_0);
        // D s_16_2: read-var datasize:i64
        let s_16_2: i64 = fn_state.datasize;
        // D s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_4: cast reint s_16_3 -> i64
        let s_16_4: i64 = (s_16_3 as i64);
        // D s_16_5: read-var esize:i
        let s_16_5: i128 = fn_state.esize;
        // D s_16_6: cast reint s_16_5 -> i64
        let s_16_6: i64 = (s_16_5 as i64);
        // D s_16_7: read-var d:i64
        let s_16_7: i64 = fn_state.d;
        // D s_16_8: read-var elements:i
        let s_16_8: i128 = fn_state.elements;
        // D s_16_9: read-var n:i64
        let s_16_9: i64 = fn_state.n;
        // D s_16_10: call execute_aarch64_instrs_vector_transfer_integer_dup(s_16_7, s_16_4, s_16_8, s_16_6, s_16_9)
        let s_16_10: () = execute_aarch64_instrs_vector_transfer_integer_dup(
            state,
            tracer,
            s_16_7,
            s_16_4,
            s_16_8,
            s_16_6,
            s_16_9,
        );
        // N s_16_11: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#148066 <= s_17_0
        fn_state.gs_148066 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#148064 <= s_18_0
        fn_state.gs_148064 = s_18_0;
        // N s_18_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#148062 <= s_19_0
        fn_state.gs_148062 = s_19_0;
        // N s_19_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#148060 <= s_20_0
        fn_state.gs_148060 = s_20_0;
        // N s_20_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#148058 <= s_21_0
        fn_state.gs_148058 = s_21_0;
        // N s_21_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #128s : i64
        let s_22_0: i64 = 128;
        // D s_22_1: write-var ga#252524 <= s_22_0
        fn_state.ga_252524 = s_22_0;
        // N s_22_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var Q:u8
        let s_24_0: bool = fn_state.Q;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #0u : u8
        let s_24_2: bool = false;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#148047 <= s_24_4
        fn_state.gs_148047 = s_24_4;
        // N s_24_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
}
