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
use CheckVFPEnabled::*;
use Zeros::*;
use S_set::*;
use S_read::*;
use common::*;
pub fn execute_aarch32_instrs_VMOVX_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16s : i
        let s_1_0: i128 = 16;
        // S s_1_1: call Zeros(s_1_0)
        let s_1_1: Bits = Zeros(state, tracer, s_1_0);
        // S s_1_2: cast reint s_1_1 -> u16
        let s_1_2: u16 = (s_1_1.value() as u16);
        // D s_1_3: read-var m:i64
        let s_1_3: i64 = fn_state.m;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call S_read(s_1_4)
        let s_1_5: u32 = S_read(state, tracer, s_1_4);
        // C s_1_6: const #16s : i
        let s_1_6: i128 = 16;
        // D s_1_7: cast zx s_1_5 -> bv
        let s_1_7: Bits = Bits::new(s_1_5 as u128, 32u16);
        // C s_1_8: const #1s : i64
        let s_1_8: i64 = 1;
        // C s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // C s_1_10: const #15s : i
        let s_1_10: i128 = 15;
        // C s_1_11: add s_1_10 s_1_9
        let s_1_11: i128 = (s_1_10 + s_1_9);
        // D s_1_12: bit-extract s_1_7 s_1_6 s_1_11
        let s_1_12: Bits = (Bits::new(
            ((s_1_7) >> (s_1_6)).value(),
            u16::try_from(s_1_11).unwrap(),
        ));
        // D s_1_13: cast reint s_1_12 -> u16
        let s_1_13: u16 = (s_1_12.value() as u16);
        // S s_1_14: cast zx s_1_2 -> bv
        let s_1_14: Bits = Bits::new(s_1_2 as u128, 16u16);
        // D s_1_15: cast zx s_1_13 -> bv
        let s_1_15: Bits = Bits::new(s_1_13 as u128, 16u16);
        // S s_1_16: cast reint s_1_14 -> u128
        let s_1_16: u128 = (s_1_14.value() as u128);
        // D s_1_17: size-of s_1_14
        let s_1_17: u16 = s_1_14.length();
        // D s_1_18: cast reint s_1_15 -> u128
        let s_1_18: u128 = (s_1_15.value() as u128);
        // D s_1_19: size-of s_1_15
        let s_1_19: u16 = s_1_15.length();
        // D s_1_20: lsl s_1_16 s_1_19
        let s_1_20: u128 = s_1_16 << s_1_19;
        // D s_1_21: or s_1_20 s_1_18
        let s_1_21: u128 = ((s_1_20) | (s_1_18));
        // D s_1_22: add s_1_17 s_1_19
        let s_1_22: u16 = (s_1_17 + s_1_19);
        // D s_1_23: create-bits s_1_21 s_1_22
        let s_1_23: Bits = Bits::new(s_1_21, s_1_22);
        // D s_1_24: cast reint s_1_23 -> u32
        let s_1_24: u32 = (s_1_23.value() as u32);
        // D s_1_25: read-var d:i64
        let s_1_25: i64 = fn_state.d;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: call S_set(s_1_26, s_1_24)
        let s_1_27: () = S_set(state, tracer, s_1_26, s_1_24);
        // N s_1_28: return
        return;
    }
}
