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
use decode_udf_perm_undef_aarch64_instrs_udf::*;
use common::*;
pub fn u__DecodeA64_Reserved<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_331580: i128,
    gs_331581: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u__opcode: u32,
        gs_331586: bool,
        gs_331580: i128,
        gs_331581: u32,
    }
    let fn_state = FunctionState {
        gs_331580,
        gs_331581,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var gs#331581:u32
        let s_0_0: u32 = fn_state.gs_331581;
        // D s_0_1: write-var __opcode <= s_0_0
        fn_state.u__opcode = s_0_0;
        // C s_0_2: const #16s : i
        let s_0_2: i128 = 16;
        // D s_0_3: read-var __opcode:u32
        let s_0_3: u32 = fn_state.u__opcode;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 32u16);
        // C s_0_5: const #1s : i64
        let s_0_5: i64 = 1;
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_7: const #15s : i
        let s_0_7: i128 = 15;
        // C s_0_8: add s_0_7 s_0_6
        let s_0_8: i128 = (s_0_7 + s_0_6);
        // D s_0_9: bit-extract s_0_4 s_0_2 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_4) >> (s_0_2)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u16
        let s_0_10: u16 = (s_0_9.value() as u16);
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 16u16);
        // C s_0_12: const #0u : u16
        let s_0_12: u16 = 0;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 16u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b5 b1
        if s_0_14 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#331586 <= s_1_0
        fn_state.gs_331586 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#331586:u8
        let s_2_0: bool = fn_state.gs_331586;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
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
        // C s_3_0: const #1017s : i
        let s_3_0: i128 = 1017;
        // C s_3_1: const #14696u : u32
        let s_3_1: u32 = 14696;
        // N s_3_2: write-reg s_3_1 <= s_3_0
        let s_3_2: () = {
            state.write_register::<i128>(s_3_1 as isize, s_3_0);
            tracer.write_register(s_3_1 as isize, s_3_0);
        };
        // C s_3_3: const #0s : i
        let s_3_3: i128 = 0;
        // C s_3_4: const #16s : i
        let s_3_4: i128 = 16;
        // D s_3_5: read-var __opcode:u32
        let s_3_5: u32 = fn_state.u__opcode;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 32u16);
        // D s_3_7: bit-extract s_3_6 s_3_3 s_3_4
        let s_3_7: Bits = (Bits::new(
            ((s_3_6) >> (s_3_3)).value(),
            u16::try_from(s_3_4).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u16
        let s_3_8: u16 = (s_3_7.value() as u16);
        // D s_3_9: call decode_udf_perm_undef_aarch64_instrs_udf(s_3_8)
        let s_3_9: () = decode_udf_perm_undef_aarch64_instrs_udf(state, tracer, s_3_8);
        // N s_3_10: return
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
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1017s : i
        let s_5_0: i128 = 1017;
        // C s_5_1: const #14696u : u32
        let s_5_1: u32 = 14696;
        // D s_5_2: read-reg s_5_1:i
        let s_5_2: i128 = {
            let value = state.read_register::<i128>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: cmp-lt s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) < (s_5_0));
        // D s_5_4: write-var gs#331586 <= s_5_3
        fn_state.gs_331586 = s_5_3;
        // N s_5_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
