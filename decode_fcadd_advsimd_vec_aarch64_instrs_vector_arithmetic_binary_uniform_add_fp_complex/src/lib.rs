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
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_add_fp_complex::*;
use HaveFCADDExt::*;
use common::*;
pub fn decode_fcadd_advsimd_vec_aarch64_instrs_vector_arithmetic_binary_uniform_add_fp_complex<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    rot: bool,
    Rm: u8,
    size: u8,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        n: i64,
        d: i64,
        ga_253158: i64,
        gs_149166: bool,
        Rd: u8,
        Rn: u8,
        rot: bool,
        Rm: u8,
        size: u8,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        rot,
        Rm,
        size,
        Q,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveFCADDExt(s_0_0)
        let s_0_1: bool = HaveFCADDExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
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
    ) -> () {
        // D s_1_0: read-var Rd:u8
        let s_1_0: u8 = fn_state.Rd;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var d <= s_1_3
        fn_state.d = s_1_3;
        // D s_1_5: read-var Rn:u8
        let s_1_5: u8 = fn_state.Rn;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var n <= s_1_8
        fn_state.n = s_1_8;
        // D s_1_10: read-var Rm:u8
        let s_1_10: u8 = fn_state.Rm;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 5u16);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (s_1_11.value() as i128);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var m <= s_1_13
        fn_state.m = s_1_13;
        // D s_1_15: read-var size:u8
        let s_1_15: u8 = fn_state.size;
        // D s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 2u16);
        // C s_1_17: const #0u : u8
        let s_1_17: u8 = 0;
        // C s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 2u16);
        // D s_1_19: cmp-eq s_1_16 s_1_18
        let s_1_19: bool = ((s_1_16) == (s_1_18));
        // N s_1_20: branch s_1_19 b11 b2
        if s_1_19 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Q:u8
        let s_2_0: bool = fn_state.Q;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b10 b3
        if s_2_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#149166 <= s_3_0
        fn_state.gs_149166 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#149166:u8
        let s_4_0: bool = fn_state.gs_149166;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #8s : i64
        let s_5_4: i64 = 8;
        // D s_5_5: lsl s_5_4 s_5_3
        let s_5_5: i64 = s_5_4 << s_5_3;
        // D s_5_6: write-var esize <= s_5_5
        fn_state.esize = s_5_5;
        // D s_5_7: read-var Q:u8
        let s_5_7: bool = fn_state.Q;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 1u16);
        // C s_5_9: const #1u : u8
        let s_5_9: bool = true;
        // C s_5_10: cast zx s_5_9 -> bv
        let s_5_10: Bits = Bits::new(s_5_9 as u128, 1u16);
        // D s_5_11: cmp-eq s_5_8 s_5_10
        let s_5_11: bool = ((s_5_8) == (s_5_10));
        // N s_5_12: branch s_5_11 b8 b6
        if s_5_11 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: write-var ga#253158 <= s_6_0
        fn_state.ga_253158 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#253158:i64
        let s_7_0: i64 = fn_state.ga_253158;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var esize:i64
        let s_7_2: i64 = fn_state.esize;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: div s_7_1 s_7_3
        let s_7_4: i128 = ((s_7_1) / (s_7_3));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: cast zx s_7_0 -> i
        let s_7_6: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // D s_7_8: read-var esize:i64
        let s_7_8: i64 = fn_state.esize;
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: cast zx s_7_5 -> i
        let s_7_11: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_12: read-var d:i64
        let s_7_12: i64 = fn_state.d;
        // D s_7_13: read-var m:i64
        let s_7_13: i64 = fn_state.m;
        // D s_7_14: read-var n:i64
        let s_7_14: i64 = fn_state.n;
        // D s_7_15: read-var rot:u8
        let s_7_15: bool = fn_state.rot;
        // D s_7_16: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_add_fp_complex(s_7_12, s_7_7, s_7_11, s_7_10, s_7_13, s_7_14, s_7_15)
        let s_7_16: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_add_fp_complex(
            state,
            tracer,
            s_7_12,
            s_7_7,
            s_7_11,
            s_7_10,
            s_7_13,
            s_7_14,
            s_7_15,
        );
        // N s_7_17: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #128s : i64
        let s_8_0: i64 = 128;
        // D s_8_1: write-var ga#253158 <= s_8_0
        fn_state.ga_253158 = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var size:u8
        let s_10_0: u8 = fn_state.size;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #3u : u8
        let s_10_2: u8 = 3;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: write-var gs#149166 <= s_10_4
        fn_state.gs_149166 = s_10_4;
        // N s_10_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
}
