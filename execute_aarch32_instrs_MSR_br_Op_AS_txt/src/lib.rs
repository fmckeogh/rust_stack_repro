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
use R_read::*;
use SPSR_hyp_read::*;
use BankedRegisterAccessValid::*;
use ELUsingAArch32::*;
use ELR_hyp_write::*;
use AArch64_MonitorModeTrap::*;
use Mk_SPSR_svc_Type::*;
use SPSR_svc_read::*;
use SPSRaccessValid::*;
use Rmode_set::*;
use SPSR_svc_write::*;
use SPSR_hyp_write::*;
use Mk_SPSR_hyp_Type::*;
use common::*;
pub fn execute_aarch32_instrs_MSR_br_Op_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    SYSm: u8,
    n: i64,
    write_spsr: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_364115: ProductType700c18a878c5601b,
        ga_364099: ProductType700c18a878c5601b,
        modeshadow_7896: u8,
        SYSm: u8,
        n: i64,
        write_spsr: bool,
    }
    let fn_state = FunctionState {
        SYSm,
        n,
        write_spsr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b44 b1
        if s_0_6 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16983u : u32
        let s_1_0: u32 = 16983;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: write-var modeshadow#7896 <= s_1_1
        fn_state.modeshadow_7896 = s_1_1;
        // D s_1_3: read-var write_spsr:u8
        let s_1_3: bool = fn_state.write_spsr;
        // N s_1_4: branch s_1_3 b25 b2
        if s_1_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var SYSm:u8
        let s_2_0: u8 = fn_state.SYSm;
        // D s_2_1: read-var modeshadow#7896:u8
        let s_2_1: u8 = fn_state.modeshadow_7896;
        // D s_2_2: call BankedRegisterAccessValid(s_2_0, s_2_1)
        let s_2_2: () = BankedRegisterAccessValid(state, tracer, s_2_0, s_2_1);
        // N s_2_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var SYSm:u8
        let s_3_0: u8 = fn_state.SYSm;
        // C s_3_1: const #3s : i
        let s_3_1: i128 = 3;
        // D s_3_2: cast zx s_3_0 -> bv
        let s_3_2: Bits = Bits::new(s_3_0 as u128, 5u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_1 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_1)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: u8 = (s_3_7.value() as u8);
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 2u16);
        // C s_3_10: const #0u : u8
        let s_3_10: u8 = 0;
        // C s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 2u16);
        // D s_3_12: cmp-eq s_3_9 s_3_11
        let s_3_12: bool = ((s_3_9) == (s_3_11));
        // D s_3_13: not s_3_12
        let s_3_13: bool = !s_3_12;
        // N s_3_14: branch s_3_13 b5 b4
        if s_3_13 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var SYSm:u8
        let s_4_1: u8 = fn_state.SYSm;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 5u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #2s : i
        let s_4_5: i128 = 2;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: u8 = (s_4_7.value() as u8);
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 3u16);
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (s_4_9.value() as i128);
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // C s_4_12: const #8s : i
        let s_4_12: i128 = 8;
        // D s_4_13: cast zx s_4_11 -> i
        let s_4_13: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_14: add s_4_13 s_4_12
        let s_4_14: i128 = (s_4_13 + s_4_12);
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: read-var n:i64
        let s_4_16: i64 = fn_state.n;
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // D s_4_18: call R_read(s_4_17)
        let s_4_18: u32 = R_read(state, tracer, s_4_17);
        // D s_4_19: cast zx s_4_15 -> i
        let s_4_19: i128 = (i128::try_from(s_4_15).unwrap());
        // C s_4_20: const #352u : u32
        let s_4_20: u32 = 352;
        // D s_4_21: read-reg s_4_20:u8
        let s_4_21: u8 = {
            let value = state.read_register::<u8>(s_4_20 as isize);
            tracer.read_register(s_4_20 as isize, value);
            value
        };
        // D s_4_22: call Rmode_set(s_4_19, s_4_21, s_4_18)
        let s_4_22: () = Rmode_set(state, tracer, s_4_19, s_4_21, s_4_18);
        // N s_4_23: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var SYSm:u8
        let s_5_0: u8 = fn_state.SYSm;
        // C s_5_1: const #3s : i
        let s_5_1: i128 = 3;
        // D s_5_2: cast zx s_5_0 -> bv
        let s_5_2: Bits = Bits::new(s_5_0 as u128, 5u16);
        // C s_5_3: const #1s : i64
        let s_5_3: i64 = 1;
        // C s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: const #1s : i
        let s_5_5: i128 = 1;
        // C s_5_6: add s_5_5 s_5_4
        let s_5_6: i128 = (s_5_5 + s_5_4);
        // D s_5_7: bit-extract s_5_2 s_5_1 s_5_6
        let s_5_7: Bits = (Bits::new(
            ((s_5_2) >> (s_5_1)).value(),
            u16::try_from(s_5_6).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: u8 = (s_5_7.value() as u8);
        // D s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 2u16);
        // C s_5_10: const #1u : u8
        let s_5_10: u8 = 1;
        // C s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 2u16);
        // D s_5_12: cmp-eq s_5_9 s_5_11
        let s_5_12: bool = ((s_5_9) == (s_5_11));
        // D s_5_13: not s_5_12
        let s_5_13: bool = !s_5_12;
        // N s_5_14: branch s_5_13 b7 b6
        if s_5_13 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i
        let s_6_0: i128 = 0;
        // D s_6_1: read-var SYSm:u8
        let s_6_1: u8 = fn_state.SYSm;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 5u16);
        // C s_6_3: const #1s : i64
        let s_6_3: i64 = 1;
        // C s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // C s_6_5: const #2s : i
        let s_6_5: i128 = 2;
        // C s_6_6: add s_6_5 s_6_4
        let s_6_6: i128 = (s_6_5 + s_6_4);
        // D s_6_7: bit-extract s_6_2 s_6_0 s_6_6
        let s_6_7: Bits = (Bits::new(
            ((s_6_2) >> (s_6_0)).value(),
            u16::try_from(s_6_6).unwrap(),
        ));
        // D s_6_8: cast reint s_6_7 -> u8
        let s_6_8: u8 = (s_6_7.value() as u8);
        // D s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 3u16);
        // D s_6_10: cast zx s_6_9 -> i
        let s_6_10: i128 = (s_6_9.value() as i128);
        // D s_6_11: cast reint s_6_10 -> i64
        let s_6_11: i64 = (s_6_10 as i64);
        // C s_6_12: const #8s : i
        let s_6_12: i128 = 8;
        // D s_6_13: cast zx s_6_11 -> i
        let s_6_13: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_14: add s_6_13 s_6_12
        let s_6_14: i128 = (s_6_13 + s_6_12);
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: read-var n:i64
        let s_6_16: i64 = fn_state.n;
        // D s_6_17: cast zx s_6_16 -> i
        let s_6_17: i128 = (i128::try_from(s_6_16).unwrap());
        // D s_6_18: call R_read(s_6_17)
        let s_6_18: u32 = R_read(state, tracer, s_6_17);
        // D s_6_19: cast zx s_6_15 -> i
        let s_6_19: i128 = (i128::try_from(s_6_15).unwrap());
        // C s_6_20: const #360u : u32
        let s_6_20: u32 = 360;
        // D s_6_21: read-reg s_6_20:u8
        let s_6_21: u8 = {
            let value = state.read_register::<u8>(s_6_20 as isize);
            tracer.read_register(s_6_20 as isize, value);
            value
        };
        // D s_6_22: call Rmode_set(s_6_19, s_6_21, s_6_18)
        let s_6_22: () = Rmode_set(state, tracer, s_6_19, s_6_21, s_6_18);
        // N s_6_23: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var SYSm:u8
        let s_7_0: u8 = fn_state.SYSm;
        // C s_7_1: const #1s : i
        let s_7_1: i128 = 1;
        // D s_7_2: cast zx s_7_0 -> bv
        let s_7_2: Bits = Bits::new(s_7_0 as u128, 5u16);
        // C s_7_3: const #1s : i64
        let s_7_3: i64 = 1;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #3s : i
        let s_7_5: i128 = 3;
        // C s_7_6: add s_7_5 s_7_4
        let s_7_6: i128 = (s_7_5 + s_7_4);
        // D s_7_7: bit-extract s_7_2 s_7_1 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_2) >> (s_7_1)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: u8 = (s_7_7.value() as u8);
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 4u16);
        // C s_7_10: const #8u : u8
        let s_7_10: u8 = 8;
        // C s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 4u16);
        // D s_7_12: cmp-eq s_7_9 s_7_11
        let s_7_12: bool = ((s_7_9) == (s_7_11));
        // D s_7_13: not s_7_12
        let s_7_13: bool = !s_7_12;
        // N s_7_14: branch s_7_13 b9 b8
        if s_7_13 {
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
        // C s_8_0: const #0s : i
        let s_8_0: i128 = 0;
        // D s_8_1: read-var SYSm:u8
        let s_8_1: u8 = fn_state.SYSm;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 5u16);
        // C s_8_3: const #1u : u64
        let s_8_3: u64 = 1;
        // D s_8_4: bit-extract s_8_2 s_8_0 s_8_3
        let s_8_4: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_3).unwrap(),
        ));
        // D s_8_5: cast reint s_8_4 -> u8
        let s_8_5: bool = ((s_8_4.value()) != 0);
        // C s_8_6: const #0s : i
        let s_8_6: i128 = 0;
        // C s_8_7: const #0u : u64
        let s_8_7: u64 = 0;
        // D s_8_8: cast zx s_8_5 -> u64
        let s_8_8: u64 = (s_8_5 as u64);
        // C s_8_9: const #1u : u64
        let s_8_9: u64 = 1;
        // D s_8_10: and s_8_8 s_8_9
        let s_8_10: u64 = ((s_8_8) & (s_8_9));
        // D s_8_11: cmp-eq s_8_10 s_8_9
        let s_8_11: bool = ((s_8_10) == (s_8_9));
        // D s_8_12: lsl s_8_8 s_8_6
        let s_8_12: u64 = s_8_8 << s_8_6;
        // D s_8_13: or s_8_7 s_8_12
        let s_8_13: u64 = ((s_8_7) | (s_8_12));
        // D s_8_14: cmpl s_8_12
        let s_8_14: u64 = !s_8_12;
        // D s_8_15: and s_8_7 s_8_14
        let s_8_15: u64 = ((s_8_7) & (s_8_14));
        // D s_8_16: select s_8_11 s_8_13 s_8_15
        let s_8_16: u64 = if s_8_11 { s_8_13 } else { s_8_15 };
        // D s_8_17: cast trunc s_8_16 -> u8
        let s_8_17: bool = ((s_8_16) != 0);
        // D s_8_18: cast zx s_8_17 -> bv
        let s_8_18: Bits = Bits::new(s_8_17 as u128, 1u16);
        // D s_8_19: cast zx s_8_18 -> i
        let s_8_19: i128 = (s_8_18.value() as i128);
        // D s_8_20: cast reint s_8_19 -> i64
        let s_8_20: i64 = (s_8_19 as i64);
        // C s_8_21: const #14s : i
        let s_8_21: i128 = 14;
        // D s_8_22: cast zx s_8_20 -> i
        let s_8_22: i128 = (i128::try_from(s_8_20).unwrap());
        // D s_8_23: sub s_8_21 s_8_22
        let s_8_23: i128 = ((s_8_21) - (s_8_22));
        // D s_8_24: cast reint s_8_23 -> i64
        let s_8_24: i64 = (s_8_23 as i64);
        // D s_8_25: read-var n:i64
        let s_8_25: i64 = fn_state.n;
        // D s_8_26: cast zx s_8_25 -> i
        let s_8_26: i128 = (i128::try_from(s_8_25).unwrap());
        // D s_8_27: call R_read(s_8_26)
        let s_8_27: u32 = R_read(state, tracer, s_8_26);
        // D s_8_28: cast zx s_8_24 -> i
        let s_8_28: i128 = (i128::try_from(s_8_24).unwrap());
        // C s_8_29: const #368u : u32
        let s_8_29: u32 = 368;
        // D s_8_30: read-reg s_8_29:u8
        let s_8_30: u8 = {
            let value = state.read_register::<u8>(s_8_29 as isize);
            tracer.read_register(s_8_29 as isize, value);
            value
        };
        // D s_8_31: call Rmode_set(s_8_28, s_8_30, s_8_27)
        let s_8_31: () = Rmode_set(state, tracer, s_8_28, s_8_30, s_8_27);
        // N s_8_32: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var SYSm:u8
        let s_9_0: u8 = fn_state.SYSm;
        // C s_9_1: const #1s : i
        let s_9_1: i128 = 1;
        // D s_9_2: cast zx s_9_0 -> bv
        let s_9_2: Bits = Bits::new(s_9_0 as u128, 5u16);
        // C s_9_3: const #1s : i64
        let s_9_3: i64 = 1;
        // C s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // C s_9_5: const #3s : i
        let s_9_5: i128 = 3;
        // C s_9_6: add s_9_5 s_9_4
        let s_9_6: i128 = (s_9_5 + s_9_4);
        // D s_9_7: bit-extract s_9_2 s_9_1 s_9_6
        let s_9_7: Bits = (Bits::new(
            ((s_9_2) >> (s_9_1)).value(),
            u16::try_from(s_9_6).unwrap(),
        ));
        // D s_9_8: cast reint s_9_7 -> u8
        let s_9_8: u8 = (s_9_7.value() as u8);
        // D s_9_9: cast zx s_9_8 -> bv
        let s_9_9: Bits = Bits::new(s_9_8 as u128, 4u16);
        // C s_9_10: const #9u : u8
        let s_9_10: u8 = 9;
        // C s_9_11: cast zx s_9_10 -> bv
        let s_9_11: Bits = Bits::new(s_9_10 as u128, 4u16);
        // D s_9_12: cmp-eq s_9_9 s_9_11
        let s_9_12: bool = ((s_9_9) == (s_9_11));
        // D s_9_13: not s_9_12
        let s_9_13: bool = !s_9_12;
        // N s_9_14: branch s_9_13 b11 b10
        if s_9_13 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var SYSm:u8
        let s_10_1: u8 = fn_state.SYSm;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 5u16);
        // C s_10_3: const #1u : u64
        let s_10_3: u64 = 1;
        // D s_10_4: bit-extract s_10_2 s_10_0 s_10_3
        let s_10_4: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_3).unwrap(),
        ));
        // D s_10_5: cast reint s_10_4 -> u8
        let s_10_5: bool = ((s_10_4.value()) != 0);
        // C s_10_6: const #0s : i
        let s_10_6: i128 = 0;
        // C s_10_7: const #0u : u64
        let s_10_7: u64 = 0;
        // D s_10_8: cast zx s_10_5 -> u64
        let s_10_8: u64 = (s_10_5 as u64);
        // C s_10_9: const #1u : u64
        let s_10_9: u64 = 1;
        // D s_10_10: and s_10_8 s_10_9
        let s_10_10: u64 = ((s_10_8) & (s_10_9));
        // D s_10_11: cmp-eq s_10_10 s_10_9
        let s_10_11: bool = ((s_10_10) == (s_10_9));
        // D s_10_12: lsl s_10_8 s_10_6
        let s_10_12: u64 = s_10_8 << s_10_6;
        // D s_10_13: or s_10_7 s_10_12
        let s_10_13: u64 = ((s_10_7) | (s_10_12));
        // D s_10_14: cmpl s_10_12
        let s_10_14: u64 = !s_10_12;
        // D s_10_15: and s_10_7 s_10_14
        let s_10_15: u64 = ((s_10_7) & (s_10_14));
        // D s_10_16: select s_10_11 s_10_13 s_10_15
        let s_10_16: u64 = if s_10_11 { s_10_13 } else { s_10_15 };
        // D s_10_17: cast trunc s_10_16 -> u8
        let s_10_17: bool = ((s_10_16) != 0);
        // D s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 1u16);
        // D s_10_19: cast zx s_10_18 -> i
        let s_10_19: i128 = (s_10_18.value() as i128);
        // D s_10_20: cast reint s_10_19 -> i64
        let s_10_20: i64 = (s_10_19 as i64);
        // C s_10_21: const #14s : i
        let s_10_21: i128 = 14;
        // D s_10_22: cast zx s_10_20 -> i
        let s_10_22: i128 = (i128::try_from(s_10_20).unwrap());
        // D s_10_23: sub s_10_21 s_10_22
        let s_10_23: i128 = ((s_10_21) - (s_10_22));
        // D s_10_24: cast reint s_10_23 -> i64
        let s_10_24: i64 = (s_10_23 as i64);
        // D s_10_25: read-var n:i64
        let s_10_25: i64 = fn_state.n;
        // D s_10_26: cast zx s_10_25 -> i
        let s_10_26: i128 = (i128::try_from(s_10_25).unwrap());
        // D s_10_27: call R_read(s_10_26)
        let s_10_27: u32 = R_read(state, tracer, s_10_26);
        // D s_10_28: cast zx s_10_24 -> i
        let s_10_28: i128 = (i128::try_from(s_10_24).unwrap());
        // C s_10_29: const #376u : u32
        let s_10_29: u32 = 376;
        // D s_10_30: read-reg s_10_29:u8
        let s_10_30: u8 = {
            let value = state.read_register::<u8>(s_10_29 as isize);
            tracer.read_register(s_10_29 as isize, value);
            value
        };
        // D s_10_31: call Rmode_set(s_10_28, s_10_30, s_10_27)
        let s_10_31: () = Rmode_set(state, tracer, s_10_28, s_10_30, s_10_27);
        // N s_10_32: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var SYSm:u8
        let s_11_0: u8 = fn_state.SYSm;
        // C s_11_1: const #1s : i
        let s_11_1: i128 = 1;
        // D s_11_2: cast zx s_11_0 -> bv
        let s_11_2: Bits = Bits::new(s_11_0 as u128, 5u16);
        // C s_11_3: const #1s : i64
        let s_11_3: i64 = 1;
        // C s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // C s_11_5: const #3s : i
        let s_11_5: i128 = 3;
        // C s_11_6: add s_11_5 s_11_4
        let s_11_6: i128 = (s_11_5 + s_11_4);
        // D s_11_7: bit-extract s_11_2 s_11_1 s_11_6
        let s_11_7: Bits = (Bits::new(
            ((s_11_2) >> (s_11_1)).value(),
            u16::try_from(s_11_6).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u8
        let s_11_8: u8 = (s_11_7.value() as u8);
        // D s_11_9: cast zx s_11_8 -> bv
        let s_11_9: Bits = Bits::new(s_11_8 as u128, 4u16);
        // C s_11_10: const #10u : u8
        let s_11_10: u8 = 10;
        // C s_11_11: cast zx s_11_10 -> bv
        let s_11_11: Bits = Bits::new(s_11_10 as u128, 4u16);
        // D s_11_12: cmp-eq s_11_9 s_11_11
        let s_11_12: bool = ((s_11_9) == (s_11_11));
        // D s_11_13: not s_11_12
        let s_11_13: bool = !s_11_12;
        // N s_11_14: branch s_11_13 b13 b12
        if s_11_13 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var SYSm:u8
        let s_12_1: u8 = fn_state.SYSm;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 5u16);
        // C s_12_3: const #1u : u64
        let s_12_3: u64 = 1;
        // D s_12_4: bit-extract s_12_2 s_12_0 s_12_3
        let s_12_4: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_3).unwrap(),
        ));
        // D s_12_5: cast reint s_12_4 -> u8
        let s_12_5: bool = ((s_12_4.value()) != 0);
        // C s_12_6: const #0s : i
        let s_12_6: i128 = 0;
        // C s_12_7: const #0u : u64
        let s_12_7: u64 = 0;
        // D s_12_8: cast zx s_12_5 -> u64
        let s_12_8: u64 = (s_12_5 as u64);
        // C s_12_9: const #1u : u64
        let s_12_9: u64 = 1;
        // D s_12_10: and s_12_8 s_12_9
        let s_12_10: u64 = ((s_12_8) & (s_12_9));
        // D s_12_11: cmp-eq s_12_10 s_12_9
        let s_12_11: bool = ((s_12_10) == (s_12_9));
        // D s_12_12: lsl s_12_8 s_12_6
        let s_12_12: u64 = s_12_8 << s_12_6;
        // D s_12_13: or s_12_7 s_12_12
        let s_12_13: u64 = ((s_12_7) | (s_12_12));
        // D s_12_14: cmpl s_12_12
        let s_12_14: u64 = !s_12_12;
        // D s_12_15: and s_12_7 s_12_14
        let s_12_15: u64 = ((s_12_7) & (s_12_14));
        // D s_12_16: select s_12_11 s_12_13 s_12_15
        let s_12_16: u64 = if s_12_11 { s_12_13 } else { s_12_15 };
        // D s_12_17: cast trunc s_12_16 -> u8
        let s_12_17: bool = ((s_12_16) != 0);
        // D s_12_18: cast zx s_12_17 -> bv
        let s_12_18: Bits = Bits::new(s_12_17 as u128, 1u16);
        // D s_12_19: cast zx s_12_18 -> i
        let s_12_19: i128 = (s_12_18.value() as i128);
        // D s_12_20: cast reint s_12_19 -> i64
        let s_12_20: i64 = (s_12_19 as i64);
        // C s_12_21: const #14s : i
        let s_12_21: i128 = 14;
        // D s_12_22: cast zx s_12_20 -> i
        let s_12_22: i128 = (i128::try_from(s_12_20).unwrap());
        // D s_12_23: sub s_12_21 s_12_22
        let s_12_23: i128 = ((s_12_21) - (s_12_22));
        // D s_12_24: cast reint s_12_23 -> i64
        let s_12_24: i64 = (s_12_23 as i64);
        // D s_12_25: read-var n:i64
        let s_12_25: i64 = fn_state.n;
        // D s_12_26: cast zx s_12_25 -> i
        let s_12_26: i128 = (i128::try_from(s_12_25).unwrap());
        // D s_12_27: call R_read(s_12_26)
        let s_12_27: u32 = R_read(state, tracer, s_12_26);
        // D s_12_28: cast zx s_12_24 -> i
        let s_12_28: i128 = (i128::try_from(s_12_24).unwrap());
        // C s_12_29: const #392u : u32
        let s_12_29: u32 = 392;
        // D s_12_30: read-reg s_12_29:u8
        let s_12_30: u8 = {
            let value = state.read_register::<u8>(s_12_29 as isize);
            tracer.read_register(s_12_29 as isize, value);
            value
        };
        // D s_12_31: call Rmode_set(s_12_28, s_12_30, s_12_27)
        let s_12_31: () = Rmode_set(state, tracer, s_12_28, s_12_30, s_12_27);
        // N s_12_32: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var SYSm:u8
        let s_13_0: u8 = fn_state.SYSm;
        // C s_13_1: const #1s : i
        let s_13_1: i128 = 1;
        // D s_13_2: cast zx s_13_0 -> bv
        let s_13_2: Bits = Bits::new(s_13_0 as u128, 5u16);
        // C s_13_3: const #1s : i64
        let s_13_3: i64 = 1;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #3s : i
        let s_13_5: i128 = 3;
        // C s_13_6: add s_13_5 s_13_4
        let s_13_6: i128 = (s_13_5 + s_13_4);
        // D s_13_7: bit-extract s_13_2 s_13_1 s_13_6
        let s_13_7: Bits = (Bits::new(
            ((s_13_2) >> (s_13_1)).value(),
            u16::try_from(s_13_6).unwrap(),
        ));
        // D s_13_8: cast reint s_13_7 -> u8
        let s_13_8: u8 = (s_13_7.value() as u8);
        // D s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 4u16);
        // C s_13_10: const #11u : u8
        let s_13_10: u8 = 11;
        // C s_13_11: cast zx s_13_10 -> bv
        let s_13_11: Bits = Bits::new(s_13_10 as u128, 4u16);
        // D s_13_12: cmp-eq s_13_9 s_13_11
        let s_13_12: bool = ((s_13_9) == (s_13_11));
        // D s_13_13: not s_13_12
        let s_13_13: bool = !s_13_12;
        // N s_13_14: branch s_13_13 b15 b14
        if s_13_13 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var SYSm:u8
        let s_14_1: u8 = fn_state.SYSm;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 5u16);
        // C s_14_3: const #1u : u64
        let s_14_3: u64 = 1;
        // D s_14_4: bit-extract s_14_2 s_14_0 s_14_3
        let s_14_4: Bits = (Bits::new(
            ((s_14_2) >> (s_14_0)).value(),
            u16::try_from(s_14_3).unwrap(),
        ));
        // D s_14_5: cast reint s_14_4 -> u8
        let s_14_5: bool = ((s_14_4.value()) != 0);
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // C s_14_7: const #0u : u64
        let s_14_7: u64 = 0;
        // D s_14_8: cast zx s_14_5 -> u64
        let s_14_8: u64 = (s_14_5 as u64);
        // C s_14_9: const #1u : u64
        let s_14_9: u64 = 1;
        // D s_14_10: and s_14_8 s_14_9
        let s_14_10: u64 = ((s_14_8) & (s_14_9));
        // D s_14_11: cmp-eq s_14_10 s_14_9
        let s_14_11: bool = ((s_14_10) == (s_14_9));
        // D s_14_12: lsl s_14_8 s_14_6
        let s_14_12: u64 = s_14_8 << s_14_6;
        // D s_14_13: or s_14_7 s_14_12
        let s_14_13: u64 = ((s_14_7) | (s_14_12));
        // D s_14_14: cmpl s_14_12
        let s_14_14: u64 = !s_14_12;
        // D s_14_15: and s_14_7 s_14_14
        let s_14_15: u64 = ((s_14_7) & (s_14_14));
        // D s_14_16: select s_14_11 s_14_13 s_14_15
        let s_14_16: u64 = if s_14_11 { s_14_13 } else { s_14_15 };
        // D s_14_17: cast trunc s_14_16 -> u8
        let s_14_17: bool = ((s_14_16) != 0);
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 1u16);
        // D s_14_19: cast zx s_14_18 -> i
        let s_14_19: i128 = (s_14_18.value() as i128);
        // D s_14_20: cast reint s_14_19 -> i64
        let s_14_20: i64 = (s_14_19 as i64);
        // C s_14_21: const #14s : i
        let s_14_21: i128 = 14;
        // D s_14_22: cast zx s_14_20 -> i
        let s_14_22: i128 = (i128::try_from(s_14_20).unwrap());
        // D s_14_23: sub s_14_21 s_14_22
        let s_14_23: i128 = ((s_14_21) - (s_14_22));
        // D s_14_24: cast reint s_14_23 -> i64
        let s_14_24: i64 = (s_14_23 as i64);
        // D s_14_25: read-var n:i64
        let s_14_25: i64 = fn_state.n;
        // D s_14_26: cast zx s_14_25 -> i
        let s_14_26: i128 = (i128::try_from(s_14_25).unwrap());
        // D s_14_27: call R_read(s_14_26)
        let s_14_27: u32 = R_read(state, tracer, s_14_26);
        // D s_14_28: cast zx s_14_24 -> i
        let s_14_28: i128 = (i128::try_from(s_14_24).unwrap());
        // C s_14_29: const #408u : u32
        let s_14_29: u32 = 408;
        // D s_14_30: read-reg s_14_29:u8
        let s_14_30: u8 = {
            let value = state.read_register::<u8>(s_14_29 as isize);
            tracer.read_register(s_14_29 as isize, value);
            value
        };
        // D s_14_31: call Rmode_set(s_14_28, s_14_30, s_14_27)
        let s_14_31: () = Rmode_set(state, tracer, s_14_28, s_14_30, s_14_27);
        // N s_14_32: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var SYSm:u8
        let s_15_0: u8 = fn_state.SYSm;
        // C s_15_1: const #1s : i
        let s_15_1: i128 = 1;
        // D s_15_2: cast zx s_15_0 -> bv
        let s_15_2: Bits = Bits::new(s_15_0 as u128, 5u16);
        // C s_15_3: const #1s : i64
        let s_15_3: i64 = 1;
        // C s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // C s_15_5: const #3s : i
        let s_15_5: i128 = 3;
        // C s_15_6: add s_15_5 s_15_4
        let s_15_6: i128 = (s_15_5 + s_15_4);
        // D s_15_7: bit-extract s_15_2 s_15_1 s_15_6
        let s_15_7: Bits = (Bits::new(
            ((s_15_2) >> (s_15_1)).value(),
            u16::try_from(s_15_6).unwrap(),
        ));
        // D s_15_8: cast reint s_15_7 -> u8
        let s_15_8: u8 = (s_15_7.value() as u8);
        // D s_15_9: cast zx s_15_8 -> bv
        let s_15_9: Bits = Bits::new(s_15_8 as u128, 4u16);
        // C s_15_10: const #14u : u8
        let s_15_10: u8 = 14;
        // C s_15_11: cast zx s_15_10 -> bv
        let s_15_11: Bits = Bits::new(s_15_10 as u128, 4u16);
        // D s_15_12: cmp-eq s_15_9 s_15_11
        let s_15_12: bool = ((s_15_9) == (s_15_11));
        // D s_15_13: not s_15_12
        let s_15_13: bool = !s_15_12;
        // N s_15_14: branch s_15_13 b20 b16
        if s_15_13 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #424u : u32
        let s_16_0: u32 = 424;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call ELUsingAArch32(s_16_1)
        let s_16_2: bool = ELUsingAArch32(state, tracer, s_16_1);
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b19 b17
        if s_16_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0s : i
        let s_18_0: i128 = 0;
        // D s_18_1: read-var SYSm:u8
        let s_18_1: u8 = fn_state.SYSm;
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 5u16);
        // C s_18_3: const #1u : u64
        let s_18_3: u64 = 1;
        // D s_18_4: bit-extract s_18_2 s_18_0 s_18_3
        let s_18_4: Bits = (Bits::new(
            ((s_18_2) >> (s_18_0)).value(),
            u16::try_from(s_18_3).unwrap(),
        ));
        // D s_18_5: cast reint s_18_4 -> u8
        let s_18_5: bool = ((s_18_4.value()) != 0);
        // C s_18_6: const #0s : i
        let s_18_6: i128 = 0;
        // C s_18_7: const #0u : u64
        let s_18_7: u64 = 0;
        // D s_18_8: cast zx s_18_5 -> u64
        let s_18_8: u64 = (s_18_5 as u64);
        // C s_18_9: const #1u : u64
        let s_18_9: u64 = 1;
        // D s_18_10: and s_18_8 s_18_9
        let s_18_10: u64 = ((s_18_8) & (s_18_9));
        // D s_18_11: cmp-eq s_18_10 s_18_9
        let s_18_11: bool = ((s_18_10) == (s_18_9));
        // D s_18_12: lsl s_18_8 s_18_6
        let s_18_12: u64 = s_18_8 << s_18_6;
        // D s_18_13: or s_18_7 s_18_12
        let s_18_13: u64 = ((s_18_7) | (s_18_12));
        // D s_18_14: cmpl s_18_12
        let s_18_14: u64 = !s_18_12;
        // D s_18_15: and s_18_7 s_18_14
        let s_18_15: u64 = ((s_18_7) & (s_18_14));
        // D s_18_16: select s_18_11 s_18_13 s_18_15
        let s_18_16: u64 = if s_18_11 { s_18_13 } else { s_18_15 };
        // D s_18_17: cast trunc s_18_16 -> u8
        let s_18_17: bool = ((s_18_16) != 0);
        // D s_18_18: cast zx s_18_17 -> bv
        let s_18_18: Bits = Bits::new(s_18_17 as u128, 1u16);
        // D s_18_19: cast zx s_18_18 -> i
        let s_18_19: i128 = (s_18_18.value() as i128);
        // D s_18_20: cast reint s_18_19 -> i64
        let s_18_20: i64 = (s_18_19 as i64);
        // C s_18_21: const #14s : i
        let s_18_21: i128 = 14;
        // D s_18_22: cast zx s_18_20 -> i
        let s_18_22: i128 = (i128::try_from(s_18_20).unwrap());
        // D s_18_23: sub s_18_21 s_18_22
        let s_18_23: i128 = ((s_18_21) - (s_18_22));
        // D s_18_24: cast reint s_18_23 -> i64
        let s_18_24: i64 = (s_18_23 as i64);
        // D s_18_25: read-var n:i64
        let s_18_25: i64 = fn_state.n;
        // D s_18_26: cast zx s_18_25 -> i
        let s_18_26: i128 = (i128::try_from(s_18_25).unwrap());
        // D s_18_27: call R_read(s_18_26)
        let s_18_27: u32 = R_read(state, tracer, s_18_26);
        // D s_18_28: cast zx s_18_24 -> i
        let s_18_28: i128 = (i128::try_from(s_18_24).unwrap());
        // C s_18_29: const #384u : u32
        let s_18_29: u32 = 384;
        // D s_18_30: read-reg s_18_29:u8
        let s_18_30: u8 = {
            let value = state.read_register::<u8>(s_18_29 as isize);
            tracer.read_register(s_18_29 as isize, value);
            value
        };
        // D s_18_31: call Rmode_set(s_18_28, s_18_30, s_18_27)
        let s_18_31: () = Rmode_set(state, tracer, s_18_28, s_18_30, s_18_27);
        // N s_18_32: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call AArch64_MonitorModeTrap(s_19_0)
        let s_19_1: () = AArch64_MonitorModeTrap(state, tracer, s_19_0);
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var SYSm:u8
        let s_20_0: u8 = fn_state.SYSm;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 5u16);
        // C s_20_2: const #30u : u8
        let s_20_2: u8 = 30;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 5u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b22 b21
        if s_20_5 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var n:i64
        let s_21_0: i64 = fn_state.n;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call R_read(s_21_1)
        let s_21_2: u32 = R_read(state, tracer, s_21_1);
        // D s_21_3: call ELR_hyp_write(s_21_2)
        let s_21_3: () = ELR_hyp_write(state, tracer, s_21_2);
        // N s_21_4: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var SYSm:u8
        let s_22_0: u8 = fn_state.SYSm;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 5u16);
        // C s_22_2: const #31u : u8
        let s_22_2: u8 = 31;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 5u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: not s_22_4
        let s_22_5: bool = !s_22_4;
        // N s_22_6: branch s_22_5 b24 b23
        if s_22_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var n:i64
        let s_23_0: i64 = fn_state.n;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: call R_read(s_23_1)
        let s_23_2: u32 = R_read(state, tracer, s_23_1);
        // C s_23_3: const #13s : i
        let s_23_3: i128 = 13;
        // C s_23_4: const #400u : u32
        let s_23_4: u32 = 400;
        // D s_23_5: read-reg s_23_4:u8
        let s_23_5: u8 = {
            let value = state.read_register::<u8>(s_23_4 as isize);
            tracer.read_register(s_23_4 as isize, value);
            value
        };
        // D s_23_6: call Rmode_set(s_23_3, s_23_5, s_23_2)
        let s_23_6: () = Rmode_set(state, tracer, s_23_3, s_23_5, s_23_2);
        // N s_23_7: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var SYSm:u8
        let s_25_0: u8 = fn_state.SYSm;
        // D s_25_1: read-var modeshadow#7896:u8
        let s_25_1: u8 = fn_state.modeshadow_7896;
        // D s_25_2: call SPSRaccessValid(s_25_0, s_25_1)
        let s_25_2: () = SPSRaccessValid(state, tracer, s_25_0, s_25_1);
        // N s_25_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var SYSm:u8
        let s_26_0: u8 = fn_state.SYSm;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 5u16);
        // C s_26_2: const #14u : u8
        let s_26_2: u8 = 14;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 5u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: not s_26_4
        let s_26_5: bool = !s_26_4;
        // N s_26_6: branch s_26_5 b28 b27
        if s_26_5 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var n:i64
        let s_27_0: i64 = fn_state.n;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call R_read(s_27_1)
        let s_27_2: u32 = R_read(state, tracer, s_27_1);
        // C s_27_3: const #15536u : u32
        let s_27_3: u32 = 15536;
        // D s_27_4: read-reg s_27_3:struct
        let s_27_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_3 as isize);
            tracer.read_register(s_27_3 as isize, value);
            value
        };
        // C s_27_5: const #15536u : u32
        let s_27_5: u32 = 15536;
        // N s_27_6: write-reg s_27_5 <= s_27_4
        let s_27_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_27_5 as isize, s_27_4);
            tracer.write_register(s_27_5 as isize, s_27_4);
        };
        // N s_27_7: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var SYSm:u8
        let s_28_0: u8 = fn_state.SYSm;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 5u16);
        // C s_28_2: const #16u : u8
        let s_28_2: u8 = 16;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 5u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: not s_28_4
        let s_28_5: bool = !s_28_4;
        // N s_28_6: branch s_28_5 b30 b29
        if s_28_5 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var n:i64
        let s_29_0: i64 = fn_state.n;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: call R_read(s_29_1)
        let s_29_2: u32 = R_read(state, tracer, s_29_1);
        // C s_29_3: const #91016u : u32
        let s_29_3: u32 = 91016;
        // D s_29_4: read-reg s_29_3:struct
        let s_29_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_3 as isize);
            tracer.read_register(s_29_3 as isize, value);
            value
        };
        // C s_29_5: const #91016u : u32
        let s_29_5: u32 = 91016;
        // N s_29_6: write-reg s_29_5 <= s_29_4
        let s_29_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_29_5 as isize, s_29_4);
            tracer.write_register(s_29_5 as isize, s_29_4);
        };
        // N s_29_7: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var SYSm:u8
        let s_30_0: u8 = fn_state.SYSm;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 5u16);
        // C s_30_2: const #18u : u8
        let s_30_2: u8 = 18;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 5u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: not s_30_4
        let s_30_5: bool = !s_30_4;
        // N s_30_6: branch s_30_5 b32 b31
        if s_30_5 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call SPSR_svc_read(s_31_0)
        let s_31_1: ProductType700c18a878c5601b = SPSR_svc_read(state, tracer, s_31_0);
        // D s_31_2: write-var ga#364099 <= s_31_1
        fn_state.ga_364099 = s_31_1;
        // D s_31_3: read-var ga#364099.0:struct
        let s_31_3: u32 = fn_state.ga_364099._0;
        // D s_31_4: read-var n:i64
        let s_31_4: i64 = fn_state.n;
        // D s_31_5: cast zx s_31_4 -> i
        let s_31_5: i128 = (i128::try_from(s_31_4).unwrap());
        // D s_31_6: call R_read(s_31_5)
        let s_31_6: u32 = R_read(state, tracer, s_31_5);
        // C s_31_7: const #0s : i
        let s_31_7: i128 = 0;
        // D s_31_8: cast zx s_31_3 -> bv
        let s_31_8: Bits = Bits::new(s_31_3 as u128, 32u16);
        // D s_31_9: cast zx s_31_6 -> bv
        let s_31_9: Bits = Bits::new(s_31_6 as u128, 32u16);
        // C s_31_10: const #31s : i
        let s_31_10: i128 = 31;
        // C s_31_11: const #1u : u64
        let s_31_11: u64 = 1;
        // C s_31_12: cast zx s_31_11 -> bv
        let s_31_12: Bits = Bits::new(s_31_11 as u128, 64u16);
        // C s_31_13: lsl s_31_12 s_31_10
        let s_31_13: Bits = s_31_12 << s_31_10;
        // C s_31_14: sub s_31_13 s_31_12
        let s_31_14: Bits = ((s_31_13) - (s_31_12));
        // D s_31_15: and s_31_9 s_31_14
        let s_31_15: Bits = ((s_31_9) & (s_31_14));
        // D s_31_16: lsl s_31_15 s_31_7
        let s_31_16: Bits = s_31_15 << s_31_7;
        // C s_31_17: lsl s_31_14 s_31_7
        let s_31_17: Bits = s_31_14 << s_31_7;
        // C s_31_18: cmpl s_31_17
        let s_31_18: Bits = !s_31_17;
        // D s_31_19: and s_31_8 s_31_18
        let s_31_19: Bits = ((s_31_8) & (s_31_18));
        // D s_31_20: or s_31_19 s_31_16
        let s_31_20: Bits = ((s_31_19) | (s_31_16));
        // D s_31_21: cast reint s_31_20 -> u32
        let s_31_21: u32 = (s_31_20.value() as u32);
        // D s_31_22: call Mk_SPSR_svc_Type(s_31_21)
        let s_31_22: ProductType700c18a878c5601b = Mk_SPSR_svc_Type(
            state,
            tracer,
            s_31_21,
        );
        // D s_31_23: call SPSR_svc_write(s_31_22)
        let s_31_23: () = SPSR_svc_write(state, tracer, s_31_22);
        // N s_31_24: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var SYSm:u8
        let s_32_0: u8 = fn_state.SYSm;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 5u16);
        // C s_32_2: const #20u : u8
        let s_32_2: u8 = 20;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 5u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: not s_32_4
        let s_32_5: bool = !s_32_4;
        // N s_32_6: branch s_32_5 b34 b33
        if s_32_5 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var n:i64
        let s_33_0: i64 = fn_state.n;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call R_read(s_33_1)
        let s_33_2: u32 = R_read(state, tracer, s_33_1);
        // C s_33_3: const #20032u : u32
        let s_33_3: u32 = 20032;
        // D s_33_4: read-reg s_33_3:struct
        let s_33_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_33_3 as isize);
            tracer.read_register(s_33_3 as isize, value);
            value
        };
        // C s_33_5: const #20032u : u32
        let s_33_5: u32 = 20032;
        // N s_33_6: write-reg s_33_5 <= s_33_4
        let s_33_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_33_5 as isize, s_33_4);
            tracer.write_register(s_33_5 as isize, s_33_4);
        };
        // N s_33_7: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var SYSm:u8
        let s_34_0: u8 = fn_state.SYSm;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 5u16);
        // C s_34_2: const #22u : u8
        let s_34_2: u8 = 22;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 5u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: not s_34_4
        let s_34_5: bool = !s_34_4;
        // N s_34_6: branch s_34_5 b36 b35
        if s_34_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var n:i64
        let s_35_0: i64 = fn_state.n;
        // D s_35_1: cast zx s_35_0 -> i
        let s_35_1: i128 = (i128::try_from(s_35_0).unwrap());
        // D s_35_2: call R_read(s_35_1)
        let s_35_2: u32 = R_read(state, tracer, s_35_1);
        // C s_35_3: const #18424u : u32
        let s_35_3: u32 = 18424;
        // D s_35_4: read-reg s_35_3:struct
        let s_35_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_3 as isize);
            tracer.read_register(s_35_3 as isize, value);
            value
        };
        // C s_35_5: const #18424u : u32
        let s_35_5: u32 = 18424;
        // N s_35_6: write-reg s_35_5 <= s_35_4
        let s_35_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_35_5 as isize, s_35_4);
            tracer.write_register(s_35_5 as isize, s_35_4);
        };
        // N s_35_7: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var SYSm:u8
        let s_36_0: u8 = fn_state.SYSm;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 5u16);
        // C s_36_2: const #28u : u8
        let s_36_2: u8 = 28;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 5u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: not s_36_4
        let s_36_5: bool = !s_36_4;
        // N s_36_6: branch s_36_5 b41 b37
        if s_36_5 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #424u : u32
        let s_37_0: u32 = 424;
        // D s_37_1: read-reg s_37_0:u8
        let s_37_1: u8 = {
            let value = state.read_register::<u8>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call ELUsingAArch32(s_37_1)
        let s_37_2: bool = ELUsingAArch32(state, tracer, s_37_1);
        // D s_37_3: not s_37_2
        let s_37_3: bool = !s_37_2;
        // N s_37_4: branch s_37_3 b40 b38
        if s_37_3 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var n:i64
        let s_39_0: i64 = fn_state.n;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: call R_read(s_39_1)
        let s_39_2: u32 = R_read(state, tracer, s_39_1);
        // C s_39_3: const #21136u : u32
        let s_39_3: u32 = 21136;
        // D s_39_4: read-reg s_39_3:struct
        let s_39_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_39_3 as isize);
            tracer.read_register(s_39_3 as isize, value);
            value
        };
        // C s_39_5: const #21136u : u32
        let s_39_5: u32 = 21136;
        // N s_39_6: write-reg s_39_5 <= s_39_4
        let s_39_6: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_39_5 as isize, s_39_4);
            tracer.write_register(s_39_5 as isize, s_39_4);
        };
        // N s_39_7: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call AArch64_MonitorModeTrap(s_40_0)
        let s_40_1: () = AArch64_MonitorModeTrap(state, tracer, s_40_0);
        // N s_40_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var SYSm:u8
        let s_41_0: u8 = fn_state.SYSm;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 5u16);
        // C s_41_2: const #30u : u8
        let s_41_2: u8 = 30;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 5u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: not s_41_4
        let s_41_5: bool = !s_41_4;
        // N s_41_6: branch s_41_5 b43 b42
        if s_41_5 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call SPSR_hyp_read(s_42_0)
        let s_42_1: ProductType700c18a878c5601b = SPSR_hyp_read(state, tracer, s_42_0);
        // D s_42_2: write-var ga#364115 <= s_42_1
        fn_state.ga_364115 = s_42_1;
        // D s_42_3: read-var ga#364115.0:struct
        let s_42_3: u32 = fn_state.ga_364115._0;
        // D s_42_4: read-var n:i64
        let s_42_4: i64 = fn_state.n;
        // D s_42_5: cast zx s_42_4 -> i
        let s_42_5: i128 = (i128::try_from(s_42_4).unwrap());
        // D s_42_6: call R_read(s_42_5)
        let s_42_6: u32 = R_read(state, tracer, s_42_5);
        // C s_42_7: const #0s : i
        let s_42_7: i128 = 0;
        // D s_42_8: cast zx s_42_3 -> bv
        let s_42_8: Bits = Bits::new(s_42_3 as u128, 32u16);
        // D s_42_9: cast zx s_42_6 -> bv
        let s_42_9: Bits = Bits::new(s_42_6 as u128, 32u16);
        // C s_42_10: const #31s : i
        let s_42_10: i128 = 31;
        // C s_42_11: const #1u : u64
        let s_42_11: u64 = 1;
        // C s_42_12: cast zx s_42_11 -> bv
        let s_42_12: Bits = Bits::new(s_42_11 as u128, 64u16);
        // C s_42_13: lsl s_42_12 s_42_10
        let s_42_13: Bits = s_42_12 << s_42_10;
        // C s_42_14: sub s_42_13 s_42_12
        let s_42_14: Bits = ((s_42_13) - (s_42_12));
        // D s_42_15: and s_42_9 s_42_14
        let s_42_15: Bits = ((s_42_9) & (s_42_14));
        // D s_42_16: lsl s_42_15 s_42_7
        let s_42_16: Bits = s_42_15 << s_42_7;
        // C s_42_17: lsl s_42_14 s_42_7
        let s_42_17: Bits = s_42_14 << s_42_7;
        // C s_42_18: cmpl s_42_17
        let s_42_18: Bits = !s_42_17;
        // D s_42_19: and s_42_8 s_42_18
        let s_42_19: Bits = ((s_42_8) & (s_42_18));
        // D s_42_20: or s_42_19 s_42_16
        let s_42_20: Bits = ((s_42_19) | (s_42_16));
        // D s_42_21: cast reint s_42_20 -> u32
        let s_42_21: u32 = (s_42_20.value() as u32);
        // D s_42_22: call Mk_SPSR_hyp_Type(s_42_21)
        let s_42_22: ProductType700c18a878c5601b = Mk_SPSR_hyp_Type(
            state,
            tracer,
            s_42_21,
        );
        // D s_42_23: call SPSR_hyp_write(s_42_22)
        let s_42_23: () = SPSR_hyp_write(state, tracer, s_42_22);
        // N s_42_24: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: panic
        panic!("{:?}", ());
        // N s_44_1: return
        return;
    }
}
