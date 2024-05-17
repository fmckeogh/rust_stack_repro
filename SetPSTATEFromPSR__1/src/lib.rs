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
use DebugExceptionReturnSS::*;
use ConstrainUnpredictableBool::*;
use HaveDITExt::*;
use u__UNKNOWN_bit::*;
use HaveFeatNMI::*;
use HaveBTIExt::*;
use HaveGCS::*;
use HavePANExt::*;
use RestoredITBits::*;
use u__UNKNOWN_bits::*;
use HaveSSBSExt::*;
use HaveUAOExt::*;
use HaveMTEExt::*;
use AArch32_WriteMode::*;
use Bit::*;
use UsingAArch32::*;
use Restarting::*;
use common::*;
pub fn SetPSTATEFromPSR__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    spsr_in: Bits,
    illegal_psr_state: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        from_aarch64: bool,
        gs_5530: bool,
        spsr: Bits,
        gs_5581: bool,
        spsr_in: Bits,
        illegal_psr_state: bool,
    }
    let fn_state = FunctionState {
        spsr_in,
        illegal_psr_state,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var spsr_in:bv
        let s_0_0: Bits = fn_state.spsr_in;
        // D s_0_1: write-var spsr <= s_0_0
        fn_state.spsr = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call UsingAArch32(s_0_2)
        let s_0_3: bool = UsingAArch32(state, tracer, s_0_2);
        // S s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // D s_0_5: write-var from_aarch64 <= s_0_4
        fn_state.from_aarch64 = s_0_4;
        // D s_0_6: read-var spsr:bv
        let s_0_6: Bits = fn_state.spsr;
        // D s_0_7: call DebugExceptionReturnSS(s_0_6)
        let s_0_7: bool = DebugExceptionReturnSS(state, tracer, s_0_6);
        // C s_0_8: const #16991u : u32
        let s_0_8: u32 = 16991;
        // N s_0_9: write-reg s_0_8 <= s_0_7
        let s_0_9: () = {
            state.write_register::<bool>(s_0_8 as isize, s_0_7);
            tracer.write_register(s_0_8 as isize, s_0_7);
        };
        // C s_0_10: const #0u : u8
        let s_0_10: bool = false;
        // C s_0_11: const #22992u : u32
        let s_0_11: u32 = 22992;
        // N s_0_12: write-reg s_0_11 <= s_0_10
        let s_0_12: () = {
            state.write_register::<bool>(s_0_11 as isize, s_0_10);
            tracer.write_register(s_0_11 as isize, s_0_10);
        };
        // D s_0_13: read-var illegal_psr_state:u8
        let s_0_13: bool = fn_state.illegal_psr_state;
        // N s_0_14: branch s_0_13 b52 b1
        if s_0_13 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var spsr_in:bv
        let s_1_0: Bits = fn_state.spsr_in;
        // D s_1_1: size-of s_1_0
        let s_1_1: u16 = s_1_0.length();
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // C s_1_3: const #20s : i
        let s_1_3: i128 = 20;
        // D s_1_4: cmp-lt s_1_3 s_1_2
        let s_1_4: bool = ((s_1_3) < (s_1_2));
        // N s_1_5: assert s_1_4
        let s_1_5: () = assert!(s_1_4);
        // C s_1_6: const #20s : i
        let s_1_6: i128 = 20;
        // D s_1_7: read-var spsr:bv
        let s_1_7: Bits = fn_state.spsr;
        // C s_1_8: const #1u : u64
        let s_1_8: u64 = 1;
        // D s_1_9: bit-extract s_1_7 s_1_6 s_1_8
        let s_1_9: Bits = (Bits::new(
            ((s_1_7) >> (s_1_6)).value(),
            u16::try_from(s_1_8).unwrap(),
        ));
        // D s_1_10: cast reint s_1_9 -> u8
        let s_1_10: bool = ((s_1_9.value()) != 0);
        // C s_1_11: const #0s : i
        let s_1_11: i128 = 0;
        // C s_1_12: const #0u : u64
        let s_1_12: u64 = 0;
        // D s_1_13: cast zx s_1_10 -> u64
        let s_1_13: u64 = (s_1_10 as u64);
        // C s_1_14: const #1u : u64
        let s_1_14: u64 = 1;
        // D s_1_15: and s_1_13 s_1_14
        let s_1_15: u64 = ((s_1_13) & (s_1_14));
        // D s_1_16: cmp-eq s_1_15 s_1_14
        let s_1_16: bool = ((s_1_15) == (s_1_14));
        // D s_1_17: lsl s_1_13 s_1_11
        let s_1_17: u64 = s_1_13 << s_1_11;
        // D s_1_18: or s_1_12 s_1_17
        let s_1_18: u64 = ((s_1_12) | (s_1_17));
        // D s_1_19: cmpl s_1_17
        let s_1_19: u64 = !s_1_17;
        // D s_1_20: and s_1_12 s_1_19
        let s_1_20: u64 = ((s_1_12) & (s_1_19));
        // D s_1_21: select s_1_16 s_1_18 s_1_20
        let s_1_21: u64 = if s_1_16 { s_1_18 } else { s_1_20 };
        // D s_1_22: cast trunc s_1_21 -> u8
        let s_1_22: bool = ((s_1_21) != 0);
        // C s_1_23: const #16980u : u32
        let s_1_23: u32 = 16980;
        // N s_1_24: write-reg s_1_23 <= s_1_22
        let s_1_24: () = {
            state.write_register::<bool>(s_1_23 as isize, s_1_22);
            tracer.write_register(s_1_23 as isize, s_1_22);
        };
        // C s_1_25: const #4s : i
        let s_1_25: i128 = 4;
        // D s_1_26: read-var spsr:bv
        let s_1_26: Bits = fn_state.spsr;
        // C s_1_27: const #1u : u64
        let s_1_27: u64 = 1;
        // D s_1_28: bit-extract s_1_26 s_1_25 s_1_27
        let s_1_28: Bits = (Bits::new(
            ((s_1_26) >> (s_1_25)).value(),
            u16::try_from(s_1_27).unwrap(),
        ));
        // D s_1_29: cast reint s_1_28 -> u8
        let s_1_29: bool = ((s_1_28.value()) != 0);
        // C s_1_30: const #0s : i
        let s_1_30: i128 = 0;
        // C s_1_31: const #0u : u64
        let s_1_31: u64 = 0;
        // D s_1_32: cast zx s_1_29 -> u64
        let s_1_32: u64 = (s_1_29 as u64);
        // C s_1_33: const #1u : u64
        let s_1_33: u64 = 1;
        // D s_1_34: and s_1_32 s_1_33
        let s_1_34: u64 = ((s_1_32) & (s_1_33));
        // D s_1_35: cmp-eq s_1_34 s_1_33
        let s_1_35: bool = ((s_1_34) == (s_1_33));
        // D s_1_36: lsl s_1_32 s_1_30
        let s_1_36: u64 = s_1_32 << s_1_30;
        // D s_1_37: or s_1_31 s_1_36
        let s_1_37: u64 = ((s_1_31) | (s_1_36));
        // D s_1_38: cmpl s_1_36
        let s_1_38: u64 = !s_1_36;
        // D s_1_39: and s_1_31 s_1_38
        let s_1_39: u64 = ((s_1_31) & (s_1_38));
        // D s_1_40: select s_1_35 s_1_37 s_1_39
        let s_1_40: u64 = if s_1_35 { s_1_37 } else { s_1_39 };
        // D s_1_41: cast trunc s_1_40 -> u8
        let s_1_41: bool = ((s_1_40) != 0);
        // D s_1_42: cast zx s_1_41 -> bv
        let s_1_42: Bits = Bits::new(s_1_41 as u128, 1u16);
        // C s_1_43: const #1u : u8
        let s_1_43: bool = true;
        // C s_1_44: cast zx s_1_43 -> bv
        let s_1_44: Bits = Bits::new(s_1_43 as u128, 1u16);
        // D s_1_45: cmp-eq s_1_42 s_1_44
        let s_1_45: bool = ((s_1_42) == (s_1_44));
        // N s_1_46: branch s_1_45 b48 b2
        if s_1_45 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // C s_2_1: const #16999u : u32
        let s_2_1: u32 = 16999;
        // N s_2_2: write-reg s_2_1 <= s_2_0
        let s_2_2: () = {
            state.write_register::<bool>(s_2_1 as isize, s_2_0);
            tracer.write_register(s_2_1 as isize, s_2_0);
        };
        // C s_2_3: const #2s : i
        let s_2_3: i128 = 2;
        // D s_2_4: read-var spsr:bv
        let s_2_4: Bits = fn_state.spsr;
        // C s_2_5: const #1s : i64
        let s_2_5: i64 = 1;
        // C s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // C s_2_7: const #1s : i
        let s_2_7: i128 = 1;
        // C s_2_8: add s_2_7 s_2_6
        let s_2_8: i128 = (s_2_7 + s_2_6);
        // D s_2_9: bit-extract s_2_4 s_2_3 s_2_8
        let s_2_9: Bits = (Bits::new(
            ((s_2_4) >> (s_2_3)).value(),
            u16::try_from(s_2_8).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u8
        let s_2_10: u8 = (s_2_9.value() as u8);
        // C s_2_11: const #16975u : u32
        let s_2_11: u32 = 16975;
        // N s_2_12: write-reg s_2_11 <= s_2_10
        let s_2_12: () = {
            state.write_register::<u8>(s_2_11 as isize, s_2_10);
            tracer.write_register(s_2_11 as isize, s_2_10);
        };
        // C s_2_13: const #0s : i
        let s_2_13: i128 = 0;
        // D s_2_14: read-var spsr:bv
        let s_2_14: Bits = fn_state.spsr;
        // C s_2_15: const #1u : u64
        let s_2_15: u64 = 1;
        // D s_2_16: bit-extract s_2_14 s_2_13 s_2_15
        let s_2_16: Bits = (Bits::new(
            ((s_2_14) >> (s_2_13)).value(),
            u16::try_from(s_2_15).unwrap(),
        ));
        // D s_2_17: cast reint s_2_16 -> u8
        let s_2_17: bool = ((s_2_16.value()) != 0);
        // C s_2_18: const #0s : i
        let s_2_18: i128 = 0;
        // C s_2_19: const #0u : u64
        let s_2_19: u64 = 0;
        // D s_2_20: cast zx s_2_17 -> u64
        let s_2_20: u64 = (s_2_17 as u64);
        // C s_2_21: const #1u : u64
        let s_2_21: u64 = 1;
        // D s_2_22: and s_2_20 s_2_21
        let s_2_22: u64 = ((s_2_20) & (s_2_21));
        // D s_2_23: cmp-eq s_2_22 s_2_21
        let s_2_23: bool = ((s_2_22) == (s_2_21));
        // D s_2_24: lsl s_2_20 s_2_18
        let s_2_24: u64 = s_2_20 << s_2_18;
        // D s_2_25: or s_2_19 s_2_24
        let s_2_25: u64 = ((s_2_19) | (s_2_24));
        // D s_2_26: cmpl s_2_24
        let s_2_26: u64 = !s_2_24;
        // D s_2_27: and s_2_19 s_2_26
        let s_2_27: u64 = ((s_2_19) & (s_2_26));
        // D s_2_28: select s_2_23 s_2_25 s_2_27
        let s_2_28: u64 = if s_2_23 { s_2_25 } else { s_2_27 };
        // D s_2_29: cast trunc s_2_28 -> u8
        let s_2_29: bool = ((s_2_28) != 0);
        // C s_2_30: const #16990u : u32
        let s_2_30: u32 = 16990;
        // N s_2_31: write-reg s_2_30 <= s_2_29
        let s_2_31: () = {
            state.write_register::<bool>(s_2_30 as isize, s_2_29);
            tracer.write_register(s_2_30 as isize, s_2_29);
        };
        // C s_2_32: const #() : ()
        let s_2_32: () = ();
        // S s_2_33: call HaveBTIExt(s_2_32)
        let s_2_33: bool = HaveBTIExt(state, tracer, s_2_32);
        // N s_2_34: branch s_2_33 b47 b3
        if s_2_33 {
            return block_47(state, tracer, fn_state);
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
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call HaveSSBSExt(s_4_0)
        let s_4_1: bool = HaveSSBSExt(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b46 b5
        if s_4_1 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaveUAOExt(s_6_0)
        let s_6_1: bool = HaveUAOExt(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b45 b7
        if s_6_1 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HaveDITExt(s_8_0)
        let s_8_1: bool = HaveDITExt(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b44 b9
        if s_8_1 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call HaveMTEExt(s_10_0)
        let s_10_1: bool = HaveMTEExt(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b43 b11
        if s_10_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call HaveGCS(s_12_0)
        let s_12_1: bool = HaveGCS(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b42 b13
        if s_12_1 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16980u : u32
        let s_15_0: u32 = 16980;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: bool = {
            let value = state.read_register::<bool>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 1u16);
        // C s_15_3: const #1u : u8
        let s_15_3: bool = true;
        // C s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 1u16);
        // D s_15_5: cmp-eq s_15_2 s_15_4
        let s_15_5: bool = ((s_15_2) == (s_15_4));
        // N s_15_6: branch s_15_5 b41 b16
        if s_15_5 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#5530 <= s_16_0
        fn_state.gs_5530 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#5530:u8
        let s_17_0: bool = fn_state.gs_5530;
        // N s_17_1: branch s_17_0 b37 b18
        if s_17_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #28s : i
        let s_19_0: i128 = 28;
        // D s_19_1: read-var spsr:bv
        let s_19_1: Bits = fn_state.spsr;
        // C s_19_2: const #1s : i64
        let s_19_2: i64 = 1;
        // C s_19_3: cast zx s_19_2 -> i
        let s_19_3: i128 = (i128::try_from(s_19_2).unwrap());
        // C s_19_4: const #3s : i
        let s_19_4: i128 = 3;
        // C s_19_5: add s_19_4 s_19_3
        let s_19_5: i128 = (s_19_4 + s_19_3);
        // D s_19_6: bit-extract s_19_1 s_19_0 s_19_5
        let s_19_6: Bits = (Bits::new(
            ((s_19_1) >> (s_19_0)).value(),
            u16::try_from(s_19_5).unwrap(),
        ));
        // D s_19_7: cast reint s_19_6 -> u8
        let s_19_7: u8 = (s_19_6.value() as u8);
        // C s_19_8: const #3s : i
        let s_19_8: i128 = 3;
        // D s_19_9: cast zx s_19_7 -> bv
        let s_19_9: Bits = Bits::new(s_19_7 as u128, 4u16);
        // C s_19_10: const #1s : i64
        let s_19_10: i64 = 1;
        // C s_19_11: cast zx s_19_10 -> i
        let s_19_11: i128 = (i128::try_from(s_19_10).unwrap());
        // C s_19_12: const #0s : i
        let s_19_12: i128 = 0;
        // C s_19_13: add s_19_12 s_19_11
        let s_19_13: i128 = (s_19_12 + s_19_11);
        // D s_19_14: bit-extract s_19_9 s_19_8 s_19_13
        let s_19_14: Bits = (Bits::new(
            ((s_19_9) >> (s_19_8)).value(),
            u16::try_from(s_19_13).unwrap(),
        ));
        // D s_19_15: cast reint s_19_14 -> u8
        let s_19_15: bool = ((s_19_14.value()) != 0);
        // C s_19_16: const #16984u : u32
        let s_19_16: u32 = 16984;
        // N s_19_17: write-reg s_19_16 <= s_19_15
        let s_19_17: () = {
            state.write_register::<bool>(s_19_16 as isize, s_19_15);
            tracer.write_register(s_19_16 as isize, s_19_15);
        };
        // C s_19_18: const #2s : i
        let s_19_18: i128 = 2;
        // D s_19_19: cast zx s_19_7 -> bv
        let s_19_19: Bits = Bits::new(s_19_7 as u128, 4u16);
        // C s_19_20: const #1s : i64
        let s_19_20: i64 = 1;
        // C s_19_21: cast zx s_19_20 -> i
        let s_19_21: i128 = (i128::try_from(s_19_20).unwrap());
        // C s_19_22: const #0s : i
        let s_19_22: i128 = 0;
        // C s_19_23: add s_19_22 s_19_21
        let s_19_23: i128 = (s_19_22 + s_19_21);
        // D s_19_24: bit-extract s_19_19 s_19_18 s_19_23
        let s_19_24: Bits = (Bits::new(
            ((s_19_19) >> (s_19_18)).value(),
            u16::try_from(s_19_23).unwrap(),
        ));
        // D s_19_25: cast reint s_19_24 -> u8
        let s_19_25: bool = ((s_19_24.value()) != 0);
        // C s_19_26: const #16997u : u32
        let s_19_26: u32 = 16997;
        // N s_19_27: write-reg s_19_26 <= s_19_25
        let s_19_27: () = {
            state.write_register::<bool>(s_19_26 as isize, s_19_25);
            tracer.write_register(s_19_26 as isize, s_19_25);
        };
        // C s_19_28: const #1s : i
        let s_19_28: i128 = 1;
        // D s_19_29: cast zx s_19_7 -> bv
        let s_19_29: Bits = Bits::new(s_19_7 as u128, 4u16);
        // C s_19_30: const #1s : i64
        let s_19_30: i64 = 1;
        // C s_19_31: cast zx s_19_30 -> i
        let s_19_31: i128 = (i128::try_from(s_19_30).unwrap());
        // C s_19_32: const #0s : i
        let s_19_32: i128 = 0;
        // C s_19_33: add s_19_32 s_19_31
        let s_19_33: i128 = (s_19_32 + s_19_31);
        // D s_19_34: bit-extract s_19_29 s_19_28 s_19_33
        let s_19_34: Bits = (Bits::new(
            ((s_19_29) >> (s_19_28)).value(),
            u16::try_from(s_19_33).unwrap(),
        ));
        // D s_19_35: cast reint s_19_34 -> u8
        let s_19_35: bool = ((s_19_34.value()) != 0);
        // C s_19_36: const #16971u : u32
        let s_19_36: u32 = 16971;
        // N s_19_37: write-reg s_19_36 <= s_19_35
        let s_19_37: () = {
            state.write_register::<bool>(s_19_36 as isize, s_19_35);
            tracer.write_register(s_19_36 as isize, s_19_35);
        };
        // C s_19_38: const #0s : i
        let s_19_38: i128 = 0;
        // D s_19_39: cast zx s_19_7 -> bv
        let s_19_39: Bits = Bits::new(s_19_7 as u128, 4u16);
        // C s_19_40: const #1s : i64
        let s_19_40: i64 = 1;
        // C s_19_41: cast zx s_19_40 -> i
        let s_19_41: i128 = (i128::try_from(s_19_40).unwrap());
        // C s_19_42: const #0s : i
        let s_19_42: i128 = 0;
        // C s_19_43: add s_19_42 s_19_41
        let s_19_43: i128 = (s_19_42 + s_19_41);
        // D s_19_44: bit-extract s_19_39 s_19_38 s_19_43
        let s_19_44: Bits = (Bits::new(
            ((s_19_39) >> (s_19_38)).value(),
            u16::try_from(s_19_43).unwrap(),
        ));
        // D s_19_45: cast reint s_19_44 -> u8
        let s_19_45: bool = ((s_19_44.value()) != 0);
        // C s_19_46: const #16996u : u32
        let s_19_46: u32 = 16996;
        // N s_19_47: write-reg s_19_46 <= s_19_45
        let s_19_47: () = {
            state.write_register::<bool>(s_19_46 as isize, s_19_45);
            tracer.write_register(s_19_46 as isize, s_19_45);
        };
        // C s_19_48: const #() : ()
        let s_19_48: () = ();
        // S s_19_49: call HavePANExt(s_19_48)
        let s_19_49: bool = HavePANExt(state, tracer, s_19_48);
        // N s_19_50: branch s_19_49 b36 b20
        if s_19_49 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #16999u : u32
        let s_21_0: u32 = 16999;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: bool = {
            let value = state.read_register::<bool>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 1u16);
        // C s_21_3: const #1u : u8
        let s_21_3: bool = true;
        // C s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 1u16);
        // D s_21_5: cmp-eq s_21_2 s_21_4
        let s_21_5: bool = ((s_21_2) == (s_21_4));
        // N s_21_6: branch s_21_5 b26 b22
        if s_21_5 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call HaveFeatNMI(s_22_0)
        let s_22_1: bool = HaveFeatNMI(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b25 b23
        if s_22_1 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #6s : i
        let s_24_0: i128 = 6;
        // D s_24_1: read-var spsr:bv
        let s_24_1: Bits = fn_state.spsr;
        // C s_24_2: const #1s : i64
        let s_24_2: i64 = 1;
        // C s_24_3: cast zx s_24_2 -> i
        let s_24_3: i128 = (i128::try_from(s_24_2).unwrap());
        // C s_24_4: const #3s : i
        let s_24_4: i128 = 3;
        // C s_24_5: add s_24_4 s_24_3
        let s_24_5: i128 = (s_24_4 + s_24_3);
        // D s_24_6: bit-extract s_24_1 s_24_0 s_24_5
        let s_24_6: Bits = (Bits::new(
            ((s_24_1) >> (s_24_0)).value(),
            u16::try_from(s_24_5).unwrap(),
        ));
        // D s_24_7: cast reint s_24_6 -> u8
        let s_24_7: u8 = (s_24_6.value() as u8);
        // C s_24_8: const #3s : i
        let s_24_8: i128 = 3;
        // D s_24_9: cast zx s_24_7 -> bv
        let s_24_9: Bits = Bits::new(s_24_7 as u128, 4u16);
        // C s_24_10: const #1s : i64
        let s_24_10: i64 = 1;
        // C s_24_11: cast zx s_24_10 -> i
        let s_24_11: i128 = (i128::try_from(s_24_10).unwrap());
        // C s_24_12: const #0s : i
        let s_24_12: i128 = 0;
        // C s_24_13: add s_24_12 s_24_11
        let s_24_13: i128 = (s_24_12 + s_24_11);
        // D s_24_14: bit-extract s_24_9 s_24_8 s_24_13
        let s_24_14: Bits = (Bits::new(
            ((s_24_9) >> (s_24_8)).value(),
            u16::try_from(s_24_13).unwrap(),
        ));
        // D s_24_15: cast reint s_24_14 -> u8
        let s_24_15: bool = ((s_24_14.value()) != 0);
        // C s_24_16: const #16972u : u32
        let s_24_16: u32 = 16972;
        // N s_24_17: write-reg s_24_16 <= s_24_15
        let s_24_17: () = {
            state.write_register::<bool>(s_24_16 as isize, s_24_15);
            tracer.write_register(s_24_16 as isize, s_24_15);
        };
        // C s_24_18: const #2s : i
        let s_24_18: i128 = 2;
        // D s_24_19: cast zx s_24_7 -> bv
        let s_24_19: Bits = Bits::new(s_24_7 as u128, 4u16);
        // C s_24_20: const #1s : i64
        let s_24_20: i64 = 1;
        // C s_24_21: cast zx s_24_20 -> i
        let s_24_21: i128 = (i128::try_from(s_24_20).unwrap());
        // C s_24_22: const #0s : i
        let s_24_22: i128 = 0;
        // C s_24_23: add s_24_22 s_24_21
        let s_24_23: i128 = (s_24_22 + s_24_21);
        // D s_24_24: bit-extract s_24_19 s_24_18 s_24_23
        let s_24_24: Bits = (Bits::new(
            ((s_24_19) >> (s_24_18)).value(),
            u16::try_from(s_24_23).unwrap(),
        ));
        // D s_24_25: cast reint s_24_24 -> u8
        let s_24_25: bool = ((s_24_24.value()) != 0);
        // C s_24_26: const #16968u : u32
        let s_24_26: u32 = 16968;
        // N s_24_27: write-reg s_24_26 <= s_24_25
        let s_24_27: () = {
            state.write_register::<bool>(s_24_26 as isize, s_24_25);
            tracer.write_register(s_24_26 as isize, s_24_25);
        };
        // C s_24_28: const #1s : i
        let s_24_28: i128 = 1;
        // D s_24_29: cast zx s_24_7 -> bv
        let s_24_29: Bits = Bits::new(s_24_7 as u128, 4u16);
        // C s_24_30: const #1s : i64
        let s_24_30: i64 = 1;
        // C s_24_31: cast zx s_24_30 -> i
        let s_24_31: i128 = (i128::try_from(s_24_30).unwrap());
        // C s_24_32: const #0s : i
        let s_24_32: i128 = 0;
        // C s_24_33: add s_24_32 s_24_31
        let s_24_33: i128 = (s_24_32 + s_24_31);
        // D s_24_34: bit-extract s_24_29 s_24_28 s_24_33
        let s_24_34: Bits = (Bits::new(
            ((s_24_29) >> (s_24_28)).value(),
            u16::try_from(s_24_33).unwrap(),
        ));
        // D s_24_35: cast reint s_24_34 -> u8
        let s_24_35: bool = ((s_24_34.value()) != 0);
        // C s_24_36: const #16979u : u32
        let s_24_36: u32 = 16979;
        // N s_24_37: write-reg s_24_36 <= s_24_35
        let s_24_37: () = {
            state.write_register::<bool>(s_24_36 as isize, s_24_35);
            tracer.write_register(s_24_36 as isize, s_24_35);
        };
        // C s_24_38: const #0s : i
        let s_24_38: i128 = 0;
        // D s_24_39: cast zx s_24_7 -> bv
        let s_24_39: Bits = Bits::new(s_24_7 as u128, 4u16);
        // C s_24_40: const #1s : i64
        let s_24_40: i64 = 1;
        // C s_24_41: cast zx s_24_40 -> i
        let s_24_41: i128 = (i128::try_from(s_24_40).unwrap());
        // C s_24_42: const #0s : i
        let s_24_42: i128 = 0;
        // C s_24_43: add s_24_42 s_24_41
        let s_24_43: i128 = (s_24_42 + s_24_41);
        // D s_24_44: bit-extract s_24_39 s_24_38 s_24_43
        let s_24_44: Bits = (Bits::new(
            ((s_24_39) >> (s_24_38)).value(),
            u16::try_from(s_24_43).unwrap(),
        ));
        // D s_24_45: cast reint s_24_44 -> u8
        let s_24_45: bool = ((s_24_44.value()) != 0);
        // C s_24_46: const #16977u : u32
        let s_24_46: u32 = 16977;
        // N s_24_47: write-reg s_24_46 <= s_24_45
        let s_24_47: () = {
            state.write_register::<bool>(s_24_46 as isize, s_24_45);
            tracer.write_register(s_24_46 as isize, s_24_45);
        };
        // N s_24_48: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #13s : i
        let s_25_0: i128 = 13;
        // D s_25_1: read-var spsr:bv
        let s_25_1: Bits = fn_state.spsr;
        // C s_25_2: const #1u : u64
        let s_25_2: u64 = 1;
        // D s_25_3: bit-extract s_25_1 s_25_0 s_25_2
        let s_25_3: Bits = (Bits::new(
            ((s_25_1) >> (s_25_0)).value(),
            u16::try_from(s_25_2).unwrap(),
        ));
        // D s_25_4: cast reint s_25_3 -> u8
        let s_25_4: bool = ((s_25_3.value()) != 0);
        // C s_25_5: const #0s : i
        let s_25_5: i128 = 0;
        // C s_25_6: const #0u : u64
        let s_25_6: u64 = 0;
        // D s_25_7: cast zx s_25_4 -> u64
        let s_25_7: u64 = (s_25_4 as u64);
        // C s_25_8: const #1u : u64
        let s_25_8: u64 = 1;
        // D s_25_9: and s_25_7 s_25_8
        let s_25_9: u64 = ((s_25_7) & (s_25_8));
        // D s_25_10: cmp-eq s_25_9 s_25_8
        let s_25_10: bool = ((s_25_9) == (s_25_8));
        // D s_25_11: lsl s_25_7 s_25_5
        let s_25_11: u64 = s_25_7 << s_25_5;
        // D s_25_12: or s_25_6 s_25_11
        let s_25_12: u64 = ((s_25_6) | (s_25_11));
        // D s_25_13: cmpl s_25_11
        let s_25_13: u64 = !s_25_11;
        // D s_25_14: and s_25_6 s_25_13
        let s_25_14: u64 = ((s_25_6) & (s_25_13));
        // D s_25_15: select s_25_10 s_25_12 s_25_14
        let s_25_15: u64 = if s_25_10 { s_25_12 } else { s_25_14 };
        // D s_25_16: cast trunc s_25_15 -> u8
        let s_25_16: bool = ((s_25_15) != 0);
        // C s_25_17: const #16969u : u32
        let s_25_17: u32 = 16969;
        // N s_25_18: write-reg s_25_17 <= s_25_16
        let s_25_18: () = {
            state.write_register::<bool>(s_25_17 as isize, s_25_16);
            tracer.write_register(s_25_17 as isize, s_25_16);
        };
        // N s_25_19: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #27s : i
        let s_26_0: i128 = 27;
        // D s_26_1: read-var spsr:bv
        let s_26_1: Bits = fn_state.spsr;
        // C s_26_2: const #1u : u64
        let s_26_2: u64 = 1;
        // D s_26_3: bit-extract s_26_1 s_26_0 s_26_2
        let s_26_3: Bits = (Bits::new(
            ((s_26_1) >> (s_26_0)).value(),
            u16::try_from(s_26_2).unwrap(),
        ));
        // D s_26_4: cast reint s_26_3 -> u8
        let s_26_4: bool = ((s_26_3.value()) != 0);
        // C s_26_5: const #0s : i
        let s_26_5: i128 = 0;
        // C s_26_6: const #0u : u64
        let s_26_6: u64 = 0;
        // D s_26_7: cast zx s_26_4 -> u64
        let s_26_7: u64 = (s_26_4 as u64);
        // C s_26_8: const #1u : u64
        let s_26_8: u64 = 1;
        // D s_26_9: and s_26_7 s_26_8
        let s_26_9: u64 = ((s_26_7) & (s_26_8));
        // D s_26_10: cmp-eq s_26_9 s_26_8
        let s_26_10: bool = ((s_26_9) == (s_26_8));
        // D s_26_11: lsl s_26_7 s_26_5
        let s_26_11: u64 = s_26_7 << s_26_5;
        // D s_26_12: or s_26_6 s_26_11
        let s_26_12: u64 = ((s_26_6) | (s_26_11));
        // D s_26_13: cmpl s_26_11
        let s_26_13: u64 = !s_26_11;
        // D s_26_14: and s_26_6 s_26_13
        let s_26_14: u64 = ((s_26_6) & (s_26_13));
        // D s_26_15: select s_26_10 s_26_12 s_26_14
        let s_26_15: u64 = if s_26_10 { s_26_12 } else { s_26_14 };
        // D s_26_16: cast trunc s_26_15 -> u8
        let s_26_16: bool = ((s_26_15) != 0);
        // C s_26_17: const #16988u : u32
        let s_26_17: u32 = 16988;
        // N s_26_18: write-reg s_26_17 <= s_26_16
        let s_26_18: () = {
            state.write_register::<bool>(s_26_17 as isize, s_26_16);
            tracer.write_register(s_26_17 as isize, s_26_16);
        };
        // D s_26_19: read-var spsr:bv
        let s_26_19: Bits = fn_state.spsr;
        // D s_26_20: call RestoredITBits(s_26_19)
        let s_26_20: u8 = RestoredITBits(state, tracer, s_26_19);
        // C s_26_21: const #16981u : u32
        let s_26_21: u32 = 16981;
        // N s_26_22: write-reg s_26_21 <= s_26_20
        let s_26_22: () = {
            state.write_register::<u8>(s_26_21 as isize, s_26_20);
            tracer.write_register(s_26_21 as isize, s_26_20);
        };
        // C s_26_23: const #0u : u8
        let s_26_23: bool = false;
        // C s_26_24: const #104792u : u32
        let s_26_24: u32 = 104792;
        // N s_26_25: write-reg s_26_24 <= s_26_23
        let s_26_25: () = {
            state.write_register::<bool>(s_26_24 as isize, s_26_23);
            tracer.write_register(s_26_24 as isize, s_26_23);
        };
        // C s_26_26: const #() : ()
        let s_26_26: () = ();
        // S s_26_27: call HaveDITExt(s_26_26)
        let s_26_27: bool = HaveDITExt(state, tracer, s_26_26);
        // N s_26_28: branch s_26_27 b29 b27
        if s_26_27 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #16s : i
        let s_28_0: i128 = 16;
        // D s_28_1: read-var spsr:bv
        let s_28_1: Bits = fn_state.spsr;
        // C s_28_2: const #1s : i64
        let s_28_2: i64 = 1;
        // C s_28_3: cast zx s_28_2 -> i
        let s_28_3: i128 = (i128::try_from(s_28_2).unwrap());
        // C s_28_4: const #3s : i
        let s_28_4: i128 = 3;
        // C s_28_5: add s_28_4 s_28_3
        let s_28_5: i128 = (s_28_4 + s_28_3);
        // D s_28_6: bit-extract s_28_1 s_28_0 s_28_5
        let s_28_6: Bits = (Bits::new(
            ((s_28_1) >> (s_28_0)).value(),
            u16::try_from(s_28_5).unwrap(),
        ));
        // D s_28_7: cast reint s_28_6 -> u8
        let s_28_7: u8 = (s_28_6.value() as u8);
        // C s_28_8: const #16978u : u32
        let s_28_8: u32 = 16978;
        // N s_28_9: write-reg s_28_8 <= s_28_7
        let s_28_9: () = {
            state.write_register::<u8>(s_28_8 as isize, s_28_7);
            tracer.write_register(s_28_8 as isize, s_28_7);
        };
        // C s_28_10: const #9s : i
        let s_28_10: i128 = 9;
        // D s_28_11: read-var spsr:bv
        let s_28_11: Bits = fn_state.spsr;
        // C s_28_12: const #1u : u64
        let s_28_12: u64 = 1;
        // D s_28_13: bit-extract s_28_11 s_28_10 s_28_12
        let s_28_13: Bits = (Bits::new(
            ((s_28_11) >> (s_28_10)).value(),
            u16::try_from(s_28_12).unwrap(),
        ));
        // D s_28_14: cast reint s_28_13 -> u8
        let s_28_14: bool = ((s_28_13.value()) != 0);
        // C s_28_15: const #0s : i
        let s_28_15: i128 = 0;
        // C s_28_16: const #0u : u64
        let s_28_16: u64 = 0;
        // D s_28_17: cast zx s_28_14 -> u64
        let s_28_17: u64 = (s_28_14 as u64);
        // C s_28_18: const #1u : u64
        let s_28_18: u64 = 1;
        // D s_28_19: and s_28_17 s_28_18
        let s_28_19: u64 = ((s_28_17) & (s_28_18));
        // D s_28_20: cmp-eq s_28_19 s_28_18
        let s_28_20: bool = ((s_28_19) == (s_28_18));
        // D s_28_21: lsl s_28_17 s_28_15
        let s_28_21: u64 = s_28_17 << s_28_15;
        // D s_28_22: or s_28_16 s_28_21
        let s_28_22: u64 = ((s_28_16) | (s_28_21));
        // D s_28_23: cmpl s_28_21
        let s_28_23: u64 = !s_28_21;
        // D s_28_24: and s_28_16 s_28_23
        let s_28_24: u64 = ((s_28_16) & (s_28_23));
        // D s_28_25: select s_28_20 s_28_22 s_28_24
        let s_28_25: u64 = if s_28_20 { s_28_22 } else { s_28_24 };
        // D s_28_26: cast trunc s_28_25 -> u8
        let s_28_26: bool = ((s_28_25) != 0);
        // C s_28_27: const #16974u : u32
        let s_28_27: u32 = 16974;
        // N s_28_28: write-reg s_28_27 <= s_28_26
        let s_28_28: () = {
            state.write_register::<bool>(s_28_27 as isize, s_28_26);
            tracer.write_register(s_28_27 as isize, s_28_26);
        };
        // C s_28_29: const #6s : i
        let s_28_29: i128 = 6;
        // D s_28_30: read-var spsr:bv
        let s_28_30: Bits = fn_state.spsr;
        // C s_28_31: const #1s : i64
        let s_28_31: i64 = 1;
        // C s_28_32: cast zx s_28_31 -> i
        let s_28_32: i128 = (i128::try_from(s_28_31).unwrap());
        // C s_28_33: const #2s : i
        let s_28_33: i128 = 2;
        // C s_28_34: add s_28_33 s_28_32
        let s_28_34: i128 = (s_28_33 + s_28_32);
        // D s_28_35: bit-extract s_28_30 s_28_29 s_28_34
        let s_28_35: Bits = (Bits::new(
            ((s_28_30) >> (s_28_29)).value(),
            u16::try_from(s_28_34).unwrap(),
        ));
        // D s_28_36: cast reint s_28_35 -> u8
        let s_28_36: u8 = (s_28_35.value() as u8);
        // C s_28_37: const #2s : i
        let s_28_37: i128 = 2;
        // D s_28_38: cast zx s_28_36 -> bv
        let s_28_38: Bits = Bits::new(s_28_36 as u128, 3u16);
        // C s_28_39: const #1s : i64
        let s_28_39: i64 = 1;
        // C s_28_40: cast zx s_28_39 -> i
        let s_28_40: i128 = (i128::try_from(s_28_39).unwrap());
        // C s_28_41: const #0s : i
        let s_28_41: i128 = 0;
        // C s_28_42: add s_28_41 s_28_40
        let s_28_42: i128 = (s_28_41 + s_28_40);
        // D s_28_43: bit-extract s_28_38 s_28_37 s_28_42
        let s_28_43: Bits = (Bits::new(
            ((s_28_38) >> (s_28_37)).value(),
            u16::try_from(s_28_42).unwrap(),
        ));
        // D s_28_44: cast reint s_28_43 -> u8
        let s_28_44: bool = ((s_28_43.value()) != 0);
        // C s_28_45: const #16968u : u32
        let s_28_45: u32 = 16968;
        // N s_28_46: write-reg s_28_45 <= s_28_44
        let s_28_46: () = {
            state.write_register::<bool>(s_28_45 as isize, s_28_44);
            tracer.write_register(s_28_45 as isize, s_28_44);
        };
        // C s_28_47: const #1s : i
        let s_28_47: i128 = 1;
        // D s_28_48: cast zx s_28_36 -> bv
        let s_28_48: Bits = Bits::new(s_28_36 as u128, 3u16);
        // C s_28_49: const #1s : i64
        let s_28_49: i64 = 1;
        // C s_28_50: cast zx s_28_49 -> i
        let s_28_50: i128 = (i128::try_from(s_28_49).unwrap());
        // C s_28_51: const #0s : i
        let s_28_51: i128 = 0;
        // C s_28_52: add s_28_51 s_28_50
        let s_28_52: i128 = (s_28_51 + s_28_50);
        // D s_28_53: bit-extract s_28_48 s_28_47 s_28_52
        let s_28_53: Bits = (Bits::new(
            ((s_28_48) >> (s_28_47)).value(),
            u16::try_from(s_28_52).unwrap(),
        ));
        // D s_28_54: cast reint s_28_53 -> u8
        let s_28_54: bool = ((s_28_53.value()) != 0);
        // C s_28_55: const #16979u : u32
        let s_28_55: u32 = 16979;
        // N s_28_56: write-reg s_28_55 <= s_28_54
        let s_28_56: () = {
            state.write_register::<bool>(s_28_55 as isize, s_28_54);
            tracer.write_register(s_28_55 as isize, s_28_54);
        };
        // C s_28_57: const #0s : i
        let s_28_57: i128 = 0;
        // D s_28_58: cast zx s_28_36 -> bv
        let s_28_58: Bits = Bits::new(s_28_36 as u128, 3u16);
        // C s_28_59: const #1s : i64
        let s_28_59: i64 = 1;
        // C s_28_60: cast zx s_28_59 -> i
        let s_28_60: i128 = (i128::try_from(s_28_59).unwrap());
        // C s_28_61: const #0s : i
        let s_28_61: i128 = 0;
        // C s_28_62: add s_28_61 s_28_60
        let s_28_62: i128 = (s_28_61 + s_28_60);
        // D s_28_63: bit-extract s_28_58 s_28_57 s_28_62
        let s_28_63: Bits = (Bits::new(
            ((s_28_58) >> (s_28_57)).value(),
            u16::try_from(s_28_62).unwrap(),
        ));
        // D s_28_64: cast reint s_28_63 -> u8
        let s_28_64: bool = ((s_28_63.value()) != 0);
        // C s_28_65: const #16977u : u32
        let s_28_65: u32 = 16977;
        // N s_28_66: write-reg s_28_65 <= s_28_64
        let s_28_66: () = {
            state.write_register::<bool>(s_28_65 as isize, s_28_64);
            tracer.write_register(s_28_65 as isize, s_28_64);
        };
        // C s_28_67: const #5s : i
        let s_28_67: i128 = 5;
        // D s_28_68: read-var spsr:bv
        let s_28_68: Bits = fn_state.spsr;
        // C s_28_69: const #1u : u64
        let s_28_69: u64 = 1;
        // D s_28_70: bit-extract s_28_68 s_28_67 s_28_69
        let s_28_70: Bits = (Bits::new(
            ((s_28_68) >> (s_28_67)).value(),
            u16::try_from(s_28_69).unwrap(),
        ));
        // D s_28_71: cast reint s_28_70 -> u8
        let s_28_71: bool = ((s_28_70.value()) != 0);
        // C s_28_72: const #0s : i
        let s_28_72: i128 = 0;
        // C s_28_73: const #0u : u64
        let s_28_73: u64 = 0;
        // D s_28_74: cast zx s_28_71 -> u64
        let s_28_74: u64 = (s_28_71 as u64);
        // C s_28_75: const #1u : u64
        let s_28_75: u64 = 1;
        // D s_28_76: and s_28_74 s_28_75
        let s_28_76: u64 = ((s_28_74) & (s_28_75));
        // D s_28_77: cmp-eq s_28_76 s_28_75
        let s_28_77: bool = ((s_28_76) == (s_28_75));
        // D s_28_78: lsl s_28_74 s_28_72
        let s_28_78: u64 = s_28_74 << s_28_72;
        // D s_28_79: or s_28_73 s_28_78
        let s_28_79: u64 = ((s_28_73) | (s_28_78));
        // D s_28_80: cmpl s_28_78
        let s_28_80: u64 = !s_28_78;
        // D s_28_81: and s_28_73 s_28_80
        let s_28_81: u64 = ((s_28_73) & (s_28_80));
        // D s_28_82: select s_28_77 s_28_79 s_28_81
        let s_28_82: u64 = if s_28_77 { s_28_79 } else { s_28_81 };
        // D s_28_83: cast trunc s_28_82 -> u8
        let s_28_83: bool = ((s_28_82) != 0);
        // C s_28_84: const #16993u : u32
        let s_28_84: u32 = 16993;
        // N s_28_85: write-reg s_28_84 <= s_28_83
        let s_28_85: () = {
            state.write_register::<bool>(s_28_84 as isize, s_28_83);
            tracer.write_register(s_28_84 as isize, s_28_83);
        };
        // N s_28_86: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call Restarting(s_29_0)
        let s_29_1: bool = Restarting(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b35 b30
        if s_29_1 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var from_aarch64:u8
        let s_30_0: bool = fn_state.from_aarch64;
        // D s_30_1: write-var gs#5581 <= s_30_0
        fn_state.gs_5581 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#5581:u8
        let s_31_0: bool = fn_state.gs_5581;
        // N s_31_1: branch s_31_0 b34 b32
        if s_31_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #21s : i
        let s_32_0: i128 = 21;
        // D s_32_1: read-var spsr:bv
        let s_32_1: Bits = fn_state.spsr;
        // C s_32_2: const #1u : u64
        let s_32_2: u64 = 1;
        // D s_32_3: bit-extract s_32_1 s_32_0 s_32_2
        let s_32_3: Bits = (Bits::new(
            ((s_32_1) >> (s_32_0)).value(),
            u16::try_from(s_32_2).unwrap(),
        ));
        // D s_32_4: cast reint s_32_3 -> u8
        let s_32_4: bool = ((s_32_3.value()) != 0);
        // C s_32_5: const #0s : i
        let s_32_5: i128 = 0;
        // C s_32_6: const #0u : u64
        let s_32_6: u64 = 0;
        // D s_32_7: cast zx s_32_4 -> u64
        let s_32_7: u64 = (s_32_4 as u64);
        // C s_32_8: const #1u : u64
        let s_32_8: u64 = 1;
        // D s_32_9: and s_32_7 s_32_8
        let s_32_9: u64 = ((s_32_7) & (s_32_8));
        // D s_32_10: cmp-eq s_32_9 s_32_8
        let s_32_10: bool = ((s_32_9) == (s_32_8));
        // D s_32_11: lsl s_32_7 s_32_5
        let s_32_11: u64 = s_32_7 << s_32_5;
        // D s_32_12: or s_32_6 s_32_11
        let s_32_12: u64 = ((s_32_6) | (s_32_11));
        // D s_32_13: cmpl s_32_11
        let s_32_13: u64 = !s_32_11;
        // D s_32_14: and s_32_6 s_32_13
        let s_32_14: u64 = ((s_32_6) & (s_32_13));
        // D s_32_15: select s_32_10 s_32_12 s_32_14
        let s_32_15: u64 = if s_32_10 { s_32_12 } else { s_32_14 };
        // D s_32_16: cast trunc s_32_15 -> u8
        let s_32_16: bool = ((s_32_15) != 0);
        // C s_32_17: const #16973u : u32
        let s_32_17: u32 = 16973;
        // N s_32_18: write-reg s_32_17 <= s_32_16
        let s_32_18: () = {
            state.write_register::<bool>(s_32_17 as isize, s_32_16);
            tracer.write_register(s_32_17 as isize, s_32_16);
        };
        // N s_32_19: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #24s : i
        let s_34_0: i128 = 24;
        // D s_34_1: read-var spsr:bv
        let s_34_1: Bits = fn_state.spsr;
        // C s_34_2: const #1u : u64
        let s_34_2: u64 = 1;
        // D s_34_3: bit-extract s_34_1 s_34_0 s_34_2
        let s_34_3: Bits = (Bits::new(
            ((s_34_1) >> (s_34_0)).value(),
            u16::try_from(s_34_2).unwrap(),
        ));
        // D s_34_4: cast reint s_34_3 -> u8
        let s_34_4: bool = ((s_34_3.value()) != 0);
        // C s_34_5: const #0s : i
        let s_34_5: i128 = 0;
        // C s_34_6: const #0u : u64
        let s_34_6: u64 = 0;
        // D s_34_7: cast zx s_34_4 -> u64
        let s_34_7: u64 = (s_34_4 as u64);
        // C s_34_8: const #1u : u64
        let s_34_8: u64 = 1;
        // D s_34_9: and s_34_7 s_34_8
        let s_34_9: u64 = ((s_34_7) & (s_34_8));
        // D s_34_10: cmp-eq s_34_9 s_34_8
        let s_34_10: bool = ((s_34_9) == (s_34_8));
        // D s_34_11: lsl s_34_7 s_34_5
        let s_34_11: u64 = s_34_7 << s_34_5;
        // D s_34_12: or s_34_6 s_34_11
        let s_34_12: u64 = ((s_34_6) | (s_34_11));
        // D s_34_13: cmpl s_34_11
        let s_34_13: u64 = !s_34_11;
        // D s_34_14: and s_34_6 s_34_13
        let s_34_14: u64 = ((s_34_6) & (s_34_13));
        // D s_34_15: select s_34_10 s_34_12 s_34_14
        let s_34_15: u64 = if s_34_10 { s_34_12 } else { s_34_14 };
        // D s_34_16: cast trunc s_34_15 -> u8
        let s_34_16: bool = ((s_34_15) != 0);
        // C s_34_17: const #16973u : u32
        let s_34_17: u32 = 16973;
        // N s_34_18: write-reg s_34_17 <= s_34_16
        let s_34_18: () = {
            state.write_register::<bool>(s_34_17 as isize, s_34_16);
            tracer.write_register(s_34_17 as isize, s_34_16);
        };
        // N s_34_19: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#5581 <= s_35_0
        fn_state.gs_5581 = s_35_0;
        // N s_35_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #22s : i
        let s_36_0: i128 = 22;
        // D s_36_1: read-var spsr:bv
        let s_36_1: Bits = fn_state.spsr;
        // C s_36_2: const #1u : u64
        let s_36_2: u64 = 1;
        // D s_36_3: bit-extract s_36_1 s_36_0 s_36_2
        let s_36_3: Bits = (Bits::new(
            ((s_36_1) >> (s_36_0)).value(),
            u16::try_from(s_36_2).unwrap(),
        ));
        // D s_36_4: cast reint s_36_3 -> u8
        let s_36_4: bool = ((s_36_3.value()) != 0);
        // C s_36_5: const #0s : i
        let s_36_5: i128 = 0;
        // C s_36_6: const #0u : u64
        let s_36_6: u64 = 0;
        // D s_36_7: cast zx s_36_4 -> u64
        let s_36_7: u64 = (s_36_4 as u64);
        // C s_36_8: const #1u : u64
        let s_36_8: u64 = 1;
        // D s_36_9: and s_36_7 s_36_8
        let s_36_9: u64 = ((s_36_7) & (s_36_8));
        // D s_36_10: cmp-eq s_36_9 s_36_8
        let s_36_10: bool = ((s_36_9) == (s_36_8));
        // D s_36_11: lsl s_36_7 s_36_5
        let s_36_11: u64 = s_36_7 << s_36_5;
        // D s_36_12: or s_36_6 s_36_11
        let s_36_12: u64 = ((s_36_6) | (s_36_11));
        // D s_36_13: cmpl s_36_11
        let s_36_13: u64 = !s_36_11;
        // D s_36_14: and s_36_6 s_36_13
        let s_36_14: u64 = ((s_36_6) & (s_36_13));
        // D s_36_15: select s_36_10 s_36_12 s_36_14
        let s_36_15: u64 = if s_36_10 { s_36_12 } else { s_36_14 };
        // D s_36_16: cast trunc s_36_15 -> u8
        let s_36_16: bool = ((s_36_15) != 0);
        // C s_36_17: const #16985u : u32
        let s_36_17: u32 = 16985;
        // N s_36_18: write-reg s_36_17 <= s_36_16
        let s_36_18: () = {
            state.write_register::<bool>(s_36_17 as isize, s_36_16);
            tracer.write_register(s_36_17 as isize, s_36_16);
        };
        // N s_36_19: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #23u : u32
        let s_37_0: u32 = 23;
        // S s_37_1: call ConstrainUnpredictableBool(s_37_0)
        let s_37_1: bool = ConstrainUnpredictableBool(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b40 b38
        if s_37_1 {
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
        // N s_39_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var spsr_in:bv
        let s_40_0: Bits = fn_state.spsr_in;
        // D s_40_1: size-of s_40_0
        let s_40_1: u16 = s_40_0.length();
        // D s_40_2: cast zx s_40_1 -> i
        let s_40_2: i128 = (i128::try_from(s_40_1).unwrap());
        // C s_40_3: const #5s : i
        let s_40_3: i128 = 5;
        // D s_40_4: cmp-lt s_40_3 s_40_2
        let s_40_4: bool = ((s_40_3) < (s_40_2));
        // N s_40_5: assert s_40_4
        let s_40_5: () = assert!(s_40_4);
        // C s_40_6: const #0u : u8
        let s_40_6: bool = false;
        // S s_40_7: call Bit(s_40_6)
        let s_40_7: bool = Bit(state, tracer, s_40_6);
        // C s_40_8: const #5s : i
        let s_40_8: i128 = 5;
        // D s_40_9: read-var spsr:bv
        let s_40_9: Bits = fn_state.spsr;
        // C s_40_10: const #1u : u64
        let s_40_10: u64 = 1;
        // D s_40_11: bit-insert s_40_9 s_40_9 s_40_8 s_40_10
        let s_40_11: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_40_10 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_40_9.length(),
            );
            (s_40_9 & mask) | (s_40_9 << s_40_8)
        };
        // D s_40_12: write-var spsr <= s_40_11
        fn_state.spsr = s_40_11;
        // N s_40_13: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #16999u : u32
        let s_41_0: u32 = 16999;
        // D s_41_1: read-reg s_41_0:u8
        let s_41_1: bool = {
            let value = state.read_register::<bool>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: cast zx s_41_1 -> bv
        let s_41_2: Bits = Bits::new(s_41_1 as u128, 1u16);
        // C s_41_3: const #1u : u8
        let s_41_3: bool = true;
        // C s_41_4: cast zx s_41_3 -> bv
        let s_41_4: Bits = Bits::new(s_41_3 as u128, 1u16);
        // D s_41_5: cmp-eq s_41_2 s_41_4
        let s_41_5: bool = ((s_41_2) == (s_41_4));
        // D s_41_6: write-var gs#5530 <= s_41_5
        fn_state.gs_5530 = s_41_5;
        // N s_41_7: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var spsr_in:bv
        let s_42_0: Bits = fn_state.spsr_in;
        // D s_42_1: size-of s_42_0
        let s_42_1: u16 = s_42_0.length();
        // D s_42_2: cast zx s_42_1 -> i
        let s_42_2: i128 = (i128::try_from(s_42_1).unwrap());
        // C s_42_3: const #34s : i
        let s_42_3: i128 = 34;
        // D s_42_4: cmp-lt s_42_3 s_42_2
        let s_42_4: bool = ((s_42_3) < (s_42_2));
        // N s_42_5: assert s_42_4
        let s_42_5: () = assert!(s_42_4);
        // C s_42_6: const #34s : i
        let s_42_6: i128 = 34;
        // D s_42_7: read-var spsr:bv
        let s_42_7: Bits = fn_state.spsr;
        // C s_42_8: const #1u : u64
        let s_42_8: u64 = 1;
        // D s_42_9: bit-extract s_42_7 s_42_6 s_42_8
        let s_42_9: Bits = (Bits::new(
            ((s_42_7) >> (s_42_6)).value(),
            u16::try_from(s_42_8).unwrap(),
        ));
        // D s_42_10: cast reint s_42_9 -> u8
        let s_42_10: bool = ((s_42_9.value()) != 0);
        // C s_42_11: const #0s : i
        let s_42_11: i128 = 0;
        // C s_42_12: const #0u : u64
        let s_42_12: u64 = 0;
        // D s_42_13: cast zx s_42_10 -> u64
        let s_42_13: u64 = (s_42_10 as u64);
        // C s_42_14: const #1u : u64
        let s_42_14: u64 = 1;
        // D s_42_15: and s_42_13 s_42_14
        let s_42_15: u64 = ((s_42_13) & (s_42_14));
        // D s_42_16: cmp-eq s_42_15 s_42_14
        let s_42_16: bool = ((s_42_15) == (s_42_14));
        // D s_42_17: lsl s_42_13 s_42_11
        let s_42_17: u64 = s_42_13 << s_42_11;
        // D s_42_18: or s_42_12 s_42_17
        let s_42_18: u64 = ((s_42_12) | (s_42_17));
        // D s_42_19: cmpl s_42_17
        let s_42_19: u64 = !s_42_17;
        // D s_42_20: and s_42_12 s_42_19
        let s_42_20: u64 = ((s_42_12) & (s_42_19));
        // D s_42_21: select s_42_16 s_42_18 s_42_20
        let s_42_21: u64 = if s_42_16 { s_42_18 } else { s_42_20 };
        // D s_42_22: cast trunc s_42_21 -> u8
        let s_42_22: bool = ((s_42_21) != 0);
        // C s_42_23: const #16976u : u32
        let s_42_23: u32 = 16976;
        // N s_42_24: write-reg s_42_23 <= s_42_22
        let s_42_24: () = {
            state.write_register::<bool>(s_42_23 as isize, s_42_22);
            tracer.write_register(s_42_23 as isize, s_42_22);
        };
        // N s_42_25: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var spsr_in:bv
        let s_43_0: Bits = fn_state.spsr_in;
        // D s_43_1: size-of s_43_0
        let s_43_1: u16 = s_43_0.length();
        // D s_43_2: cast zx s_43_1 -> i
        let s_43_2: i128 = (i128::try_from(s_43_1).unwrap());
        // C s_43_3: const #25s : i
        let s_43_3: i128 = 25;
        // D s_43_4: cmp-lt s_43_3 s_43_2
        let s_43_4: bool = ((s_43_3) < (s_43_2));
        // N s_43_5: assert s_43_4
        let s_43_5: () = assert!(s_43_4);
        // C s_43_6: const #25s : i
        let s_43_6: i128 = 25;
        // D s_43_7: read-var spsr:bv
        let s_43_7: Bits = fn_state.spsr;
        // C s_43_8: const #1u : u64
        let s_43_8: u64 = 1;
        // D s_43_9: bit-extract s_43_7 s_43_6 s_43_8
        let s_43_9: Bits = (Bits::new(
            ((s_43_7) >> (s_43_6)).value(),
            u16::try_from(s_43_8).unwrap(),
        ));
        // D s_43_10: cast reint s_43_9 -> u8
        let s_43_10: bool = ((s_43_9.value()) != 0);
        // C s_43_11: const #0s : i
        let s_43_11: i128 = 0;
        // C s_43_12: const #0u : u64
        let s_43_12: u64 = 0;
        // D s_43_13: cast zx s_43_10 -> u64
        let s_43_13: u64 = (s_43_10 as u64);
        // C s_43_14: const #1u : u64
        let s_43_14: u64 = 1;
        // D s_43_15: and s_43_13 s_43_14
        let s_43_15: u64 = ((s_43_13) & (s_43_14));
        // D s_43_16: cmp-eq s_43_15 s_43_14
        let s_43_16: bool = ((s_43_15) == (s_43_14));
        // D s_43_17: lsl s_43_13 s_43_11
        let s_43_17: u64 = s_43_13 << s_43_11;
        // D s_43_18: or s_43_12 s_43_17
        let s_43_18: u64 = ((s_43_12) | (s_43_17));
        // D s_43_19: cmpl s_43_17
        let s_43_19: u64 = !s_43_17;
        // D s_43_20: and s_43_12 s_43_19
        let s_43_20: u64 = ((s_43_12) & (s_43_19));
        // D s_43_21: select s_43_16 s_43_18 s_43_20
        let s_43_21: u64 = if s_43_16 { s_43_18 } else { s_43_20 };
        // D s_43_22: cast trunc s_43_21 -> u8
        let s_43_22: bool = ((s_43_21) != 0);
        // C s_43_23: const #16994u : u32
        let s_43_23: u32 = 16994;
        // N s_43_24: write-reg s_43_23 <= s_43_22
        let s_43_24: () = {
            state.write_register::<bool>(s_43_23 as isize, s_43_22);
            tracer.write_register(s_43_23 as isize, s_43_22);
        };
        // N s_43_25: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var spsr_in:bv
        let s_44_0: Bits = fn_state.spsr_in;
        // D s_44_1: size-of s_44_0
        let s_44_1: u16 = s_44_0.length();
        // D s_44_2: cast zx s_44_1 -> i
        let s_44_2: i128 = (i128::try_from(s_44_1).unwrap());
        // C s_44_3: const #24s : i
        let s_44_3: i128 = 24;
        // D s_44_4: cmp-lt s_44_3 s_44_2
        let s_44_4: bool = ((s_44_3) < (s_44_2));
        // N s_44_5: assert s_44_4
        let s_44_5: () = assert!(s_44_4);
        // C s_44_6: const #24s : i
        let s_44_6: i128 = 24;
        // D s_44_7: read-var spsr:bv
        let s_44_7: Bits = fn_state.spsr;
        // C s_44_8: const #1u : u64
        let s_44_8: u64 = 1;
        // D s_44_9: bit-extract s_44_7 s_44_6 s_44_8
        let s_44_9: Bits = (Bits::new(
            ((s_44_7) >> (s_44_6)).value(),
            u16::try_from(s_44_8).unwrap(),
        ));
        // D s_44_10: cast reint s_44_9 -> u8
        let s_44_10: bool = ((s_44_9.value()) != 0);
        // C s_44_11: const #0s : i
        let s_44_11: i128 = 0;
        // C s_44_12: const #0u : u64
        let s_44_12: u64 = 0;
        // D s_44_13: cast zx s_44_10 -> u64
        let s_44_13: u64 = (s_44_10 as u64);
        // C s_44_14: const #1u : u64
        let s_44_14: u64 = 1;
        // D s_44_15: and s_44_13 s_44_14
        let s_44_15: u64 = ((s_44_13) & (s_44_14));
        // D s_44_16: cmp-eq s_44_15 s_44_14
        let s_44_16: bool = ((s_44_15) == (s_44_14));
        // D s_44_17: lsl s_44_13 s_44_11
        let s_44_17: u64 = s_44_13 << s_44_11;
        // D s_44_18: or s_44_12 s_44_17
        let s_44_18: u64 = ((s_44_12) | (s_44_17));
        // D s_44_19: cmpl s_44_17
        let s_44_19: u64 = !s_44_17;
        // D s_44_20: and s_44_12 s_44_19
        let s_44_20: u64 = ((s_44_12) & (s_44_19));
        // D s_44_21: select s_44_16 s_44_18 s_44_20
        let s_44_21: u64 = if s_44_16 { s_44_18 } else { s_44_20 };
        // D s_44_22: cast trunc s_44_21 -> u8
        let s_44_22: bool = ((s_44_21) != 0);
        // C s_44_23: const #16973u : u32
        let s_44_23: u32 = 16973;
        // N s_44_24: write-reg s_44_23 <= s_44_22
        let s_44_24: () = {
            state.write_register::<bool>(s_44_23 as isize, s_44_22);
            tracer.write_register(s_44_23 as isize, s_44_22);
        };
        // N s_44_25: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var spsr_in:bv
        let s_45_0: Bits = fn_state.spsr_in;
        // D s_45_1: size-of s_45_0
        let s_45_1: u16 = s_45_0.length();
        // D s_45_2: cast zx s_45_1 -> i
        let s_45_2: i128 = (i128::try_from(s_45_1).unwrap());
        // C s_45_3: const #23s : i
        let s_45_3: i128 = 23;
        // D s_45_4: cmp-lt s_45_3 s_45_2
        let s_45_4: bool = ((s_45_3) < (s_45_2));
        // N s_45_5: assert s_45_4
        let s_45_5: () = assert!(s_45_4);
        // C s_45_6: const #23s : i
        let s_45_6: i128 = 23;
        // D s_45_7: read-var spsr:bv
        let s_45_7: Bits = fn_state.spsr;
        // C s_45_8: const #1u : u64
        let s_45_8: u64 = 1;
        // D s_45_9: bit-extract s_45_7 s_45_6 s_45_8
        let s_45_9: Bits = (Bits::new(
            ((s_45_7) >> (s_45_6)).value(),
            u16::try_from(s_45_8).unwrap(),
        ));
        // D s_45_10: cast reint s_45_9 -> u8
        let s_45_10: bool = ((s_45_9.value()) != 0);
        // C s_45_11: const #0s : i
        let s_45_11: i128 = 0;
        // C s_45_12: const #0u : u64
        let s_45_12: u64 = 0;
        // D s_45_13: cast zx s_45_10 -> u64
        let s_45_13: u64 = (s_45_10 as u64);
        // C s_45_14: const #1u : u64
        let s_45_14: u64 = 1;
        // D s_45_15: and s_45_13 s_45_14
        let s_45_15: u64 = ((s_45_13) & (s_45_14));
        // D s_45_16: cmp-eq s_45_15 s_45_14
        let s_45_16: bool = ((s_45_15) == (s_45_14));
        // D s_45_17: lsl s_45_13 s_45_11
        let s_45_17: u64 = s_45_13 << s_45_11;
        // D s_45_18: or s_45_12 s_45_17
        let s_45_18: u64 = ((s_45_12) | (s_45_17));
        // D s_45_19: cmpl s_45_17
        let s_45_19: u64 = !s_45_17;
        // D s_45_20: and s_45_12 s_45_19
        let s_45_20: u64 = ((s_45_12) & (s_45_19));
        // D s_45_21: select s_45_16 s_45_18 s_45_20
        let s_45_21: u64 = if s_45_16 { s_45_18 } else { s_45_20 };
        // D s_45_22: cast trunc s_45_21 -> u8
        let s_45_22: bool = ((s_45_21) != 0);
        // C s_45_23: const #16995u : u32
        let s_45_23: u32 = 16995;
        // N s_45_24: write-reg s_45_23 <= s_45_22
        let s_45_24: () = {
            state.write_register::<bool>(s_45_23 as isize, s_45_22);
            tracer.write_register(s_45_23 as isize, s_45_22);
        };
        // N s_45_25: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #12s : i
        let s_46_0: i128 = 12;
        // D s_46_1: read-var spsr:bv
        let s_46_1: Bits = fn_state.spsr;
        // C s_46_2: const #1u : u64
        let s_46_2: u64 = 1;
        // D s_46_3: bit-extract s_46_1 s_46_0 s_46_2
        let s_46_3: Bits = (Bits::new(
            ((s_46_1) >> (s_46_0)).value(),
            u16::try_from(s_46_2).unwrap(),
        ));
        // D s_46_4: cast reint s_46_3 -> u8
        let s_46_4: bool = ((s_46_3.value()) != 0);
        // C s_46_5: const #0s : i
        let s_46_5: i128 = 0;
        // C s_46_6: const #0u : u64
        let s_46_6: u64 = 0;
        // D s_46_7: cast zx s_46_4 -> u64
        let s_46_7: u64 = (s_46_4 as u64);
        // C s_46_8: const #1u : u64
        let s_46_8: u64 = 1;
        // D s_46_9: and s_46_7 s_46_8
        let s_46_9: u64 = ((s_46_7) & (s_46_8));
        // D s_46_10: cmp-eq s_46_9 s_46_8
        let s_46_10: bool = ((s_46_9) == (s_46_8));
        // D s_46_11: lsl s_46_7 s_46_5
        let s_46_11: u64 = s_46_7 << s_46_5;
        // D s_46_12: or s_46_6 s_46_11
        let s_46_12: u64 = ((s_46_6) | (s_46_11));
        // D s_46_13: cmpl s_46_11
        let s_46_13: u64 = !s_46_11;
        // D s_46_14: and s_46_6 s_46_13
        let s_46_14: u64 = ((s_46_6) & (s_46_13));
        // D s_46_15: select s_46_10 s_46_12 s_46_14
        let s_46_15: u64 = if s_46_10 { s_46_12 } else { s_46_14 };
        // D s_46_16: cast trunc s_46_15 -> u8
        let s_46_16: bool = ((s_46_15) != 0);
        // C s_46_17: const #16992u : u32
        let s_46_17: u32 = 16992;
        // N s_46_18: write-reg s_46_17 <= s_46_16
        let s_46_18: () = {
            state.write_register::<bool>(s_46_17 as isize, s_46_16);
            tracer.write_register(s_46_17 as isize, s_46_16);
        };
        // N s_46_19: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #10s : i
        let s_47_0: i128 = 10;
        // D s_47_1: read-var spsr:bv
        let s_47_1: Bits = fn_state.spsr;
        // C s_47_2: const #1s : i64
        let s_47_2: i64 = 1;
        // C s_47_3: cast zx s_47_2 -> i
        let s_47_3: i128 = (i128::try_from(s_47_2).unwrap());
        // C s_47_4: const #1s : i
        let s_47_4: i128 = 1;
        // C s_47_5: add s_47_4 s_47_3
        let s_47_5: i128 = (s_47_4 + s_47_3);
        // D s_47_6: bit-extract s_47_1 s_47_0 s_47_5
        let s_47_6: Bits = (Bits::new(
            ((s_47_1) >> (s_47_0)).value(),
            u16::try_from(s_47_5).unwrap(),
        ));
        // D s_47_7: cast reint s_47_6 -> u8
        let s_47_7: u8 = (s_47_6.value() as u8);
        // C s_47_8: const #16970u : u32
        let s_47_8: u32 = 16970;
        // N s_47_9: write-reg s_47_8 <= s_47_7
        let s_47_9: () = {
            state.write_register::<u8>(s_47_8 as isize, s_47_7);
            tracer.write_register(s_47_8 as isize, s_47_7);
        };
        // N s_47_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0s : i
        let s_48_0: i128 = 0;
        // D s_48_1: read-var spsr:bv
        let s_48_1: Bits = fn_state.spsr;
        // C s_48_2: const #1s : i64
        let s_48_2: i64 = 1;
        // C s_48_3: cast zx s_48_2 -> i
        let s_48_3: i128 = (i128::try_from(s_48_2).unwrap());
        // C s_48_4: const #4s : i
        let s_48_4: i128 = 4;
        // C s_48_5: add s_48_4 s_48_3
        let s_48_5: i128 = (s_48_4 + s_48_3);
        // D s_48_6: bit-extract s_48_1 s_48_0 s_48_5
        let s_48_6: Bits = (Bits::new(
            ((s_48_1) >> (s_48_0)).value(),
            u16::try_from(s_48_5).unwrap(),
        ));
        // D s_48_7: cast reint s_48_6 -> u8
        let s_48_7: u8 = (s_48_6.value() as u8);
        // D s_48_8: call AArch32_WriteMode(s_48_7)
        let s_48_8: () = AArch32_WriteMode(state, tracer, s_48_7);
        // C s_48_9: const #() : ()
        let s_48_9: () = ();
        // S s_48_10: call HaveSSBSExt(s_48_9)
        let s_48_10: bool = HaveSSBSExt(state, tracer, s_48_9);
        // N s_48_11: branch s_48_10 b51 b49
        if s_48_10 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_49_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var spsr_in:bv
        let s_51_0: Bits = fn_state.spsr_in;
        // D s_51_1: size-of s_51_0
        let s_51_1: u16 = s_51_0.length();
        // D s_51_2: cast zx s_51_1 -> i
        let s_51_2: i128 = (i128::try_from(s_51_1).unwrap());
        // C s_51_3: const #23s : i
        let s_51_3: i128 = 23;
        // D s_51_4: cmp-lt s_51_3 s_51_2
        let s_51_4: bool = ((s_51_3) < (s_51_2));
        // N s_51_5: assert s_51_4
        let s_51_5: () = assert!(s_51_4);
        // C s_51_6: const #23s : i
        let s_51_6: i128 = 23;
        // D s_51_7: read-var spsr:bv
        let s_51_7: Bits = fn_state.spsr;
        // C s_51_8: const #1u : u64
        let s_51_8: u64 = 1;
        // D s_51_9: bit-extract s_51_7 s_51_6 s_51_8
        let s_51_9: Bits = (Bits::new(
            ((s_51_7) >> (s_51_6)).value(),
            u16::try_from(s_51_8).unwrap(),
        ));
        // D s_51_10: cast reint s_51_9 -> u8
        let s_51_10: bool = ((s_51_9.value()) != 0);
        // C s_51_11: const #0s : i
        let s_51_11: i128 = 0;
        // C s_51_12: const #0u : u64
        let s_51_12: u64 = 0;
        // D s_51_13: cast zx s_51_10 -> u64
        let s_51_13: u64 = (s_51_10 as u64);
        // C s_51_14: const #1u : u64
        let s_51_14: u64 = 1;
        // D s_51_15: and s_51_13 s_51_14
        let s_51_15: u64 = ((s_51_13) & (s_51_14));
        // D s_51_16: cmp-eq s_51_15 s_51_14
        let s_51_16: bool = ((s_51_15) == (s_51_14));
        // D s_51_17: lsl s_51_13 s_51_11
        let s_51_17: u64 = s_51_13 << s_51_11;
        // D s_51_18: or s_51_12 s_51_17
        let s_51_18: u64 = ((s_51_12) | (s_51_17));
        // D s_51_19: cmpl s_51_17
        let s_51_19: u64 = !s_51_17;
        // D s_51_20: and s_51_12 s_51_19
        let s_51_20: u64 = ((s_51_12) & (s_51_19));
        // D s_51_21: select s_51_16 s_51_18 s_51_20
        let s_51_21: u64 = if s_51_16 { s_51_18 } else { s_51_20 };
        // D s_51_22: cast trunc s_51_21 -> u8
        let s_51_22: bool = ((s_51_21) != 0);
        // C s_51_23: const #16992u : u32
        let s_51_23: u32 = 16992;
        // N s_51_24: write-reg s_51_23 <= s_51_22
        let s_51_24: () = {
            state.write_register::<bool>(s_51_23 as isize, s_51_22);
            tracer.write_register(s_51_23 as isize, s_51_22);
        };
        // N s_51_25: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // C s_52_1: const #16980u : u32
        let s_52_1: u32 = 16980;
        // N s_52_2: write-reg s_52_1 <= s_52_0
        let s_52_2: () = {
            state.write_register::<bool>(s_52_1 as isize, s_52_0);
            tracer.write_register(s_52_1 as isize, s_52_0);
        };
        // C s_52_3: const #() : ()
        let s_52_3: () = ();
        // S s_52_4: call HaveSSBSExt(s_52_3)
        let s_52_4: bool = HaveSSBSExt(state, tracer, s_52_3);
        // N s_52_5: branch s_52_4 b67 b53
        if s_52_4 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call HaveBTIExt(s_54_0)
        let s_54_1: bool = HaveBTIExt(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b66 b55
        if s_54_1 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call HaveUAOExt(s_56_0)
        let s_56_1: bool = HaveUAOExt(state, tracer, s_56_0);
        // N s_56_2: branch s_56_1 b65 b57
        if s_56_1 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call HaveDITExt(s_58_0)
        let s_58_1: bool = HaveDITExt(state, tracer, s_58_0);
        // N s_58_2: branch s_58_1 b64 b59
        if s_58_1 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call HaveMTEExt(s_60_0)
        let s_60_1: bool = HaveMTEExt(state, tracer, s_60_0);
        // N s_60_2: branch s_60_1 b63 b61
        if s_60_1 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_61_0: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_62_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call __UNKNOWN_bit(s_63_0)
        let s_63_1: bool = u__UNKNOWN_bit(state, tracer, s_63_0);
        // C s_63_2: const #16994u : u32
        let s_63_2: u32 = 16994;
        // N s_63_3: write-reg s_63_2 <= s_63_1
        let s_63_3: () = {
            state.write_register::<bool>(s_63_2 as isize, s_63_1);
            tracer.write_register(s_63_2 as isize, s_63_1);
        };
        // N s_63_4: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call __UNKNOWN_bit(s_64_0)
        let s_64_1: bool = u__UNKNOWN_bit(state, tracer, s_64_0);
        // C s_64_2: const #16973u : u32
        let s_64_2: u32 = 16973;
        // N s_64_3: write-reg s_64_2 <= s_64_1
        let s_64_3: () = {
            state.write_register::<bool>(s_64_2 as isize, s_64_1);
            tracer.write_register(s_64_2 as isize, s_64_1);
        };
        // N s_64_4: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call __UNKNOWN_bit(s_65_0)
        let s_65_1: bool = u__UNKNOWN_bit(state, tracer, s_65_0);
        // C s_65_2: const #16995u : u32
        let s_65_2: u32 = 16995;
        // N s_65_3: write-reg s_65_2 <= s_65_1
        let s_65_3: () = {
            state.write_register::<bool>(s_65_2 as isize, s_65_1);
            tracer.write_register(s_65_2 as isize, s_65_1);
        };
        // N s_65_4: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #2s : i64
        let s_66_0: i64 = 2;
        // C s_66_1: cast zx s_66_0 -> i
        let s_66_1: i128 = (i128::try_from(s_66_0).unwrap());
        // S s_66_2: call __UNKNOWN_bits(s_66_1)
        let s_66_2: Bits = u__UNKNOWN_bits(state, tracer, s_66_1);
        // S s_66_3: cast reint s_66_2 -> u8
        let s_66_3: u8 = (s_66_2.value() as u8);
        // C s_66_4: const #16970u : u32
        let s_66_4: u32 = 16970;
        // N s_66_5: write-reg s_66_4 <= s_66_3
        let s_66_5: () = {
            state.write_register::<u8>(s_66_4 as isize, s_66_3);
            tracer.write_register(s_66_4 as isize, s_66_3);
        };
        // N s_66_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call __UNKNOWN_bit(s_67_0)
        let s_67_1: bool = u__UNKNOWN_bit(state, tracer, s_67_0);
        // C s_67_2: const #16992u : u32
        let s_67_2: u32 = 16992;
        // N s_67_3: write-reg s_67_2 <= s_67_1
        let s_67_3: () = {
            state.write_register::<bool>(s_67_2 as isize, s_67_1);
            tracer.write_register(s_67_2 as isize, s_67_1);
        };
        // N s_67_4: jump b54
        return block_54(state, tracer, fn_state);
    }
}
