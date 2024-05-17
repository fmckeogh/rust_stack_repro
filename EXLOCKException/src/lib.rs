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
use ThisInstrAddr::*;
use Bit::*;
use Zeros::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use common::*;
pub fn EXLOCKException<T: Tracer>(state: &mut State, tracer: &T, gs_23091: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_23091: (),
    }
    let fn_state = FunctionState {
        gs_23091,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #64s : i64
        let s_0_0: i64 = 64;
        // C s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // S s_0_2: call ThisInstrAddr(s_0_1)
        let s_0_2: Bits = ThisInstrAddr(state, tracer, s_0_1);
        // S s_0_3: cast reint s_0_2 -> u64
        let s_0_3: u64 = (s_0_2.value() as u64);
        // C s_0_4: const #0u : u8
        let s_0_4: u8 = 0;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 4u16);
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (s_0_5.value() as i128);
        // C s_0_7: cast reint s_0_6 -> i64
        let s_0_7: i64 = (s_0_6 as i64);
        // C s_0_8: const #38u : u32
        let s_0_8: u32 = 38;
        // S s_0_9: call ExceptionSyndrome(s_0_8)
        let s_0_9: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_0_8);
        // C s_0_10: const #1s : i
        let s_0_10: i128 = 1;
        // S s_0_11: call Zeros(s_0_10)
        let s_0_11: Bits = Zeros(state, tracer, s_0_10);
        // S s_0_12: cast reint s_0_11 -> u8
        let s_0_12: bool = ((s_0_11.value()) != 0);
        // S s_0_13: call Bit(s_0_12)
        let s_0_13: bool = Bit(state, tracer, s_0_12);
        // C s_0_14: const #20s : i
        let s_0_14: i128 = 20;
        // S s_0_15: call Zeros(s_0_14)
        let s_0_15: Bits = Zeros(state, tracer, s_0_14);
        // C s_0_16: const #16975u : u32
        let s_0_16: u32 = 16975;
        // D s_0_17: read-reg s_0_16:u8
        let s_0_17: u8 = {
            let value = state.read_register::<u8>(s_0_16 as isize);
            tracer.read_register(s_0_16 as isize, value);
            value
        };
        // C s_0_18: cast zx s_0_7 -> i
        let s_0_18: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_19: call AArch64_TakeException(s_0_17, s_0_9, s_0_3, s_0_18)
        let s_0_19: () = AArch64_TakeException(
            state,
            tracer,
            s_0_17,
            s_0_9,
            s_0_3,
            s_0_18,
        );
        // N s_0_20: return
        return;
    }
}
