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
use unsigned_subrange::*;
use execute_aarch64_instrs_vector_transfer_integer_insert::*;
use u__id::*;
use LowestSetBit::*;
use u_shl_int_general::*;
use common::*;
pub fn decode_ins_advsimd_gen_aarch64_instrs_vector_transfer_integer_insert<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    imm5: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i128,
        gs_155156: bool,
        n: i64,
        index: i64,
        gs_155150: bool,
        d: i64,
        size: i128,
        gs_155158: bool,
        gs_155152: bool,
        gs_155154: bool,
        Rd: u8,
        Rn: u8,
        imm5: u8,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        imm5,
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
        // N s_0_17: branch s_0_16 b17 b1
        if s_0_16 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:i
        let s_1_0: i128 = fn_state.size;
        // D s_1_1: call __id(s_1_0)
        let s_1_1: i128 = u__id(state, tracer, s_1_0);
        // C s_1_2: const #1s : i
        let s_1_2: i128 = 1;
        // D s_1_3: add s_1_1 s_1_2
        let s_1_3: i128 = (s_1_1 + s_1_2);
        // C s_1_4: const #0s : i
        let s_1_4: i128 = 0;
        // D s_1_5: cmp-le s_1_4 s_1_3
        let s_1_5: bool = ((s_1_4) <= (s_1_3));
        // N s_1_6: assert s_1_5
        let s_1_6: () = assert!(s_1_5);
        // C s_1_7: const #1s : i
        let s_1_7: i128 = 1;
        // D s_1_8: read-var size:i
        let s_1_8: i128 = fn_state.size;
        // D s_1_9: add s_1_8 s_1_7
        let s_1_9: i128 = (s_1_8 + s_1_7);
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // C s_1_11: const #4s : i
        let s_1_11: i128 = 4;
        // D s_1_12: read-var imm5:u8
        let s_1_12: u8 = fn_state.imm5;
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 5u16);
        // D s_1_14: cast zx s_1_10 -> i
        let s_1_14: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_15: call unsigned_subrange(s_1_13, s_1_11, s_1_14)
        let s_1_15: i128 = unsigned_subrange(state, tracer, s_1_13, s_1_11, s_1_14);
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: write-var index <= s_1_16
        fn_state.index = s_1_16;
        // C s_1_18: const #8s : i
        let s_1_18: i128 = 8;
        // D s_1_19: read-var size:i
        let s_1_19: i128 = fn_state.size;
        // D s_1_20: call _shl_int_general(s_1_18, s_1_19)
        let s_1_20: i128 = u_shl_int_general(state, tracer, s_1_18, s_1_19);
        // D s_1_21: write-var esize <= s_1_20
        fn_state.esize = s_1_20;
        // D s_1_22: read-var esize:i
        let s_1_22: i128 = fn_state.esize;
        // D s_1_23: call __id(s_1_22)
        let s_1_23: i128 = u__id(state, tracer, s_1_22);
        // C s_1_24: const #8s : i
        let s_1_24: i128 = 8;
        // D s_1_25: cmp-eq s_1_23 s_1_24
        let s_1_25: bool = ((s_1_23) == (s_1_24));
        // N s_1_26: branch s_1_25 b16 b2
        if s_1_25 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var esize:i
        let s_2_0: i128 = fn_state.esize;
        // D s_2_1: call __id(s_2_0)
        let s_2_1: i128 = u__id(state, tracer, s_2_0);
        // C s_2_2: const #16s : i
        let s_2_2: i128 = 16;
        // D s_2_3: cmp-eq s_2_1 s_2_2
        let s_2_3: bool = ((s_2_1) == (s_2_2));
        // D s_2_4: write-var gs#155150 <= s_2_3
        fn_state.gs_155150 = s_2_3;
        // N s_2_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#155150:u8
        let s_3_0: bool = fn_state.gs_155150;
        // N s_3_1: branch s_3_0 b15 b4
        if s_3_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esize:i
        let s_4_0: i128 = fn_state.esize;
        // D s_4_1: call __id(s_4_0)
        let s_4_1: i128 = u__id(state, tracer, s_4_0);
        // C s_4_2: const #32s : i
        let s_4_2: i128 = 32;
        // D s_4_3: cmp-eq s_4_1 s_4_2
        let s_4_3: bool = ((s_4_1) == (s_4_2));
        // D s_4_4: write-var gs#155152 <= s_4_3
        fn_state.gs_155152 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#155152:u8
        let s_5_0: bool = fn_state.gs_155152;
        // N s_5_1: branch s_5_0 b14 b6
        if s_5_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esize:i
        let s_6_0: i128 = fn_state.esize;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // C s_6_2: const #64s : i
        let s_6_2: i128 = 64;
        // D s_6_3: cmp-eq s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) == (s_6_2));
        // D s_6_4: write-var gs#155154 <= s_6_3
        fn_state.gs_155154 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#155154:u8
        let s_7_0: bool = fn_state.gs_155154;
        // N s_7_1: branch s_7_0 b13 b8
        if s_7_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i
        let s_8_0: i128 = fn_state.esize;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // C s_8_2: const #128s : i
        let s_8_2: i128 = 128;
        // D s_8_3: cmp-eq s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) == (s_8_2));
        // D s_8_4: write-var gs#155156 <= s_8_3
        fn_state.gs_155156 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#155156:u8
        let s_9_0: bool = fn_state.gs_155156;
        // N s_9_1: branch s_9_0 b12 b10
        if s_9_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esize:i
        let s_10_0: i128 = fn_state.esize;
        // D s_10_1: call __id(s_10_0)
        let s_10_1: i128 = u__id(state, tracer, s_10_0);
        // C s_10_2: const #256s : i
        let s_10_2: i128 = 256;
        // D s_10_3: cmp-eq s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) == (s_10_2));
        // D s_10_4: write-var gs#155158 <= s_10_3
        fn_state.gs_155158 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#155158:u8
        let s_11_0: bool = fn_state.gs_155158;
        // N s_11_1: assert s_11_0
        let s_11_1: () = assert!(s_11_0);
        // D s_11_2: read-var esize:i
        let s_11_2: i128 = fn_state.esize;
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #128s : i64
        let s_11_4: i64 = 128;
        // D s_11_5: read-var index:i64
        let s_11_5: i64 = fn_state.index;
        // D s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_7: read-var d:i64
        let s_11_7: i64 = fn_state.d;
        // D s_11_8: read-var n:i64
        let s_11_8: i64 = fn_state.n;
        // D s_11_9: call execute_aarch64_instrs_vector_transfer_integer_insert(s_11_7, s_11_4, s_11_3, s_11_6, s_11_8)
        let s_11_9: () = execute_aarch64_instrs_vector_transfer_integer_insert(
            state,
            tracer,
            s_11_7,
            s_11_4,
            s_11_3,
            s_11_6,
            s_11_8,
        );
        // N s_11_10: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#155158 <= s_12_0
        fn_state.gs_155158 = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#155156 <= s_13_0
        fn_state.gs_155156 = s_13_0;
        // N s_13_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#155154 <= s_14_0
        fn_state.gs_155154 = s_14_0;
        // N s_14_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#155152 <= s_15_0
        fn_state.gs_155152 = s_15_0;
        // N s_15_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#155150 <= s_16_0
        fn_state.gs_155150 = s_16_0;
        // N s_16_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
}
