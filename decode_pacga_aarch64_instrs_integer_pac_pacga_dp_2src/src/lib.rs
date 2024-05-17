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
use execute_aarch64_instrs_integer_pac_pacga_dp_2src::*;
use common::*;
pub fn decode_pacga_aarch64_instrs_integer_pac_pacga_dp_2src<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        n: i64,
        source_is_sp: bool,
        d: i64,
        Rd: u8,
        Rn: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        Rm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: write-var source_is_sp <= s_0_0
        fn_state.source_is_sp = s_0_0;
        // D s_0_2: read-var Rd:u8
        let s_0_2: u8 = fn_state.Rd;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 5u16);
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (s_0_3.value() as i128);
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // D s_0_7: read-var Rn:u8
        let s_0_7: u8 = fn_state.Rn;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 5u16);
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (s_0_8.value() as i128);
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // D s_0_11: write-var n <= s_0_10
        fn_state.n = s_0_10;
        // D s_0_12: read-var Rm:u8
        let s_0_12: u8 = fn_state.Rm;
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 5u16);
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (s_0_13.value() as i128);
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // D s_0_16: write-var m <= s_0_15
        fn_state.m = s_0_15;
        // C s_0_17: const #() : ()
        let s_0_17: () = ();
        // S s_0_18: call HavePACExt(s_0_17)
        let s_0_18: bool = HavePACExt(state, tracer, s_0_17);
        // S s_0_19: not s_0_18
        let s_0_19: bool = !s_0_18;
        // N s_0_20: branch s_0_19 b5 b1
        if s_0_19 {
            return block_5(state, tracer, fn_state);
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
        // D s_1_1: read-var m:i64
        let s_1_1: i64 = fn_state.m;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_0
        let s_1_3: bool = ((s_1_2) == (s_1_0));
        // N s_1_4: branch s_1_3 b4 b2
        if s_1_3 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var d:i64
        let s_3_0: i64 = fn_state.d;
        // D s_3_1: read-var m:i64
        let s_3_1: i64 = fn_state.m;
        // D s_3_2: read-var n:i64
        let s_3_2: i64 = fn_state.n;
        // D s_3_3: read-var source_is_sp:u8
        let s_3_3: bool = fn_state.source_is_sp;
        // D s_3_4: call execute_aarch64_instrs_integer_pac_pacga_dp_2src(s_3_0, s_3_1, s_3_2, s_3_3)
        let s_3_4: () = execute_aarch64_instrs_integer_pac_pacga_dp_2src(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_2,
            s_3_3,
        );
        // N s_3_5: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // D s_4_1: write-var source_is_sp <= s_4_0
        fn_state.source_is_sp = s_4_0;
        // N s_4_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
}
