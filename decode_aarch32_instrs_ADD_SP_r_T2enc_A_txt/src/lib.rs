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
use ConditionPassed::*;
use execute_aarch32_instrs_ADD_SP_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_ADD_SP_r_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rm: u8,
    }
    let fn_state = FunctionState {
        Rm,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
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
        // D s_2_0: read-var Rm:u8
        let s_2_0: u8 = fn_state.Rm;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #13u : u8
        let s_2_2: u8 = 13;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b4 b3
        if s_2_4 {
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
        // C s_3_0: const #13s : i64
        let s_3_0: i64 = 13;
        // D s_3_1: read-var Rm:u8
        let s_3_1: u8 = fn_state.Rm;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (s_3_2.value() as i128);
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // C s_3_5: const #0u : u8
        let s_3_5: bool = false;
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u32
        let s_3_7: u32 = 0;
        // D s_3_8: create-product struct = ["s_3_7", "s_3_6"]
        let s_3_8: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_3_7,
            _1: s_3_6,
        };
        // D s_3_9: extract-field s_3_8.0
        let s_3_9: u32 = s_3_8._0;
        // C s_3_10: const #0u : u32
        let s_3_10: u32 = 0;
        // D s_3_11: create-product struct = ["s_3_10", "s_3_6"]
        let s_3_11: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_3_10,
            _1: s_3_6,
        };
        // D s_3_12: extract-field s_3_11.1
        let s_3_12: i128 = s_3_11._1;
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: call execute_aarch32_instrs_ADD_SP_r_Op_A_txt(s_3_0, s_3_4, s_3_5, s_3_14, s_3_9)
        let s_3_15: () = execute_aarch32_instrs_ADD_SP_r_Op_A_txt(
            state,
            tracer,
            s_3_0,
            s_3_4,
            s_3_5,
            s_3_14,
            s_3_9,
        );
        // N s_3_16: return
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
