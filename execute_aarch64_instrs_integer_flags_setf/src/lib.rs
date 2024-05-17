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
use is_zero_subrange::*;
use common::*;
pub fn execute_aarch64_instrs_integer_flags_setf<T: Tracer>(
    state: &mut State,
    tracer: &T,
    msb: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        tmpreg: u32,
        msb: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        msb,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #32s : i64
        let s_0_0: i64 = 32;
        // D s_0_1: read-var n:i64
        let s_0_1: i64 = fn_state.n;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: call X_read(s_0_2, s_0_0)
        let s_0_3: Bits = X_read(state, tracer, s_0_2, s_0_0);
        // D s_0_4: cast reint s_0_3 -> u32
        let s_0_4: u32 = (s_0_3.value() as u32);
        // D s_0_5: write-var tmpreg <= s_0_4
        fn_state.tmpreg = s_0_4;
        // D s_0_6: read-var tmpreg:u32
        let s_0_6: u32 = fn_state.tmpreg;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 32u16);
        // D s_0_8: read-var msb:i64
        let s_0_8: i64 = fn_state.msb;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // C s_0_10: const #1u : u64
        let s_0_10: u64 = 1;
        // D s_0_11: bit-extract s_0_7 s_0_9 s_0_10
        let s_0_11: Bits = (Bits::new(
            ((s_0_7) >> (s_0_9)).value(),
            u16::try_from(s_0_10).unwrap(),
        ));
        // D s_0_12: cast reint s_0_11 -> u8
        let s_0_12: bool = ((s_0_11.value()) != 0);
        // C s_0_13: const #0s : i
        let s_0_13: i128 = 0;
        // C s_0_14: const #0u : u64
        let s_0_14: u64 = 0;
        // D s_0_15: cast zx s_0_12 -> u64
        let s_0_15: u64 = (s_0_12 as u64);
        // C s_0_16: const #1u : u64
        let s_0_16: u64 = 1;
        // D s_0_17: and s_0_15 s_0_16
        let s_0_17: u64 = ((s_0_15) & (s_0_16));
        // D s_0_18: cmp-eq s_0_17 s_0_16
        let s_0_18: bool = ((s_0_17) == (s_0_16));
        // D s_0_19: lsl s_0_15 s_0_13
        let s_0_19: u64 = s_0_15 << s_0_13;
        // D s_0_20: or s_0_14 s_0_19
        let s_0_20: u64 = ((s_0_14) | (s_0_19));
        // D s_0_21: cmpl s_0_19
        let s_0_21: u64 = !s_0_19;
        // D s_0_22: and s_0_14 s_0_21
        let s_0_22: u64 = ((s_0_14) & (s_0_21));
        // D s_0_23: select s_0_18 s_0_20 s_0_22
        let s_0_23: u64 = if s_0_18 { s_0_20 } else { s_0_22 };
        // D s_0_24: cast trunc s_0_23 -> u8
        let s_0_24: bool = ((s_0_23) != 0);
        // C s_0_25: const #16984u : u32
        let s_0_25: u32 = 16984;
        // N s_0_26: write-reg s_0_25 <= s_0_24
        let s_0_26: () = {
            state.write_register::<bool>(s_0_25 as isize, s_0_24);
            tracer.write_register(s_0_25 as isize, s_0_24);
        };
        // C s_0_27: const #0s : i
        let s_0_27: i128 = 0;
        // D s_0_28: read-var tmpreg:u32
        let s_0_28: u32 = fn_state.tmpreg;
        // D s_0_29: cast zx s_0_28 -> bv
        let s_0_29: Bits = Bits::new(s_0_28 as u128, 32u16);
        // D s_0_30: read-var msb:i64
        let s_0_30: i64 = fn_state.msb;
        // D s_0_31: cast zx s_0_30 -> i
        let s_0_31: i128 = (i128::try_from(s_0_30).unwrap());
        // D s_0_32: call is_zero_subrange(s_0_29, s_0_31, s_0_27)
        let s_0_32: bool = is_zero_subrange(state, tracer, s_0_29, s_0_31, s_0_27);
        // N s_0_33: branch s_0_32 b3 b1
        if s_0_32 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // C s_1_1: const #16997u : u32
        let s_1_1: u32 = 16997;
        // N s_1_2: write-reg s_1_1 <= s_1_0
        let s_1_2: () = {
            state.write_register::<bool>(s_1_1 as isize, s_1_0);
            tracer.write_register(s_1_1 as isize, s_1_0);
        };
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1s : i
        let s_2_0: i128 = 1;
        // D s_2_1: read-var msb:i64
        let s_2_1: i64 = fn_state.msb;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: add s_2_2 s_2_0
        let s_2_3: i128 = (s_2_2 + s_2_0);
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: read-var tmpreg:u32
        let s_2_5: u32 = fn_state.tmpreg;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 32u16);
        // D s_2_7: cast zx s_2_4 -> i
        let s_2_7: i128 = (i128::try_from(s_2_4).unwrap());
        // C s_2_8: const #1u : u64
        let s_2_8: u64 = 1;
        // D s_2_9: bit-extract s_2_6 s_2_7 s_2_8
        let s_2_9: Bits = (Bits::new(
            ((s_2_6) >> (s_2_7)).value(),
            u16::try_from(s_2_8).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u8
        let s_2_10: bool = ((s_2_9.value()) != 0);
        // C s_2_11: const #0s : i
        let s_2_11: i128 = 0;
        // C s_2_12: const #0u : u64
        let s_2_12: u64 = 0;
        // D s_2_13: cast zx s_2_10 -> u64
        let s_2_13: u64 = (s_2_10 as u64);
        // C s_2_14: const #1u : u64
        let s_2_14: u64 = 1;
        // D s_2_15: and s_2_13 s_2_14
        let s_2_15: u64 = ((s_2_13) & (s_2_14));
        // D s_2_16: cmp-eq s_2_15 s_2_14
        let s_2_16: bool = ((s_2_15) == (s_2_14));
        // D s_2_17: lsl s_2_13 s_2_11
        let s_2_17: u64 = s_2_13 << s_2_11;
        // D s_2_18: or s_2_12 s_2_17
        let s_2_18: u64 = ((s_2_12) | (s_2_17));
        // D s_2_19: cmpl s_2_17
        let s_2_19: u64 = !s_2_17;
        // D s_2_20: and s_2_12 s_2_19
        let s_2_20: u64 = ((s_2_12) & (s_2_19));
        // D s_2_21: select s_2_16 s_2_18 s_2_20
        let s_2_21: u64 = if s_2_16 { s_2_18 } else { s_2_20 };
        // D s_2_22: cast trunc s_2_21 -> u8
        let s_2_22: bool = ((s_2_21) != 0);
        // D s_2_23: read-var tmpreg:u32
        let s_2_23: u32 = fn_state.tmpreg;
        // D s_2_24: cast zx s_2_23 -> bv
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 32u16);
        // D s_2_25: read-var msb:i64
        let s_2_25: i64 = fn_state.msb;
        // D s_2_26: cast zx s_2_25 -> i
        let s_2_26: i128 = (i128::try_from(s_2_25).unwrap());
        // C s_2_27: const #1u : u64
        let s_2_27: u64 = 1;
        // D s_2_28: bit-extract s_2_24 s_2_26 s_2_27
        let s_2_28: Bits = (Bits::new(
            ((s_2_24) >> (s_2_26)).value(),
            u16::try_from(s_2_27).unwrap(),
        ));
        // D s_2_29: cast reint s_2_28 -> u8
        let s_2_29: bool = ((s_2_28.value()) != 0);
        // C s_2_30: const #0s : i
        let s_2_30: i128 = 0;
        // C s_2_31: const #0u : u64
        let s_2_31: u64 = 0;
        // D s_2_32: cast zx s_2_29 -> u64
        let s_2_32: u64 = (s_2_29 as u64);
        // C s_2_33: const #1u : u64
        let s_2_33: u64 = 1;
        // D s_2_34: and s_2_32 s_2_33
        let s_2_34: u64 = ((s_2_32) & (s_2_33));
        // D s_2_35: cmp-eq s_2_34 s_2_33
        let s_2_35: bool = ((s_2_34) == (s_2_33));
        // D s_2_36: lsl s_2_32 s_2_30
        let s_2_36: u64 = s_2_32 << s_2_30;
        // D s_2_37: or s_2_31 s_2_36
        let s_2_37: u64 = ((s_2_31) | (s_2_36));
        // D s_2_38: cmpl s_2_36
        let s_2_38: u64 = !s_2_36;
        // D s_2_39: and s_2_31 s_2_38
        let s_2_39: u64 = ((s_2_31) & (s_2_38));
        // D s_2_40: select s_2_35 s_2_37 s_2_39
        let s_2_40: u64 = if s_2_35 { s_2_37 } else { s_2_39 };
        // D s_2_41: cast trunc s_2_40 -> u8
        let s_2_41: bool = ((s_2_40) != 0);
        // D s_2_42: cast zx s_2_22 -> bv
        let s_2_42: Bits = Bits::new(s_2_22 as u128, 1u16);
        // D s_2_43: cast zx s_2_41 -> bv
        let s_2_43: Bits = Bits::new(s_2_41 as u128, 1u16);
        // D s_2_44: xor s_2_42 s_2_43
        let s_2_44: Bits = ((s_2_42) ^ (s_2_43));
        // D s_2_45: cast reint s_2_44 -> u8
        let s_2_45: bool = ((s_2_44.value()) != 0);
        // C s_2_46: const #16996u : u32
        let s_2_46: u32 = 16996;
        // N s_2_47: write-reg s_2_46 <= s_2_45
        let s_2_47: () = {
            state.write_register::<bool>(s_2_46 as isize, s_2_45);
            tracer.write_register(s_2_46 as isize, s_2_45);
        };
        // N s_2_48: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // C s_3_1: const #16997u : u32
        let s_3_1: u32 = 16997;
        // N s_3_2: write-reg s_3_1 <= s_3_0
        let s_3_2: () = {
            state.write_register::<bool>(s_3_1 as isize, s_3_0);
            tracer.write_register(s_3_1 as isize, s_3_0);
        };
        // N s_3_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
