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
use GetPSRFromPSTATE::*;
use u__UNKNOWN_bits::*;
use Bit::*;
use R_set::*;
use SPSR_read::*;
use common::*;
pub fn execute_aarch32_instrs_MRS_Op_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    read_spsr: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        psr_val: u32,
        gs_323304: bool,
        d: i64,
        read_spsr: bool,
    }
    let fn_state = FunctionState {
        d,
        read_spsr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var read_spsr:u8
        let s_0_0: bool = fn_state.read_spsr;
        // N s_0_1: branch s_0_0 b5 b1
        if s_0_0 {
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
        // C s_1_0: const #4176413663u : u32
        let s_1_0: u32 = 4176413663;
        // C s_1_1: const #32s : i64
        let s_1_1: i64 = 32;
        // C s_1_2: const #0u : u32
        let s_1_2: u32 = 0;
        // S s_1_3: call GetPSRFromPSTATE(s_1_2, s_1_1)
        let s_1_3: Bits = GetPSRFromPSTATE(state, tracer, s_1_2, s_1_1);
        // S s_1_4: cast reint s_1_3 -> u32
        let s_1_4: u32 = (s_1_3.value() as u32);
        // S s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 32u16);
        // C s_1_6: cast zx s_1_0 -> bv
        let s_1_6: Bits = Bits::new(s_1_0 as u128, 32u16);
        // S s_1_7: and s_1_5 s_1_6
        let s_1_7: Bits = ((s_1_5) & (s_1_6));
        // S s_1_8: cast reint s_1_7 -> u32
        let s_1_8: u32 = (s_1_7.value() as u32);
        // D s_1_9: write-var psr_val <= s_1_8
        fn_state.psr_val = s_1_8;
        // C s_1_10: const #16975u : u32
        let s_1_10: u32 = 16975;
        // D s_1_11: read-reg s_1_10:u8
        let s_1_11: u8 = {
            let value = state.read_register::<u8>(s_1_10 as isize);
            tracer.read_register(s_1_10 as isize, value);
            value
        };
        // D s_1_12: cast zx s_1_11 -> bv
        let s_1_12: Bits = Bits::new(s_1_11 as u128, 2u16);
        // C s_1_13: const #448u : u32
        let s_1_13: u32 = 448;
        // D s_1_14: read-reg s_1_13:u8
        let s_1_14: u8 = {
            let value = state.read_register::<u8>(s_1_13 as isize);
            tracer.read_register(s_1_13 as isize, value);
            value
        };
        // D s_1_15: cast zx s_1_14 -> bv
        let s_1_15: Bits = Bits::new(s_1_14 as u128, 2u16);
        // D s_1_16: cmp-eq s_1_12 s_1_15
        let s_1_16: bool = ((s_1_12) == (s_1_15));
        // N s_1_17: branch s_1_16 b4 b2
        if s_1_16 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var d:i64
        let s_3_0: i64 = fn_state.d;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var psr_val:u32
        let s_3_2: u32 = fn_state.psr_val;
        // D s_3_3: call R_set(s_3_1, s_3_2)
        let s_3_3: () = R_set(state, tracer, s_3_1, s_3_2);
        // N s_3_4: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1s : i64
        let s_4_0: i64 = 1;
        // C s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // S s_4_2: call __UNKNOWN_bits(s_4_1)
        let s_4_2: Bits = u__UNKNOWN_bits(state, tracer, s_4_1);
        // S s_4_3: cast reint s_4_2 -> u8
        let s_4_3: bool = ((s_4_2.value()) != 0);
        // S s_4_4: call Bit(s_4_3)
        let s_4_4: bool = Bit(state, tracer, s_4_3);
        // C s_4_5: const #22s : i
        let s_4_5: i128 = 22;
        // D s_4_6: read-var psr_val:u32
        let s_4_6: u32 = fn_state.psr_val;
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 32u16);
        // C s_4_8: const #1u : u64
        let s_4_8: u64 = 1;
        // D s_4_9: bit-insert s_4_7 s_4_7 s_4_5 s_4_8
        let s_4_9: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_4_8 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_4_7.length(),
            );
            (s_4_7 & mask) | (s_4_7 << s_4_5)
        };
        // D s_4_10: cast reint s_4_9 -> u32
        let s_4_10: u32 = (s_4_9.value() as u32);
        // D s_4_11: write-var psr_val <= s_4_10
        fn_state.psr_val = s_4_10;
        // C s_4_12: const #4s : i64
        let s_4_12: i64 = 4;
        // C s_4_13: cast zx s_4_12 -> i
        let s_4_13: i128 = (i128::try_from(s_4_12).unwrap());
        // S s_4_14: call __UNKNOWN_bits(s_4_13)
        let s_4_14: Bits = u__UNKNOWN_bits(state, tracer, s_4_13);
        // S s_4_15: cast reint s_4_14 -> u8
        let s_4_15: u8 = (s_4_14.value() as u8);
        // C s_4_16: const #6s : i
        let s_4_16: i128 = 6;
        // D s_4_17: read-var psr_val:u32
        let s_4_17: u32 = fn_state.psr_val;
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 32u16);
        // S s_4_19: cast zx s_4_15 -> bv
        let s_4_19: Bits = Bits::new(s_4_15 as u128, 4u16);
        // C s_4_20: const #3s : i
        let s_4_20: i128 = 3;
        // C s_4_21: const #1u : u64
        let s_4_21: u64 = 1;
        // C s_4_22: cast zx s_4_21 -> bv
        let s_4_22: Bits = Bits::new(s_4_21 as u128, 64u16);
        // C s_4_23: lsl s_4_22 s_4_20
        let s_4_23: Bits = s_4_22 << s_4_20;
        // C s_4_24: sub s_4_23 s_4_22
        let s_4_24: Bits = ((s_4_23) - (s_4_22));
        // S s_4_25: and s_4_19 s_4_24
        let s_4_25: Bits = ((s_4_19) & (s_4_24));
        // S s_4_26: lsl s_4_25 s_4_16
        let s_4_26: Bits = s_4_25 << s_4_16;
        // C s_4_27: lsl s_4_24 s_4_16
        let s_4_27: Bits = s_4_24 << s_4_16;
        // C s_4_28: cmpl s_4_27
        let s_4_28: Bits = !s_4_27;
        // D s_4_29: and s_4_18 s_4_28
        let s_4_29: Bits = ((s_4_18) & (s_4_28));
        // D s_4_30: or s_4_29 s_4_26
        let s_4_30: Bits = ((s_4_29) | (s_4_26));
        // D s_4_31: cast reint s_4_30 -> u32
        let s_4_31: u32 = (s_4_30.value() as u32);
        // D s_4_32: write-var psr_val <= s_4_31
        fn_state.psr_val = s_4_31;
        // C s_4_33: const #5s : i64
        let s_4_33: i64 = 5;
        // C s_4_34: cast zx s_4_33 -> i
        let s_4_34: i128 = (i128::try_from(s_4_33).unwrap());
        // S s_4_35: call __UNKNOWN_bits(s_4_34)
        let s_4_35: Bits = u__UNKNOWN_bits(state, tracer, s_4_34);
        // S s_4_36: cast reint s_4_35 -> u8
        let s_4_36: u8 = (s_4_35.value() as u8);
        // C s_4_37: const #0s : i
        let s_4_37: i128 = 0;
        // D s_4_38: read-var psr_val:u32
        let s_4_38: u32 = fn_state.psr_val;
        // D s_4_39: cast zx s_4_38 -> bv
        let s_4_39: Bits = Bits::new(s_4_38 as u128, 32u16);
        // S s_4_40: cast zx s_4_36 -> bv
        let s_4_40: Bits = Bits::new(s_4_36 as u128, 5u16);
        // C s_4_41: const #4s : i
        let s_4_41: i128 = 4;
        // C s_4_42: const #1u : u64
        let s_4_42: u64 = 1;
        // C s_4_43: cast zx s_4_42 -> bv
        let s_4_43: Bits = Bits::new(s_4_42 as u128, 64u16);
        // C s_4_44: lsl s_4_43 s_4_41
        let s_4_44: Bits = s_4_43 << s_4_41;
        // C s_4_45: sub s_4_44 s_4_43
        let s_4_45: Bits = ((s_4_44) - (s_4_43));
        // S s_4_46: and s_4_40 s_4_45
        let s_4_46: Bits = ((s_4_40) & (s_4_45));
        // S s_4_47: lsl s_4_46 s_4_37
        let s_4_47: Bits = s_4_46 << s_4_37;
        // C s_4_48: lsl s_4_45 s_4_37
        let s_4_48: Bits = s_4_45 << s_4_37;
        // C s_4_49: cmpl s_4_48
        let s_4_49: Bits = !s_4_48;
        // D s_4_50: and s_4_39 s_4_49
        let s_4_50: Bits = ((s_4_39) & (s_4_49));
        // D s_4_51: or s_4_50 s_4_47
        let s_4_51: Bits = ((s_4_50) | (s_4_47));
        // D s_4_52: cast reint s_4_51 -> u32
        let s_4_52: u32 = (s_4_51.value() as u32);
        // D s_4_53: write-var psr_val <= s_4_52
        fn_state.psr_val = s_4_52;
        // N s_4_54: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16983u : u32
        let s_5_0: u32 = 16983;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 5u16);
        // C s_5_3: const #352u : u32
        let s_5_3: u32 = 352;
        // D s_5_4: read-reg s_5_3:u8
        let s_5_4: u8 = {
            let value = state.read_register::<u8>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 5u16);
        // D s_5_6: cmp-eq s_5_2 s_5_5
        let s_5_6: bool = ((s_5_2) == (s_5_5));
        // N s_5_7: branch s_5_6 b10 b6
        if s_5_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #16983u : u32
        let s_6_0: u32 = 16983;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 5u16);
        // C s_6_3: const #416u : u32
        let s_6_3: u32 = 416;
        // D s_6_4: read-reg s_6_3:u8
        let s_6_4: u8 = {
            let value = state.read_register::<u8>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 5u16);
        // D s_6_6: cmp-eq s_6_2 s_6_5
        let s_6_6: bool = ((s_6_2) == (s_6_5));
        // D s_6_7: write-var gs#323304 <= s_6_6
        fn_state.gs_323304 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#323304:u8
        let s_7_0: bool = fn_state.gs_323304;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #32s : i
        let s_8_0: i128 = 32;
        // S s_8_1: call SPSR_read(s_8_0)
        let s_8_1: Bits = SPSR_read(state, tracer, s_8_0);
        // S s_8_2: cast reint s_8_1 -> u32
        let s_8_2: u32 = (s_8_1.value() as u32);
        // D s_8_3: read-var d:i64
        let s_8_3: i64 = fn_state.d;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: call R_set(s_8_4, s_8_2)
        let s_8_5: () = R_set(state, tracer, s_8_4, s_8_2);
        // N s_8_6: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#323304 <= s_10_0
        fn_state.gs_323304 = s_10_0;
        // N s_10_2: jump b7
        return block_7(state, tracer, fn_state);
    }
}
