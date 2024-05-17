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
use CurrentInstrSet::*;
use R_read::*;
use LR_write::*;
use BXWritePC::*;
use PC_read__1::*;
use common::*;
pub fn execute_aarch32_instrs_BLX_r_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        target: u32,
        m: i64,
    }
    let fn_state = FunctionState {
        m,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: write-var target <= s_0_2
        fn_state.target = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CurrentInstrSet(s_0_4)
        let s_0_5: u32 = CurrentInstrSet(state, tracer, s_0_4);
        // C s_0_6: const #1u : u32
        let s_0_6: u32 = 1;
        // S s_0_7: cmp-eq s_0_5 s_0_6
        let s_0_7: bool = ((s_0_5) == (s_0_6));
        // N s_0_8: branch s_0_7 b3 b1
        if s_0_7 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call PC_read__1(s_1_0)
        let s_1_1: u32 = PC_read__1(state, tracer, s_1_0);
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // S s_1_3: cast zx s_1_1 -> bv
        let s_1_3: Bits = Bits::new(s_1_1 as u128, 32u16);
        // C s_1_4: cast cvt s_1_2 -> bv
        let s_1_4: Bits = Bits::new(s_1_2 as u128, 128);
        // S s_1_5: sub s_1_3 s_1_4
        let s_1_5: Bits = ((s_1_3) - (s_1_4));
        // S s_1_6: cast reint s_1_5 -> u32
        let s_1_6: u32 = (s_1_5.value() as u32);
        // C s_1_7: const #1s : i
        let s_1_7: i128 = 1;
        // S s_1_8: cast zx s_1_6 -> bv
        let s_1_8: Bits = Bits::new(s_1_6 as u128, 32u16);
        // C s_1_9: const #1s : i64
        let s_1_9: i64 = 1;
        // C s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // C s_1_11: const #30s : i
        let s_1_11: i128 = 30;
        // C s_1_12: add s_1_11 s_1_10
        let s_1_12: i128 = (s_1_11 + s_1_10);
        // D s_1_13: bit-extract s_1_8 s_1_7 s_1_12
        let s_1_13: Bits = (Bits::new(
            ((s_1_8) >> (s_1_7)).value(),
            u16::try_from(s_1_12).unwrap(),
        ));
        // D s_1_14: cast reint s_1_13 -> u31
        let s_1_14: u32 = (s_1_13.value() as u32);
        // D s_1_15: cast zx s_1_14 -> bv
        let s_1_15: Bits = Bits::new(s_1_14 as u128, 31u16);
        // C s_1_16: const #1u : u8
        let s_1_16: bool = true;
        // C s_1_17: cast zx s_1_16 -> bv
        let s_1_17: Bits = Bits::new(s_1_16 as u128, 1u16);
        // D s_1_18: cast reint s_1_15 -> u128
        let s_1_18: u128 = (s_1_15.value() as u128);
        // D s_1_19: size-of s_1_15
        let s_1_19: u16 = s_1_15.length();
        // C s_1_20: cast reint s_1_17 -> u128
        let s_1_20: u128 = (s_1_17.value() as u128);
        // D s_1_21: size-of s_1_17
        let s_1_21: u16 = s_1_17.length();
        // D s_1_22: lsl s_1_18 s_1_21
        let s_1_22: u128 = s_1_18 << s_1_21;
        // D s_1_23: or s_1_22 s_1_20
        let s_1_23: u128 = ((s_1_22) | (s_1_20));
        // D s_1_24: add s_1_19 s_1_21
        let s_1_24: u16 = (s_1_19 + s_1_21);
        // D s_1_25: create-bits s_1_23 s_1_24
        let s_1_25: Bits = Bits::new(s_1_23, s_1_24);
        // D s_1_26: cast reint s_1_25 -> u32
        let s_1_26: u32 = (s_1_25.value() as u32);
        // D s_1_27: call LR_write(s_1_26)
        let s_1_27: () = LR_write(state, tracer, s_1_26);
        // N s_1_28: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var target:u32
        let s_2_0: u32 = fn_state.target;
        // C s_2_1: const #1u : u32
        let s_2_1: u32 = 1;
        // D s_2_2: call BXWritePC(s_2_0, s_2_1)
        let s_2_2: () = BXWritePC(state, tracer, s_2_0, s_2_1);
        // N s_2_3: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call PC_read__1(s_3_0)
        let s_3_1: u32 = PC_read__1(state, tracer, s_3_0);
        // C s_3_2: const #4s : i
        let s_3_2: i128 = 4;
        // S s_3_3: cast zx s_3_1 -> bv
        let s_3_3: Bits = Bits::new(s_3_1 as u128, 32u16);
        // C s_3_4: cast cvt s_3_2 -> bv
        let s_3_4: Bits = Bits::new(s_3_2 as u128, 128);
        // S s_3_5: sub s_3_3 s_3_4
        let s_3_5: Bits = ((s_3_3) - (s_3_4));
        // S s_3_6: cast reint s_3_5 -> u32
        let s_3_6: u32 = (s_3_5.value() as u32);
        // S s_3_7: call LR_write(s_3_6)
        let s_3_7: () = LR_write(state, tracer, s_3_6);
        // N s_3_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
