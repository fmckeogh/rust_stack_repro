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
use CollectContextIDR2::*;
use CollectContextIDR1::*;
use common::*;
pub fn SPESampleAddContext<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25530: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25530: (),
    }
    let fn_state = FunctionState {
        gs_25530,
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
        // S s_0_1: call CollectContextIDR1(s_0_0)
        let s_0_1: bool = CollectContextIDR1(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b5 b1
        if s_0_1 {
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
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call CollectContextIDR2(s_2_0)
        let s_2_1: bool = CollectContextIDR2(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b4 b3
        if s_2_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #91008u : u32
        let s_4_0: u32 = 91008;
        // D s_4_1: read-reg s_4_0:u64
        let s_4_1: u64 = {
            let value = state.read_register::<u64>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // C s_4_2: const #0s : i
        let s_4_2: i128 = 0;
        // D s_4_3: cast zx s_4_1 -> bv
        let s_4_3: Bits = Bits::new(s_4_1 as u128, 64u16);
        // C s_4_4: const #1s : i64
        let s_4_4: i64 = 1;
        // C s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // C s_4_6: const #31s : i
        let s_4_6: i128 = 31;
        // C s_4_7: add s_4_6 s_4_5
        let s_4_7: i128 = (s_4_6 + s_4_5);
        // D s_4_8: bit-extract s_4_3 s_4_2 s_4_7
        let s_4_8: Bits = (Bits::new(
            ((s_4_3) >> (s_4_2)).value(),
            u16::try_from(s_4_7).unwrap(),
        ));
        // D s_4_9: cast reint s_4_8 -> u32
        let s_4_9: u32 = (s_4_8.value() as u32);
        // C s_4_10: const #20880u : u32
        let s_4_10: u32 = 20880;
        // N s_4_11: write-reg s_4_10 <= s_4_9
        let s_4_11: () = {
            state.write_register::<u32>(s_4_10 as isize, s_4_9);
            tracer.write_register(s_4_10 as isize, s_4_9);
        };
        // C s_4_12: const #1u : u8
        let s_4_12: bool = true;
        // C s_4_13: const #14880u : u32
        let s_4_13: u32 = 14880;
        // N s_4_14: write-reg s_4_13 <= s_4_12
        let s_4_14: () = {
            state.write_register::<bool>(s_4_13 as isize, s_4_12);
            tracer.write_register(s_4_13 as isize, s_4_12);
        };
        // N s_4_15: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #10048u : u32
        let s_5_0: u32 = 10048;
        // D s_5_1: read-reg s_5_0:u64
        let s_5_1: u64 = {
            let value = state.read_register::<u64>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #0s : i
        let s_5_2: i128 = 0;
        // D s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 64u16);
        // C s_5_4: const #1s : i64
        let s_5_4: i64 = 1;
        // C s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // C s_5_6: const #31s : i
        let s_5_6: i128 = 31;
        // C s_5_7: add s_5_6 s_5_5
        let s_5_7: i128 = (s_5_6 + s_5_5);
        // D s_5_8: bit-extract s_5_3 s_5_2 s_5_7
        let s_5_8: Bits = (Bits::new(
            ((s_5_3) >> (s_5_2)).value(),
            u16::try_from(s_5_7).unwrap(),
        ));
        // D s_5_9: cast reint s_5_8 -> u32
        let s_5_9: u32 = (s_5_8.value() as u32);
        // C s_5_10: const #104568u : u32
        let s_5_10: u32 = 104568;
        // N s_5_11: write-reg s_5_10 <= s_5_9
        let s_5_11: () = {
            state.write_register::<u32>(s_5_10 as isize, s_5_9);
            tracer.write_register(s_5_10 as isize, s_5_9);
        };
        // C s_5_12: const #1u : u8
        let s_5_12: bool = true;
        // C s_5_13: const #20160u : u32
        let s_5_13: u32 = 20160;
        // N s_5_14: write-reg s_5_13 <= s_5_12
        let s_5_14: () = {
            state.write_register::<bool>(s_5_13 as isize, s_5_12);
            tracer.write_register(s_5_13 as isize, s_5_12);
        };
        // N s_5_15: jump b2
        return block_2(state, tracer, fn_state);
    }
}
