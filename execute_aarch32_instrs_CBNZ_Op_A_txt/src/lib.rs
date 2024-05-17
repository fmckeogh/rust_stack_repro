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
use IsZero::*;
use R_read::*;
use CBWritePC::*;
use PC_read__1::*;
use common::*;
pub fn execute_aarch32_instrs_CBNZ_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm32: u32,
    n: i64,
    nonzero: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        n: i64,
        nonzero: bool,
    }
    let fn_state = FunctionState {
        imm32,
        n,
        nonzero,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: call IsZero(s_0_3)
        let s_0_4: bool = IsZero(state, tracer, s_0_3);
        // D s_0_5: read-var nonzero:u8
        let s_0_5: bool = fn_state.nonzero;
        // D s_0_6: cmp-ne s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) != (s_0_4));
        // N s_0_7: branch s_0_6 b2 b1
        if s_0_6 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call PC_read__1(s_2_0)
        let s_2_1: u32 = PC_read__1(state, tracer, s_2_0);
        // S s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 32u16);
        // D s_2_3: read-var imm32:u32
        let s_2_3: u32 = fn_state.imm32;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 32u16);
        // D s_2_5: add s_2_2 s_2_4
        let s_2_5: Bits = (s_2_2 + s_2_4);
        // D s_2_6: cast reint s_2_5 -> u32
        let s_2_6: u32 = (s_2_5.value() as u32);
        // D s_2_7: call CBWritePC(s_2_6)
        let s_2_7: () = CBWritePC(state, tracer, s_2_6);
        // N s_2_8: return
        return;
    }
}
