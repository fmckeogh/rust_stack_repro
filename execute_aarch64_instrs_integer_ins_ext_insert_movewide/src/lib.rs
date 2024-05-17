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
use X_set::*;
use X_read::*;
use u__id::*;
use Zeros::*;
use common::*;
pub fn execute_aarch64_instrs_integer_ins_ext_insert_movewide<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    imm: u16,
    opcode: u32,
    pos: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasizeshadow_1761: i64,
        result: Bits,
        d: i64,
        datasize: i64,
        imm: u16,
        opcode: u32,
        pos: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        imm,
        opcode,
        pos,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1761 <= s_0_2
        fn_state.datasizeshadow_1761 = s_0_2;
        // D s_0_4: read-var opcode:u32
        let s_0_4: u32 = fn_state.opcode;
        // C s_0_5: const #2u : u32
        let s_0_5: u32 = 2;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // N s_0_7: branch s_0_6 b6 b1
        if s_0_6 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1761:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1761;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call Zeros(s_1_1)
        let s_1_2: Bits = Zeros(state, tracer, s_1_1);
        // D s_1_3: write-var result <= s_1_2
        fn_state.result = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var pos:i64
        let s_2_0: i64 = fn_state.pos;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call __id(s_2_1)
        let s_2_2: i128 = u__id(state, tracer, s_2_1);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #15s : i
        let s_2_4: i128 = 15;
        // D s_2_5: cast zx s_2_3 -> i
        let s_2_5: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // D s_2_8: read-var datasizeshadow#1761:i64
        let s_2_8: i64 = fn_state.datasizeshadow_1761;
        // D s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (i128::try_from(s_2_8).unwrap());
        // D s_2_10: call __id(s_2_9)
        let s_2_10: i128 = u__id(state, tracer, s_2_9);
        // D s_2_11: cast reint s_2_10 -> i64
        let s_2_11: i64 = (s_2_10 as i64);
        // D s_2_12: cast zx s_2_7 -> i
        let s_2_12: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_13: cast zx s_2_11 -> i
        let s_2_13: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_14: cmp-lt s_2_12 s_2_13
        let s_2_14: bool = ((s_2_12) < (s_2_13));
        // N s_2_15: assert s_2_14
        let s_2_15: () = assert!(s_2_14);
        // C s_2_16: const #15s : i
        let s_2_16: i128 = 15;
        // D s_2_17: read-var pos:i64
        let s_2_17: i64 = fn_state.pos;
        // D s_2_18: cast zx s_2_17 -> i
        let s_2_18: i128 = (i128::try_from(s_2_17).unwrap());
        // D s_2_19: add s_2_18 s_2_16
        let s_2_19: i128 = (s_2_18 + s_2_16);
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: cast zx s_2_20 -> i
        let s_2_21: i128 = (i128::try_from(s_2_20).unwrap());
        // D s_2_22: read-var pos:i64
        let s_2_22: i64 = fn_state.pos;
        // D s_2_23: cast zx s_2_22 -> i
        let s_2_23: i128 = (i128::try_from(s_2_22).unwrap());
        // D s_2_24: read-var imm:u16
        let s_2_24: u16 = fn_state.imm;
        // D s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 16u16);
        // D s_2_26: read-var result:bv
        let s_2_26: Bits = fn_state.result;
        // D s_2_27: sub s_2_21 s_2_23
        let s_2_27: i128 = ((s_2_21) - (s_2_23));
        // C s_2_28: const #1u : u64
        let s_2_28: u64 = 1;
        // C s_2_29: cast zx s_2_28 -> bv
        let s_2_29: Bits = Bits::new(s_2_28 as u128, 64u16);
        // D s_2_30: lsl s_2_29 s_2_27
        let s_2_30: Bits = s_2_29 << s_2_27;
        // D s_2_31: sub s_2_30 s_2_29
        let s_2_31: Bits = ((s_2_30) - (s_2_29));
        // D s_2_32: and s_2_25 s_2_31
        let s_2_32: Bits = ((s_2_25) & (s_2_31));
        // D s_2_33: lsl s_2_32 s_2_23
        let s_2_33: Bits = s_2_32 << s_2_23;
        // D s_2_34: lsl s_2_31 s_2_23
        let s_2_34: Bits = s_2_31 << s_2_23;
        // D s_2_35: cmpl s_2_34
        let s_2_35: Bits = !s_2_34;
        // D s_2_36: and s_2_26 s_2_35
        let s_2_36: Bits = ((s_2_26) & (s_2_35));
        // D s_2_37: or s_2_36 s_2_33
        let s_2_37: Bits = ((s_2_36) | (s_2_33));
        // D s_2_38: write-var result <= s_2_37
        fn_state.result = s_2_37;
        // D s_2_39: read-var opcode:u32
        let s_2_39: u32 = fn_state.opcode;
        // C s_2_40: const #0u : u32
        let s_2_40: u32 = 0;
        // D s_2_41: cmp-eq s_2_39 s_2_40
        let s_2_41: bool = ((s_2_39) == (s_2_40));
        // N s_2_42: branch s_2_41 b5 b3
        if s_2_41 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var result:bv
        let s_4_0: Bits = fn_state.result;
        // D s_4_1: read-var datasizeshadow#1761:i64
        let s_4_1: i64 = fn_state.datasizeshadow_1761;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: read-var d:i64
        let s_4_4: i64 = fn_state.d;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: call X_set(s_4_5, s_4_3, s_4_0)
        let s_4_6: () = X_set(state, tracer, s_4_5, s_4_3, s_4_0);
        // N s_4_7: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var result:bv
        let s_5_0: Bits = fn_state.result;
        // D s_5_1: not s_5_0
        let s_5_1: Bits = !s_5_0;
        // D s_5_2: write-var result <= s_5_1
        fn_state.result = s_5_1;
        // N s_5_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var datasizeshadow#1761:i64
        let s_6_0: i64 = fn_state.datasizeshadow_1761;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var d:i64
        let s_6_3: i64 = fn_state.d;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: call X_read(s_6_4, s_6_2)
        let s_6_5: Bits = X_read(state, tracer, s_6_4, s_6_2);
        // D s_6_6: write-var result <= s_6_5
        fn_state.result = s_6_5;
        // N s_6_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
