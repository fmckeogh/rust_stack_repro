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
use HavePACExt::*;
use neq_int::*;
use execute_aarch64_instrs_integer_pac_strip_dp_1src::*;
use common::*;
pub fn decode_xpac_aarch64_instrs_integer_pac_strip_dp_1src<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    D: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        data: bool,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        D: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        D,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var D:u8
        let s_0_0: bool = fn_state.D;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // D s_0_5: write-var data <= s_0_4
        fn_state.data = s_0_4;
        // D s_0_6: read-var Rd:u8
        let s_0_6: u8 = fn_state.Rd;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 5u16);
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (s_0_7.value() as i128);
        // D s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // D s_0_10: write-var d <= s_0_9
        fn_state.d = s_0_9;
        // D s_0_11: read-var Rn:u8
        let s_0_11: u8 = fn_state.Rn;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 5u16);
        // D s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (s_0_12.value() as i128);
        // D s_0_14: cast reint s_0_13 -> i64
        let s_0_14: i64 = (s_0_13 as i64);
        // D s_0_15: write-var n <= s_0_14
        fn_state.n = s_0_14;
        // C s_0_16: const #() : ()
        let s_0_16: () = ();
        // S s_0_17: call HavePACExt(s_0_16)
        let s_0_17: bool = HavePACExt(state, tracer, s_0_16);
        // S s_0_18: not s_0_17
        let s_0_18: bool = !s_0_17;
        // N s_0_19: branch s_0_18 b4 b1
        if s_0_18 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #31s : i
        let s_1_0: i128 = 31;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call neq_int(s_1_2, s_1_0)
        let s_1_3: bool = neq_int(state, tracer, s_1_2, s_1_0);
        // N s_1_4: branch s_1_3 b3 b2
        if s_1_3 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var d:i64
        let s_2_0: i64 = fn_state.d;
        // D s_2_1: read-var data:u8
        let s_2_1: bool = fn_state.data;
        // D s_2_2: call execute_aarch64_instrs_integer_pac_strip_dp_1src(s_2_0, s_2_1)
        let s_2_2: () = execute_aarch64_instrs_integer_pac_strip_dp_1src(
            state,
            tracer,
            s_2_0,
            s_2_1,
        );
        // N s_2_3: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: panic
        panic!("{:?}", ());
        // N s_3_1: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
}
