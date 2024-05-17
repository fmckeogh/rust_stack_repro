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
use X_read::*;
use u__id::*;
use BranchNotTaken::*;
use BranchTo::*;
use common::*;
pub fn execute_aarch64_instrs_branch_conditional_test<T: Tracer>(
    state: &mut State,
    tracer: &T,
    bit_pos: i64,
    bit_val: bool,
    datasize: i64,
    offset: u64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        bit_pos: i64,
        bit_val: bool,
        datasize: i64,
        offset: u64,
        t: i64,
    }
    let fn_state = FunctionState {
        bit_pos,
        bit_val,
        datasize,
        offset,
        t,
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
        // D s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_4: cast reint s_0_3 -> i64
        let s_0_4: i64 = (s_0_3 as i64);
        // D s_0_5: read-var t:i64
        let s_0_5: i64 = fn_state.t;
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: call X_read(s_0_6, s_0_4)
        let s_0_7: Bits = X_read(state, tracer, s_0_6, s_0_4);
        // D s_0_8: read-var bit_pos:i64
        let s_0_8: i64 = fn_state.bit_pos;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: call __id(s_0_9)
        let s_0_10: i128 = u__id(state, tracer, s_0_9);
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // D s_0_12: cast zx s_0_2 -> i
        let s_0_12: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_13: call __id(s_0_12)
        let s_0_13: i128 = u__id(state, tracer, s_0_12);
        // D s_0_14: cast reint s_0_13 -> i64
        let s_0_14: i64 = (s_0_13 as i64);
        // D s_0_15: cast zx s_0_11 -> i
        let s_0_15: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_16: cast zx s_0_14 -> i
        let s_0_16: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_17: cmp-lt s_0_15 s_0_16
        let s_0_17: bool = ((s_0_15) < (s_0_16));
        // N s_0_18: assert s_0_17
        let s_0_18: () = assert!(s_0_17);
        // D s_0_19: read-var bit_pos:i64
        let s_0_19: i64 = fn_state.bit_pos;
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // C s_0_21: const #1u : u64
        let s_0_21: u64 = 1;
        // D s_0_22: bit-extract s_0_7 s_0_20 s_0_21
        let s_0_22: Bits = (Bits::new(
            ((s_0_7) >> (s_0_20)).value(),
            u16::try_from(s_0_21).unwrap(),
        ));
        // D s_0_23: cast reint s_0_22 -> u8
        let s_0_23: bool = ((s_0_22.value()) != 0);
        // C s_0_24: const #0s : i
        let s_0_24: i128 = 0;
        // C s_0_25: const #0u : u64
        let s_0_25: u64 = 0;
        // D s_0_26: cast zx s_0_23 -> u64
        let s_0_26: u64 = (s_0_23 as u64);
        // C s_0_27: const #1u : u64
        let s_0_27: u64 = 1;
        // D s_0_28: and s_0_26 s_0_27
        let s_0_28: u64 = ((s_0_26) & (s_0_27));
        // D s_0_29: cmp-eq s_0_28 s_0_27
        let s_0_29: bool = ((s_0_28) == (s_0_27));
        // D s_0_30: lsl s_0_26 s_0_24
        let s_0_30: u64 = s_0_26 << s_0_24;
        // D s_0_31: or s_0_25 s_0_30
        let s_0_31: u64 = ((s_0_25) | (s_0_30));
        // D s_0_32: cmpl s_0_30
        let s_0_32: u64 = !s_0_30;
        // D s_0_33: and s_0_25 s_0_32
        let s_0_33: u64 = ((s_0_25) & (s_0_32));
        // D s_0_34: select s_0_29 s_0_31 s_0_33
        let s_0_34: u64 = if s_0_29 { s_0_31 } else { s_0_33 };
        // D s_0_35: cast trunc s_0_34 -> u8
        let s_0_35: bool = ((s_0_34) != 0);
        // D s_0_36: cast zx s_0_35 -> bv
        let s_0_36: Bits = Bits::new(s_0_35 as u128, 1u16);
        // D s_0_37: read-var bit_val:u8
        let s_0_37: bool = fn_state.bit_val;
        // D s_0_38: cast zx s_0_37 -> bv
        let s_0_38: Bits = Bits::new(s_0_37 as u128, 1u16);
        // D s_0_39: cmp-eq s_0_36 s_0_38
        let s_0_39: bool = ((s_0_36) == (s_0_38));
        // N s_0_40: branch s_0_39 b2 b1
        if s_0_39 {
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
        // C s_1_0: const #5u : u32
        let s_1_0: u32 = 5;
        // C s_1_1: const #1u : u8
        let s_1_1: bool = true;
        // S s_1_2: call BranchNotTaken(s_1_0, s_1_1)
        let s_1_2: () = BranchNotTaken(state, tracer, s_1_0, s_1_1);
        // N s_1_3: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #12744u : u32
        let s_2_0: u32 = 12744;
        // D s_2_1: read-reg s_2_0:u64
        let s_2_1: u64 = {
            let value = state.read_register::<u64>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 64u16);
        // D s_2_3: read-var offset:u64
        let s_2_3: u64 = fn_state.offset;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 64u16);
        // D s_2_5: add s_2_2 s_2_4
        let s_2_5: Bits = (s_2_2 + s_2_4);
        // D s_2_6: cast reint s_2_5 -> u64
        let s_2_6: u64 = (s_2_5.value() as u64);
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 64u16);
        // C s_2_8: const #5u : u32
        let s_2_8: u32 = 5;
        // C s_2_9: const #1u : u8
        let s_2_9: bool = true;
        // D s_2_10: call BranchTo(s_2_7, s_2_8, s_2_9)
        let s_2_10: () = BranchTo(state, tracer, s_2_7, s_2_8, s_2_9);
        // N s_2_11: return
        return;
    }
}
