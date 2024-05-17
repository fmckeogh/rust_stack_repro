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
use u__id::*;
use HaveUAOExt::*;
use HaveSSBSExt::*;
use HaveDITExt::*;
use HaveMTEExt::*;
use UsingAArch32::*;
use Bit::*;
use Zeros::*;
use HaveFeatNMI::*;
use HavePANExt::*;
use HaveGCS::*;
use HaveBTIExt::*;
use common::*;
pub fn GetPSRFromPSTATE<T: Tracer>(
    state: &mut State,
    tracer: &T,
    targetELState: u32,
    N: i64,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_4378: bool,
        Nshadow_54: i64,
        spsr: Bits,
        gs_4419: bool,
        targetELState: u32,
        N: i64,
    }
    let fn_state = FunctionState {
        targetELState,
        N,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var N:i64
        let s_0_0: i64 = fn_state.N;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var Nshadow#54 <= s_0_2
        fn_state.Nshadow_54 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call UsingAArch32(s_0_4)
        let s_0_5: bool = UsingAArch32(state, tracer, s_0_4);
        // N s_0_6: branch s_0_5 b47 b1
        if s_0_5 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#4378 <= s_1_0
        fn_state.gs_4378 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var gs#4378:u8
        let s_2_0: bool = fn_state.gs_4378;
        // N s_2_1: branch s_2_0 b46 b3
        if s_2_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #64s : i
        let s_3_0: i128 = 64;
        // D s_3_1: read-var Nshadow#54:i64
        let s_3_1: i64 = fn_state.Nshadow_54;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: assert s_3_3
        let s_3_4: () = assert!(s_3_3);
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var Nshadow#54:i64
        let s_4_0: i64 = fn_state.Nshadow_54;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call Zeros(s_4_1)
        let s_4_2: Bits = Zeros(state, tracer, s_4_1);
        // D s_4_3: write-var spsr <= s_4_2
        fn_state.spsr = s_4_2;
        // C s_4_4: const #16984u : u32
        let s_4_4: u32 = 16984;
        // D s_4_5: read-reg s_4_4:u8
        let s_4_5: bool = {
            let value = state.read_register::<bool>(s_4_4 as isize);
            tracer.read_register(s_4_4 as isize, value);
            value
        };
        // C s_4_6: const #16997u : u32
        let s_4_6: u32 = 16997;
        // D s_4_7: read-reg s_4_6:u8
        let s_4_7: bool = {
            let value = state.read_register::<bool>(s_4_6 as isize);
            tracer.read_register(s_4_6 as isize, value);
            value
        };
        // C s_4_8: const #16971u : u32
        let s_4_8: u32 = 16971;
        // D s_4_9: read-reg s_4_8:u8
        let s_4_9: bool = {
            let value = state.read_register::<bool>(s_4_8 as isize);
            tracer.read_register(s_4_8 as isize, value);
            value
        };
        // C s_4_10: const #16996u : u32
        let s_4_10: u32 = 16996;
        // D s_4_11: read-reg s_4_10:u8
        let s_4_11: bool = {
            let value = state.read_register::<bool>(s_4_10 as isize);
            tracer.read_register(s_4_10 as isize, value);
            value
        };
        // D s_4_12: cast zx s_4_9 -> bv
        let s_4_12: Bits = Bits::new(s_4_9 as u128, 1u16);
        // D s_4_13: cast zx s_4_11 -> bv
        let s_4_13: Bits = Bits::new(s_4_11 as u128, 1u16);
        // D s_4_14: cast reint s_4_12 -> u128
        let s_4_14: u128 = (s_4_12.value() as u128);
        // D s_4_15: size-of s_4_12
        let s_4_15: u16 = s_4_12.length();
        // D s_4_16: cast reint s_4_13 -> u128
        let s_4_16: u128 = (s_4_13.value() as u128);
        // D s_4_17: size-of s_4_13
        let s_4_17: u16 = s_4_13.length();
        // D s_4_18: lsl s_4_14 s_4_17
        let s_4_18: u128 = s_4_14 << s_4_17;
        // D s_4_19: or s_4_18 s_4_16
        let s_4_19: u128 = ((s_4_18) | (s_4_16));
        // D s_4_20: add s_4_15 s_4_17
        let s_4_20: u16 = (s_4_15 + s_4_17);
        // D s_4_21: create-bits s_4_19 s_4_20
        let s_4_21: Bits = Bits::new(s_4_19, s_4_20);
        // D s_4_22: cast reint s_4_21 -> u8
        let s_4_22: u8 = (s_4_21.value() as u8);
        // D s_4_23: cast zx s_4_7 -> bv
        let s_4_23: Bits = Bits::new(s_4_7 as u128, 1u16);
        // D s_4_24: cast zx s_4_22 -> bv
        let s_4_24: Bits = Bits::new(s_4_22 as u128, 2u16);
        // D s_4_25: cast reint s_4_23 -> u128
        let s_4_25: u128 = (s_4_23.value() as u128);
        // D s_4_26: size-of s_4_23
        let s_4_26: u16 = s_4_23.length();
        // D s_4_27: cast reint s_4_24 -> u128
        let s_4_27: u128 = (s_4_24.value() as u128);
        // D s_4_28: size-of s_4_24
        let s_4_28: u16 = s_4_24.length();
        // D s_4_29: lsl s_4_25 s_4_28
        let s_4_29: u128 = s_4_25 << s_4_28;
        // D s_4_30: or s_4_29 s_4_27
        let s_4_30: u128 = ((s_4_29) | (s_4_27));
        // D s_4_31: add s_4_26 s_4_28
        let s_4_31: u16 = (s_4_26 + s_4_28);
        // D s_4_32: create-bits s_4_30 s_4_31
        let s_4_32: Bits = Bits::new(s_4_30, s_4_31);
        // D s_4_33: cast reint s_4_32 -> u8
        let s_4_33: u8 = (s_4_32.value() as u8);
        // D s_4_34: cast zx s_4_5 -> bv
        let s_4_34: Bits = Bits::new(s_4_5 as u128, 1u16);
        // D s_4_35: cast zx s_4_33 -> bv
        let s_4_35: Bits = Bits::new(s_4_33 as u128, 3u16);
        // D s_4_36: cast reint s_4_34 -> u128
        let s_4_36: u128 = (s_4_34.value() as u128);
        // D s_4_37: size-of s_4_34
        let s_4_37: u16 = s_4_34.length();
        // D s_4_38: cast reint s_4_35 -> u128
        let s_4_38: u128 = (s_4_35.value() as u128);
        // D s_4_39: size-of s_4_35
        let s_4_39: u16 = s_4_35.length();
        // D s_4_40: lsl s_4_36 s_4_39
        let s_4_40: u128 = s_4_36 << s_4_39;
        // D s_4_41: or s_4_40 s_4_38
        let s_4_41: u128 = ((s_4_40) | (s_4_38));
        // D s_4_42: add s_4_37 s_4_39
        let s_4_42: u16 = (s_4_37 + s_4_39);
        // D s_4_43: create-bits s_4_41 s_4_42
        let s_4_43: Bits = Bits::new(s_4_41, s_4_42);
        // D s_4_44: cast reint s_4_43 -> u8
        let s_4_44: u8 = (s_4_43.value() as u8);
        // C s_4_45: const #28s : i
        let s_4_45: i128 = 28;
        // D s_4_46: cast zx s_4_44 -> bv
        let s_4_46: Bits = Bits::new(s_4_44 as u128, 4u16);
        // D s_4_47: read-var spsr:bv
        let s_4_47: Bits = fn_state.spsr;
        // C s_4_48: const #3s : i
        let s_4_48: i128 = 3;
        // C s_4_49: const #1u : u64
        let s_4_49: u64 = 1;
        // C s_4_50: cast zx s_4_49 -> bv
        let s_4_50: Bits = Bits::new(s_4_49 as u128, 64u16);
        // C s_4_51: lsl s_4_50 s_4_48
        let s_4_51: Bits = s_4_50 << s_4_48;
        // C s_4_52: sub s_4_51 s_4_50
        let s_4_52: Bits = ((s_4_51) - (s_4_50));
        // D s_4_53: and s_4_46 s_4_52
        let s_4_53: Bits = ((s_4_46) & (s_4_52));
        // D s_4_54: lsl s_4_53 s_4_45
        let s_4_54: Bits = s_4_53 << s_4_45;
        // C s_4_55: lsl s_4_52 s_4_45
        let s_4_55: Bits = s_4_52 << s_4_45;
        // C s_4_56: cmpl s_4_55
        let s_4_56: Bits = !s_4_55;
        // D s_4_57: and s_4_47 s_4_56
        let s_4_57: Bits = ((s_4_47) & (s_4_56));
        // D s_4_58: or s_4_57 s_4_54
        let s_4_58: Bits = ((s_4_57) | (s_4_54));
        // D s_4_59: write-var spsr <= s_4_58
        fn_state.spsr = s_4_58;
        // C s_4_60: const #() : ()
        let s_4_60: () = ();
        // S s_4_61: call HavePANExt(s_4_60)
        let s_4_61: bool = HavePANExt(state, tracer, s_4_60);
        // N s_4_62: branch s_4_61 b45 b5
        if s_4_61 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_6_0: const #16980u : u32
        let s_6_0: u32 = 16980;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: bool = {
            let value = state.read_register::<bool>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call Bit(s_6_1)
        let s_6_2: bool = Bit(state, tracer, s_6_1);
        // C s_6_3: const #20s : i
        let s_6_3: i128 = 20;
        // D s_6_4: read-var spsr:bv
        let s_6_4: Bits = fn_state.spsr;
        // C s_6_5: const #1u : u64
        let s_6_5: u64 = 1;
        // D s_6_6: bit-insert s_6_4 s_6_4 s_6_3 s_6_5
        let s_6_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_6_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_6_4.length(),
            );
            (s_6_4 & mask) | (s_6_4 << s_6_3)
        };
        // D s_6_7: write-var spsr <= s_6_6
        fn_state.spsr = s_6_6;
        // C s_6_8: const #16999u : u32
        let s_6_8: u32 = 16999;
        // D s_6_9: read-reg s_6_8:u8
        let s_6_9: bool = {
            let value = state.read_register::<bool>(s_6_8 as isize);
            tracer.read_register(s_6_8 as isize, value);
            value
        };
        // D s_6_10: cast zx s_6_9 -> bv
        let s_6_10: Bits = Bits::new(s_6_9 as u128, 1u16);
        // C s_6_11: const #1u : u8
        let s_6_11: bool = true;
        // C s_6_12: cast zx s_6_11 -> bv
        let s_6_12: Bits = Bits::new(s_6_11 as u128, 1u16);
        // D s_6_13: cmp-eq s_6_10 s_6_12
        let s_6_13: bool = ((s_6_10) == (s_6_12));
        // N s_6_14: branch s_6_13 b30 b7
        if s_6_13 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HaveMTEExt(s_7_0)
        let s_7_1: bool = HaveMTEExt(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b29 b8
        if s_7_1 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call HaveGCS(s_9_0)
        let s_9_1: bool = HaveGCS(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b28 b10
        if s_9_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HaveDITExt(s_11_0)
        let s_11_1: bool = HaveDITExt(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b27 b12
        if s_11_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call HaveUAOExt(s_13_0)
        let s_13_1: bool = HaveUAOExt(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b26 b14
        if s_13_1 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_15_0: const #16991u : u32
        let s_15_0: u32 = 16991;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: bool = {
            let value = state.read_register::<bool>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call Bit(s_15_1)
        let s_15_2: bool = Bit(state, tracer, s_15_1);
        // C s_15_3: const #21s : i
        let s_15_3: i128 = 21;
        // D s_15_4: read-var spsr:bv
        let s_15_4: Bits = fn_state.spsr;
        // C s_15_5: const #1u : u64
        let s_15_5: u64 = 1;
        // D s_15_6: bit-insert s_15_4 s_15_4 s_15_3 s_15_5
        let s_15_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_15_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_15_4.length(),
            );
            (s_15_4 & mask) | (s_15_4 << s_15_3)
        };
        // D s_15_7: write-var spsr <= s_15_6
        fn_state.spsr = s_15_6;
        // C s_15_8: const #() : ()
        let s_15_8: () = ();
        // S s_15_9: call HaveFeatNMI(s_15_8)
        let s_15_9: bool = HaveFeatNMI(state, tracer, s_15_8);
        // N s_15_10: branch s_15_9 b25 b16
        if s_15_9 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call HaveSSBSExt(s_17_0)
        let s_17_1: bool = HaveSSBSExt(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b24 b18
        if s_17_1 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call HaveBTIExt(s_19_0)
        let s_19_1: bool = HaveBTIExt(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b23 b20
        if s_19_1 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_21_0: const #16972u : u32
        let s_21_0: u32 = 16972;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: bool = {
            let value = state.read_register::<bool>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // C s_21_2: const #16968u : u32
        let s_21_2: u32 = 16968;
        // D s_21_3: read-reg s_21_2:u8
        let s_21_3: bool = {
            let value = state.read_register::<bool>(s_21_2 as isize);
            tracer.read_register(s_21_2 as isize, value);
            value
        };
        // C s_21_4: const #16979u : u32
        let s_21_4: u32 = 16979;
        // D s_21_5: read-reg s_21_4:u8
        let s_21_5: bool = {
            let value = state.read_register::<bool>(s_21_4 as isize);
            tracer.read_register(s_21_4 as isize, value);
            value
        };
        // C s_21_6: const #16977u : u32
        let s_21_6: u32 = 16977;
        // D s_21_7: read-reg s_21_6:u8
        let s_21_7: bool = {
            let value = state.read_register::<bool>(s_21_6 as isize);
            tracer.read_register(s_21_6 as isize, value);
            value
        };
        // D s_21_8: cast zx s_21_5 -> bv
        let s_21_8: Bits = Bits::new(s_21_5 as u128, 1u16);
        // D s_21_9: cast zx s_21_7 -> bv
        let s_21_9: Bits = Bits::new(s_21_7 as u128, 1u16);
        // D s_21_10: cast reint s_21_8 -> u128
        let s_21_10: u128 = (s_21_8.value() as u128);
        // D s_21_11: size-of s_21_8
        let s_21_11: u16 = s_21_8.length();
        // D s_21_12: cast reint s_21_9 -> u128
        let s_21_12: u128 = (s_21_9.value() as u128);
        // D s_21_13: size-of s_21_9
        let s_21_13: u16 = s_21_9.length();
        // D s_21_14: lsl s_21_10 s_21_13
        let s_21_14: u128 = s_21_10 << s_21_13;
        // D s_21_15: or s_21_14 s_21_12
        let s_21_15: u128 = ((s_21_14) | (s_21_12));
        // D s_21_16: add s_21_11 s_21_13
        let s_21_16: u16 = (s_21_11 + s_21_13);
        // D s_21_17: create-bits s_21_15 s_21_16
        let s_21_17: Bits = Bits::new(s_21_15, s_21_16);
        // D s_21_18: cast reint s_21_17 -> u8
        let s_21_18: u8 = (s_21_17.value() as u8);
        // D s_21_19: cast zx s_21_3 -> bv
        let s_21_19: Bits = Bits::new(s_21_3 as u128, 1u16);
        // D s_21_20: cast zx s_21_18 -> bv
        let s_21_20: Bits = Bits::new(s_21_18 as u128, 2u16);
        // D s_21_21: cast reint s_21_19 -> u128
        let s_21_21: u128 = (s_21_19.value() as u128);
        // D s_21_22: size-of s_21_19
        let s_21_22: u16 = s_21_19.length();
        // D s_21_23: cast reint s_21_20 -> u128
        let s_21_23: u128 = (s_21_20.value() as u128);
        // D s_21_24: size-of s_21_20
        let s_21_24: u16 = s_21_20.length();
        // D s_21_25: lsl s_21_21 s_21_24
        let s_21_25: u128 = s_21_21 << s_21_24;
        // D s_21_26: or s_21_25 s_21_23
        let s_21_26: u128 = ((s_21_25) | (s_21_23));
        // D s_21_27: add s_21_22 s_21_24
        let s_21_27: u16 = (s_21_22 + s_21_24);
        // D s_21_28: create-bits s_21_26 s_21_27
        let s_21_28: Bits = Bits::new(s_21_26, s_21_27);
        // D s_21_29: cast reint s_21_28 -> u8
        let s_21_29: u8 = (s_21_28.value() as u8);
        // D s_21_30: cast zx s_21_1 -> bv
        let s_21_30: Bits = Bits::new(s_21_1 as u128, 1u16);
        // D s_21_31: cast zx s_21_29 -> bv
        let s_21_31: Bits = Bits::new(s_21_29 as u128, 3u16);
        // D s_21_32: cast reint s_21_30 -> u128
        let s_21_32: u128 = (s_21_30.value() as u128);
        // D s_21_33: size-of s_21_30
        let s_21_33: u16 = s_21_30.length();
        // D s_21_34: cast reint s_21_31 -> u128
        let s_21_34: u128 = (s_21_31.value() as u128);
        // D s_21_35: size-of s_21_31
        let s_21_35: u16 = s_21_31.length();
        // D s_21_36: lsl s_21_32 s_21_35
        let s_21_36: u128 = s_21_32 << s_21_35;
        // D s_21_37: or s_21_36 s_21_34
        let s_21_37: u128 = ((s_21_36) | (s_21_34));
        // D s_21_38: add s_21_33 s_21_35
        let s_21_38: u16 = (s_21_33 + s_21_35);
        // D s_21_39: create-bits s_21_37 s_21_38
        let s_21_39: Bits = Bits::new(s_21_37, s_21_38);
        // D s_21_40: cast reint s_21_39 -> u8
        let s_21_40: u8 = (s_21_39.value() as u8);
        // C s_21_41: const #6s : i
        let s_21_41: i128 = 6;
        // D s_21_42: cast zx s_21_40 -> bv
        let s_21_42: Bits = Bits::new(s_21_40 as u128, 4u16);
        // D s_21_43: read-var spsr:bv
        let s_21_43: Bits = fn_state.spsr;
        // C s_21_44: const #3s : i
        let s_21_44: i128 = 3;
        // C s_21_45: const #1u : u64
        let s_21_45: u64 = 1;
        // C s_21_46: cast zx s_21_45 -> bv
        let s_21_46: Bits = Bits::new(s_21_45 as u128, 64u16);
        // C s_21_47: lsl s_21_46 s_21_44
        let s_21_47: Bits = s_21_46 << s_21_44;
        // C s_21_48: sub s_21_47 s_21_46
        let s_21_48: Bits = ((s_21_47) - (s_21_46));
        // D s_21_49: and s_21_42 s_21_48
        let s_21_49: Bits = ((s_21_42) & (s_21_48));
        // D s_21_50: lsl s_21_49 s_21_41
        let s_21_50: Bits = s_21_49 << s_21_41;
        // C s_21_51: lsl s_21_48 s_21_41
        let s_21_51: Bits = s_21_48 << s_21_41;
        // C s_21_52: cmpl s_21_51
        let s_21_52: Bits = !s_21_51;
        // D s_21_53: and s_21_43 s_21_52
        let s_21_53: Bits = ((s_21_43) & (s_21_52));
        // D s_21_54: or s_21_53 s_21_50
        let s_21_54: Bits = ((s_21_53) | (s_21_50));
        // D s_21_55: write-var spsr <= s_21_54
        fn_state.spsr = s_21_54;
        // C s_21_56: const #16999u : u32
        let s_21_56: u32 = 16999;
        // D s_21_57: read-reg s_21_56:u8
        let s_21_57: bool = {
            let value = state.read_register::<bool>(s_21_56 as isize);
            tracer.read_register(s_21_56 as isize, value);
            value
        };
        // D s_21_58: call Bit(s_21_57)
        let s_21_58: bool = Bit(state, tracer, s_21_57);
        // C s_21_59: const #4s : i
        let s_21_59: i128 = 4;
        // D s_21_60: read-var spsr:bv
        let s_21_60: Bits = fn_state.spsr;
        // C s_21_61: const #1u : u64
        let s_21_61: u64 = 1;
        // D s_21_62: bit-insert s_21_60 s_21_60 s_21_59 s_21_61
        let s_21_62: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_21_61 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_21_60.length(),
            );
            (s_21_60 & mask) | (s_21_60 << s_21_59)
        };
        // D s_21_63: write-var spsr <= s_21_62
        fn_state.spsr = s_21_62;
        // C s_21_64: const #16975u : u32
        let s_21_64: u32 = 16975;
        // D s_21_65: read-reg s_21_64:u8
        let s_21_65: u8 = {
            let value = state.read_register::<u8>(s_21_64 as isize);
            tracer.read_register(s_21_64 as isize, value);
            value
        };
        // C s_21_66: const #2s : i
        let s_21_66: i128 = 2;
        // D s_21_67: cast zx s_21_65 -> bv
        let s_21_67: Bits = Bits::new(s_21_65 as u128, 2u16);
        // D s_21_68: read-var spsr:bv
        let s_21_68: Bits = fn_state.spsr;
        // C s_21_69: const #1s : i
        let s_21_69: i128 = 1;
        // C s_21_70: const #1u : u64
        let s_21_70: u64 = 1;
        // C s_21_71: cast zx s_21_70 -> bv
        let s_21_71: Bits = Bits::new(s_21_70 as u128, 64u16);
        // C s_21_72: lsl s_21_71 s_21_69
        let s_21_72: Bits = s_21_71 << s_21_69;
        // C s_21_73: sub s_21_72 s_21_71
        let s_21_73: Bits = ((s_21_72) - (s_21_71));
        // D s_21_74: and s_21_67 s_21_73
        let s_21_74: Bits = ((s_21_67) & (s_21_73));
        // D s_21_75: lsl s_21_74 s_21_66
        let s_21_75: Bits = s_21_74 << s_21_66;
        // C s_21_76: lsl s_21_73 s_21_66
        let s_21_76: Bits = s_21_73 << s_21_66;
        // C s_21_77: cmpl s_21_76
        let s_21_77: Bits = !s_21_76;
        // D s_21_78: and s_21_68 s_21_77
        let s_21_78: Bits = ((s_21_68) & (s_21_77));
        // D s_21_79: or s_21_78 s_21_75
        let s_21_79: Bits = ((s_21_78) | (s_21_75));
        // D s_21_80: write-var spsr <= s_21_79
        fn_state.spsr = s_21_79;
        // C s_21_81: const #16990u : u32
        let s_21_81: u32 = 16990;
        // D s_21_82: read-reg s_21_81:u8
        let s_21_82: bool = {
            let value = state.read_register::<bool>(s_21_81 as isize);
            tracer.read_register(s_21_81 as isize, value);
            value
        };
        // D s_21_83: call Bit(s_21_82)
        let s_21_83: bool = Bit(state, tracer, s_21_82);
        // C s_21_84: const #0s : i
        let s_21_84: i128 = 0;
        // D s_21_85: read-var spsr:bv
        let s_21_85: Bits = fn_state.spsr;
        // C s_21_86: const #1u : u64
        let s_21_86: u64 = 1;
        // D s_21_87: bit-insert s_21_85 s_21_85 s_21_84 s_21_86
        let s_21_87: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_21_86 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_21_85.length(),
            );
            (s_21_85 & mask) | (s_21_85 << s_21_84)
        };
        // D s_21_88: write-var spsr <= s_21_87
        fn_state.spsr = s_21_87;
        // N s_21_89: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_22_0: read-var spsr:bv
        let s_22_0: Bits = fn_state.spsr;
        // N s_22_1: return s_22_0
        return s_22_0;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_23_0: const #16970u : u32
        let s_23_0: u32 = 16970;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // C s_23_2: const #10s : i
        let s_23_2: i128 = 10;
        // D s_23_3: cast zx s_23_1 -> bv
        let s_23_3: Bits = Bits::new(s_23_1 as u128, 2u16);
        // D s_23_4: read-var spsr:bv
        let s_23_4: Bits = fn_state.spsr;
        // C s_23_5: const #1s : i
        let s_23_5: i128 = 1;
        // C s_23_6: const #1u : u64
        let s_23_6: u64 = 1;
        // C s_23_7: cast zx s_23_6 -> bv
        let s_23_7: Bits = Bits::new(s_23_6 as u128, 64u16);
        // C s_23_8: lsl s_23_7 s_23_5
        let s_23_8: Bits = s_23_7 << s_23_5;
        // C s_23_9: sub s_23_8 s_23_7
        let s_23_9: Bits = ((s_23_8) - (s_23_7));
        // D s_23_10: and s_23_3 s_23_9
        let s_23_10: Bits = ((s_23_3) & (s_23_9));
        // D s_23_11: lsl s_23_10 s_23_2
        let s_23_11: Bits = s_23_10 << s_23_2;
        // C s_23_12: lsl s_23_9 s_23_2
        let s_23_12: Bits = s_23_9 << s_23_2;
        // C s_23_13: cmpl s_23_12
        let s_23_13: Bits = !s_23_12;
        // D s_23_14: and s_23_4 s_23_13
        let s_23_14: Bits = ((s_23_4) & (s_23_13));
        // D s_23_15: or s_23_14 s_23_11
        let s_23_15: Bits = ((s_23_14) | (s_23_11));
        // D s_23_16: write-var spsr <= s_23_15
        fn_state.spsr = s_23_15;
        // N s_23_17: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_24_0: const #16992u : u32
        let s_24_0: u32 = 16992;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: bool = {
            let value = state.read_register::<bool>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call Bit(s_24_1)
        let s_24_2: bool = Bit(state, tracer, s_24_1);
        // C s_24_3: const #12s : i
        let s_24_3: i128 = 12;
        // D s_24_4: read-var spsr:bv
        let s_24_4: Bits = fn_state.spsr;
        // C s_24_5: const #1u : u64
        let s_24_5: u64 = 1;
        // D s_24_6: bit-insert s_24_4 s_24_4 s_24_3 s_24_5
        let s_24_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_24_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_24_4.length(),
            );
            (s_24_4 & mask) | (s_24_4 << s_24_3)
        };
        // D s_24_7: write-var spsr <= s_24_6
        fn_state.spsr = s_24_6;
        // N s_24_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_25_0: const #16969u : u32
        let s_25_0: u32 = 16969;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: bool = {
            let value = state.read_register::<bool>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call Bit(s_25_1)
        let s_25_2: bool = Bit(state, tracer, s_25_1);
        // C s_25_3: const #13s : i
        let s_25_3: i128 = 13;
        // D s_25_4: read-var spsr:bv
        let s_25_4: Bits = fn_state.spsr;
        // C s_25_5: const #1u : u64
        let s_25_5: u64 = 1;
        // D s_25_6: bit-insert s_25_4 s_25_4 s_25_3 s_25_5
        let s_25_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_25_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_25_4.length(),
            );
            (s_25_4 & mask) | (s_25_4 << s_25_3)
        };
        // D s_25_7: write-var spsr <= s_25_6
        fn_state.spsr = s_25_6;
        // N s_25_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_26_0: const #16995u : u32
        let s_26_0: u32 = 16995;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: bool = {
            let value = state.read_register::<bool>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call Bit(s_26_1)
        let s_26_2: bool = Bit(state, tracer, s_26_1);
        // C s_26_3: const #23s : i
        let s_26_3: i128 = 23;
        // D s_26_4: read-var spsr:bv
        let s_26_4: Bits = fn_state.spsr;
        // C s_26_5: const #1u : u64
        let s_26_5: u64 = 1;
        // D s_26_6: bit-insert s_26_4 s_26_4 s_26_3 s_26_5
        let s_26_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_26_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_26_4.length(),
            );
            (s_26_4 & mask) | (s_26_4 << s_26_3)
        };
        // D s_26_7: write-var spsr <= s_26_6
        fn_state.spsr = s_26_6;
        // N s_26_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_27_0: const #16973u : u32
        let s_27_0: u32 = 16973;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: bool = {
            let value = state.read_register::<bool>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call Bit(s_27_1)
        let s_27_2: bool = Bit(state, tracer, s_27_1);
        // C s_27_3: const #24s : i
        let s_27_3: i128 = 24;
        // D s_27_4: read-var spsr:bv
        let s_27_4: Bits = fn_state.spsr;
        // C s_27_5: const #1u : u64
        let s_27_5: u64 = 1;
        // D s_27_6: bit-insert s_27_4 s_27_4 s_27_3 s_27_5
        let s_27_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_27_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_27_4.length(),
            );
            (s_27_4 & mask) | (s_27_4 << s_27_3)
        };
        // D s_27_7: write-var spsr <= s_27_6
        fn_state.spsr = s_27_6;
        // N s_27_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_28_0: read-var Nshadow#54:i64
        let s_28_0: i64 = fn_state.Nshadow_54;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: call __id(s_28_1)
        let s_28_2: i128 = u__id(state, tracer, s_28_1);
        // D s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: const #34s : i
        let s_28_4: i128 = 34;
        // D s_28_5: cast zx s_28_3 -> i
        let s_28_5: i128 = (i128::try_from(s_28_3).unwrap());
        // D s_28_6: cmp-lt s_28_4 s_28_5
        let s_28_6: bool = ((s_28_4) < (s_28_5));
        // N s_28_7: assert s_28_6
        let s_28_7: () = assert!(s_28_6);
        // C s_28_8: const #16976u : u32
        let s_28_8: u32 = 16976;
        // D s_28_9: read-reg s_28_8:u8
        let s_28_9: bool = {
            let value = state.read_register::<bool>(s_28_8 as isize);
            tracer.read_register(s_28_8 as isize, value);
            value
        };
        // D s_28_10: call Bit(s_28_9)
        let s_28_10: bool = Bit(state, tracer, s_28_9);
        // C s_28_11: const #34s : i
        let s_28_11: i128 = 34;
        // D s_28_12: read-var spsr:bv
        let s_28_12: Bits = fn_state.spsr;
        // C s_28_13: const #1u : u64
        let s_28_13: u64 = 1;
        // D s_28_14: bit-insert s_28_12 s_28_12 s_28_11 s_28_13
        let s_28_14: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_28_13 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_28_12.length(),
            );
            (s_28_12 & mask) | (s_28_12 << s_28_11)
        };
        // D s_28_15: write-var spsr <= s_28_14
        fn_state.spsr = s_28_14;
        // N s_28_16: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_29_0: const #16994u : u32
        let s_29_0: u32 = 16994;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: bool = {
            let value = state.read_register::<bool>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call Bit(s_29_1)
        let s_29_2: bool = Bit(state, tracer, s_29_1);
        // C s_29_3: const #25s : i
        let s_29_3: i128 = 25;
        // D s_29_4: read-var spsr:bv
        let s_29_4: Bits = fn_state.spsr;
        // C s_29_5: const #1u : u64
        let s_29_5: u64 = 1;
        // D s_29_6: bit-insert s_29_4 s_29_4 s_29_3 s_29_5
        let s_29_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_29_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_29_4.length(),
            );
            (s_29_4 & mask) | (s_29_4 << s_29_3)
        };
        // D s_29_7: write-var spsr <= s_29_6
        fn_state.spsr = s_29_6;
        // N s_29_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_30_0: const #16988u : u32
        let s_30_0: u32 = 16988;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: bool = {
            let value = state.read_register::<bool>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call Bit(s_30_1)
        let s_30_2: bool = Bit(state, tracer, s_30_1);
        // C s_30_3: const #27s : i
        let s_30_3: i128 = 27;
        // D s_30_4: read-var spsr:bv
        let s_30_4: Bits = fn_state.spsr;
        // C s_30_5: const #1u : u64
        let s_30_5: u64 = 1;
        // D s_30_6: bit-insert s_30_4 s_30_4 s_30_3 s_30_5
        let s_30_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_30_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_30_4.length(),
            );
            (s_30_4 & mask) | (s_30_4 << s_30_3)
        };
        // D s_30_7: write-var spsr <= s_30_6
        fn_state.spsr = s_30_6;
        // C s_30_8: const #16981u : u32
        let s_30_8: u32 = 16981;
        // D s_30_9: read-reg s_30_8:u8
        let s_30_9: u8 = {
            let value = state.read_register::<u8>(s_30_8 as isize);
            tracer.read_register(s_30_8 as isize, value);
            value
        };
        // C s_30_10: const #0s : i
        let s_30_10: i128 = 0;
        // D s_30_11: cast zx s_30_9 -> bv
        let s_30_11: Bits = Bits::new(s_30_9 as u128, 8u16);
        // C s_30_12: const #1s : i64
        let s_30_12: i64 = 1;
        // C s_30_13: cast zx s_30_12 -> i
        let s_30_13: i128 = (i128::try_from(s_30_12).unwrap());
        // C s_30_14: const #1s : i
        let s_30_14: i128 = 1;
        // C s_30_15: add s_30_14 s_30_13
        let s_30_15: i128 = (s_30_14 + s_30_13);
        // D s_30_16: bit-extract s_30_11 s_30_10 s_30_15
        let s_30_16: Bits = (Bits::new(
            ((s_30_11) >> (s_30_10)).value(),
            u16::try_from(s_30_15).unwrap(),
        ));
        // D s_30_17: cast reint s_30_16 -> u8
        let s_30_17: u8 = (s_30_16.value() as u8);
        // C s_30_18: const #25s : i
        let s_30_18: i128 = 25;
        // D s_30_19: cast zx s_30_17 -> bv
        let s_30_19: Bits = Bits::new(s_30_17 as u128, 2u16);
        // D s_30_20: read-var spsr:bv
        let s_30_20: Bits = fn_state.spsr;
        // C s_30_21: const #1s : i
        let s_30_21: i128 = 1;
        // C s_30_22: const #1u : u64
        let s_30_22: u64 = 1;
        // C s_30_23: cast zx s_30_22 -> bv
        let s_30_23: Bits = Bits::new(s_30_22 as u128, 64u16);
        // C s_30_24: lsl s_30_23 s_30_21
        let s_30_24: Bits = s_30_23 << s_30_21;
        // C s_30_25: sub s_30_24 s_30_23
        let s_30_25: Bits = ((s_30_24) - (s_30_23));
        // D s_30_26: and s_30_19 s_30_25
        let s_30_26: Bits = ((s_30_19) & (s_30_25));
        // D s_30_27: lsl s_30_26 s_30_18
        let s_30_27: Bits = s_30_26 << s_30_18;
        // C s_30_28: lsl s_30_25 s_30_18
        let s_30_28: Bits = s_30_25 << s_30_18;
        // C s_30_29: cmpl s_30_28
        let s_30_29: Bits = !s_30_28;
        // D s_30_30: and s_30_20 s_30_29
        let s_30_30: Bits = ((s_30_20) & (s_30_29));
        // D s_30_31: or s_30_30 s_30_27
        let s_30_31: Bits = ((s_30_30) | (s_30_27));
        // D s_30_32: write-var spsr <= s_30_31
        fn_state.spsr = s_30_31;
        // C s_30_33: const #() : ()
        let s_30_33: () = ();
        // S s_30_34: call HaveSSBSExt(s_30_33)
        let s_30_34: bool = HaveSSBSExt(state, tracer, s_30_33);
        // N s_30_35: branch s_30_34 b44 b31
        if s_30_34 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call HaveDITExt(s_32_0)
        let s_32_1: bool = HaveDITExt(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b41 b33
        if s_32_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_34_0: read-var targetELState:u32
        let s_34_0: u32 = fn_state.targetELState;
        // C s_34_1: const #1u : u32
        let s_34_1: u32 = 1;
        // D s_34_2: cmp-eq s_34_0 s_34_1
        let s_34_2: bool = ((s_34_0) == (s_34_1));
        // N s_34_3: branch s_34_2 b40 b35
        if s_34_2 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_35_0: read-var targetELState:u32
        let s_35_0: u32 = fn_state.targetELState;
        // C s_35_1: const #2u : u32
        let s_35_1: u32 = 2;
        // D s_35_2: cmp-eq s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) == (s_35_1));
        // D s_35_3: write-var gs#4419 <= s_35_2
        fn_state.gs_4419 = s_35_2;
        // N s_35_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_36_0: read-var gs#4419:u8
        let s_36_0: bool = fn_state.gs_4419;
        // N s_36_1: branch s_36_0 b39 b37
        if s_36_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_37_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_38_0: const #16978u : u32
        let s_38_0: u32 = 16978;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // C s_38_2: const #16s : i
        let s_38_2: i128 = 16;
        // D s_38_3: cast zx s_38_1 -> bv
        let s_38_3: Bits = Bits::new(s_38_1 as u128, 4u16);
        // D s_38_4: read-var spsr:bv
        let s_38_4: Bits = fn_state.spsr;
        // C s_38_5: const #3s : i
        let s_38_5: i128 = 3;
        // C s_38_6: const #1u : u64
        let s_38_6: u64 = 1;
        // C s_38_7: cast zx s_38_6 -> bv
        let s_38_7: Bits = Bits::new(s_38_6 as u128, 64u16);
        // C s_38_8: lsl s_38_7 s_38_5
        let s_38_8: Bits = s_38_7 << s_38_5;
        // C s_38_9: sub s_38_8 s_38_7
        let s_38_9: Bits = ((s_38_8) - (s_38_7));
        // D s_38_10: and s_38_3 s_38_9
        let s_38_10: Bits = ((s_38_3) & (s_38_9));
        // D s_38_11: lsl s_38_10 s_38_2
        let s_38_11: Bits = s_38_10 << s_38_2;
        // C s_38_12: lsl s_38_9 s_38_2
        let s_38_12: Bits = s_38_9 << s_38_2;
        // C s_38_13: cmpl s_38_12
        let s_38_13: Bits = !s_38_12;
        // D s_38_14: and s_38_4 s_38_13
        let s_38_14: Bits = ((s_38_4) & (s_38_13));
        // D s_38_15: or s_38_14 s_38_11
        let s_38_15: Bits = ((s_38_14) | (s_38_11));
        // D s_38_16: write-var spsr <= s_38_15
        fn_state.spsr = s_38_15;
        // C s_38_17: const #16981u : u32
        let s_38_17: u32 = 16981;
        // D s_38_18: read-reg s_38_17:u8
        let s_38_18: u8 = {
            let value = state.read_register::<u8>(s_38_17 as isize);
            tracer.read_register(s_38_17 as isize, value);
            value
        };
        // C s_38_19: const #2s : i
        let s_38_19: i128 = 2;
        // D s_38_20: cast zx s_38_18 -> bv
        let s_38_20: Bits = Bits::new(s_38_18 as u128, 8u16);
        // C s_38_21: const #1s : i64
        let s_38_21: i64 = 1;
        // C s_38_22: cast zx s_38_21 -> i
        let s_38_22: i128 = (i128::try_from(s_38_21).unwrap());
        // C s_38_23: const #5s : i
        let s_38_23: i128 = 5;
        // C s_38_24: add s_38_23 s_38_22
        let s_38_24: i128 = (s_38_23 + s_38_22);
        // D s_38_25: bit-extract s_38_20 s_38_19 s_38_24
        let s_38_25: Bits = (Bits::new(
            ((s_38_20) >> (s_38_19)).value(),
            u16::try_from(s_38_24).unwrap(),
        ));
        // D s_38_26: cast reint s_38_25 -> u8
        let s_38_26: u8 = (s_38_25.value() as u8);
        // C s_38_27: const #10s : i
        let s_38_27: i128 = 10;
        // D s_38_28: cast zx s_38_26 -> bv
        let s_38_28: Bits = Bits::new(s_38_26 as u128, 6u16);
        // D s_38_29: read-var spsr:bv
        let s_38_29: Bits = fn_state.spsr;
        // C s_38_30: const #5s : i
        let s_38_30: i128 = 5;
        // C s_38_31: const #1u : u64
        let s_38_31: u64 = 1;
        // C s_38_32: cast zx s_38_31 -> bv
        let s_38_32: Bits = Bits::new(s_38_31 as u128, 64u16);
        // C s_38_33: lsl s_38_32 s_38_30
        let s_38_33: Bits = s_38_32 << s_38_30;
        // C s_38_34: sub s_38_33 s_38_32
        let s_38_34: Bits = ((s_38_33) - (s_38_32));
        // D s_38_35: and s_38_28 s_38_34
        let s_38_35: Bits = ((s_38_28) & (s_38_34));
        // D s_38_36: lsl s_38_35 s_38_27
        let s_38_36: Bits = s_38_35 << s_38_27;
        // C s_38_37: lsl s_38_34 s_38_27
        let s_38_37: Bits = s_38_34 << s_38_27;
        // C s_38_38: cmpl s_38_37
        let s_38_38: Bits = !s_38_37;
        // D s_38_39: and s_38_29 s_38_38
        let s_38_39: Bits = ((s_38_29) & (s_38_38));
        // D s_38_40: or s_38_39 s_38_36
        let s_38_40: Bits = ((s_38_39) | (s_38_36));
        // D s_38_41: write-var spsr <= s_38_40
        fn_state.spsr = s_38_40;
        // C s_38_42: const #16974u : u32
        let s_38_42: u32 = 16974;
        // D s_38_43: read-reg s_38_42:u8
        let s_38_43: bool = {
            let value = state.read_register::<bool>(s_38_42 as isize);
            tracer.read_register(s_38_42 as isize, value);
            value
        };
        // D s_38_44: call Bit(s_38_43)
        let s_38_44: bool = Bit(state, tracer, s_38_43);
        // C s_38_45: const #9s : i
        let s_38_45: i128 = 9;
        // D s_38_46: read-var spsr:bv
        let s_38_46: Bits = fn_state.spsr;
        // C s_38_47: const #1u : u64
        let s_38_47: u64 = 1;
        // D s_38_48: bit-insert s_38_46 s_38_46 s_38_45 s_38_47
        let s_38_48: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_38_47 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_38_46.length(),
            );
            (s_38_46 & mask) | (s_38_46 << s_38_45)
        };
        // D s_38_49: write-var spsr <= s_38_48
        fn_state.spsr = s_38_48;
        // C s_38_50: const #16968u : u32
        let s_38_50: u32 = 16968;
        // D s_38_51: read-reg s_38_50:u8
        let s_38_51: bool = {
            let value = state.read_register::<bool>(s_38_50 as isize);
            tracer.read_register(s_38_50 as isize, value);
            value
        };
        // C s_38_52: const #16979u : u32
        let s_38_52: u32 = 16979;
        // D s_38_53: read-reg s_38_52:u8
        let s_38_53: bool = {
            let value = state.read_register::<bool>(s_38_52 as isize);
            tracer.read_register(s_38_52 as isize, value);
            value
        };
        // C s_38_54: const #16977u : u32
        let s_38_54: u32 = 16977;
        // D s_38_55: read-reg s_38_54:u8
        let s_38_55: bool = {
            let value = state.read_register::<bool>(s_38_54 as isize);
            tracer.read_register(s_38_54 as isize, value);
            value
        };
        // D s_38_56: cast zx s_38_53 -> bv
        let s_38_56: Bits = Bits::new(s_38_53 as u128, 1u16);
        // D s_38_57: cast zx s_38_55 -> bv
        let s_38_57: Bits = Bits::new(s_38_55 as u128, 1u16);
        // D s_38_58: cast reint s_38_56 -> u128
        let s_38_58: u128 = (s_38_56.value() as u128);
        // D s_38_59: size-of s_38_56
        let s_38_59: u16 = s_38_56.length();
        // D s_38_60: cast reint s_38_57 -> u128
        let s_38_60: u128 = (s_38_57.value() as u128);
        // D s_38_61: size-of s_38_57
        let s_38_61: u16 = s_38_57.length();
        // D s_38_62: lsl s_38_58 s_38_61
        let s_38_62: u128 = s_38_58 << s_38_61;
        // D s_38_63: or s_38_62 s_38_60
        let s_38_63: u128 = ((s_38_62) | (s_38_60));
        // D s_38_64: add s_38_59 s_38_61
        let s_38_64: u16 = (s_38_59 + s_38_61);
        // D s_38_65: create-bits s_38_63 s_38_64
        let s_38_65: Bits = Bits::new(s_38_63, s_38_64);
        // D s_38_66: cast reint s_38_65 -> u8
        let s_38_66: u8 = (s_38_65.value() as u8);
        // D s_38_67: cast zx s_38_51 -> bv
        let s_38_67: Bits = Bits::new(s_38_51 as u128, 1u16);
        // D s_38_68: cast zx s_38_66 -> bv
        let s_38_68: Bits = Bits::new(s_38_66 as u128, 2u16);
        // D s_38_69: cast reint s_38_67 -> u128
        let s_38_69: u128 = (s_38_67.value() as u128);
        // D s_38_70: size-of s_38_67
        let s_38_70: u16 = s_38_67.length();
        // D s_38_71: cast reint s_38_68 -> u128
        let s_38_71: u128 = (s_38_68.value() as u128);
        // D s_38_72: size-of s_38_68
        let s_38_72: u16 = s_38_68.length();
        // D s_38_73: lsl s_38_69 s_38_72
        let s_38_73: u128 = s_38_69 << s_38_72;
        // D s_38_74: or s_38_73 s_38_71
        let s_38_74: u128 = ((s_38_73) | (s_38_71));
        // D s_38_75: add s_38_70 s_38_72
        let s_38_75: u16 = (s_38_70 + s_38_72);
        // D s_38_76: create-bits s_38_74 s_38_75
        let s_38_76: Bits = Bits::new(s_38_74, s_38_75);
        // D s_38_77: cast reint s_38_76 -> u8
        let s_38_77: u8 = (s_38_76.value() as u8);
        // C s_38_78: const #6s : i
        let s_38_78: i128 = 6;
        // D s_38_79: cast zx s_38_77 -> bv
        let s_38_79: Bits = Bits::new(s_38_77 as u128, 3u16);
        // D s_38_80: read-var spsr:bv
        let s_38_80: Bits = fn_state.spsr;
        // C s_38_81: const #2s : i
        let s_38_81: i128 = 2;
        // C s_38_82: const #1u : u64
        let s_38_82: u64 = 1;
        // C s_38_83: cast zx s_38_82 -> bv
        let s_38_83: Bits = Bits::new(s_38_82 as u128, 64u16);
        // C s_38_84: lsl s_38_83 s_38_81
        let s_38_84: Bits = s_38_83 << s_38_81;
        // C s_38_85: sub s_38_84 s_38_83
        let s_38_85: Bits = ((s_38_84) - (s_38_83));
        // D s_38_86: and s_38_79 s_38_85
        let s_38_86: Bits = ((s_38_79) & (s_38_85));
        // D s_38_87: lsl s_38_86 s_38_78
        let s_38_87: Bits = s_38_86 << s_38_78;
        // C s_38_88: lsl s_38_85 s_38_78
        let s_38_88: Bits = s_38_85 << s_38_78;
        // C s_38_89: cmpl s_38_88
        let s_38_89: Bits = !s_38_88;
        // D s_38_90: and s_38_80 s_38_89
        let s_38_90: Bits = ((s_38_80) & (s_38_89));
        // D s_38_91: or s_38_90 s_38_87
        let s_38_91: Bits = ((s_38_90) | (s_38_87));
        // D s_38_92: write-var spsr <= s_38_91
        fn_state.spsr = s_38_91;
        // C s_38_93: const #16993u : u32
        let s_38_93: u32 = 16993;
        // D s_38_94: read-reg s_38_93:u8
        let s_38_94: bool = {
            let value = state.read_register::<bool>(s_38_93 as isize);
            tracer.read_register(s_38_93 as isize, value);
            value
        };
        // D s_38_95: call Bit(s_38_94)
        let s_38_95: bool = Bit(state, tracer, s_38_94);
        // C s_38_96: const #5s : i
        let s_38_96: i128 = 5;
        // D s_38_97: read-var spsr:bv
        let s_38_97: Bits = fn_state.spsr;
        // C s_38_98: const #1u : u64
        let s_38_98: u64 = 1;
        // D s_38_99: bit-insert s_38_97 s_38_97 s_38_96 s_38_98
        let s_38_99: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_38_98 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_38_97.length(),
            );
            (s_38_97 & mask) | (s_38_97 << s_38_96)
        };
        // D s_38_100: write-var spsr <= s_38_99
        fn_state.spsr = s_38_99;
        // C s_38_101: const #16983u : u32
        let s_38_101: u32 = 16983;
        // D s_38_102: read-reg s_38_101:u8
        let s_38_102: u8 = {
            let value = state.read_register::<u8>(s_38_101 as isize);
            tracer.read_register(s_38_101 as isize, value);
            value
        };
        // C s_38_103: const #4s : i
        let s_38_103: i128 = 4;
        // D s_38_104: cast zx s_38_102 -> bv
        let s_38_104: Bits = Bits::new(s_38_102 as u128, 5u16);
        // C s_38_105: const #1u : u64
        let s_38_105: u64 = 1;
        // D s_38_106: bit-extract s_38_104 s_38_103 s_38_105
        let s_38_106: Bits = (Bits::new(
            ((s_38_104) >> (s_38_103)).value(),
            u16::try_from(s_38_105).unwrap(),
        ));
        // D s_38_107: cast reint s_38_106 -> u8
        let s_38_107: bool = ((s_38_106.value()) != 0);
        // C s_38_108: const #0s : i
        let s_38_108: i128 = 0;
        // C s_38_109: const #0u : u64
        let s_38_109: u64 = 0;
        // D s_38_110: cast zx s_38_107 -> u64
        let s_38_110: u64 = (s_38_107 as u64);
        // C s_38_111: const #1u : u64
        let s_38_111: u64 = 1;
        // D s_38_112: and s_38_110 s_38_111
        let s_38_112: u64 = ((s_38_110) & (s_38_111));
        // D s_38_113: cmp-eq s_38_112 s_38_111
        let s_38_113: bool = ((s_38_112) == (s_38_111));
        // D s_38_114: lsl s_38_110 s_38_108
        let s_38_114: u64 = s_38_110 << s_38_108;
        // D s_38_115: or s_38_109 s_38_114
        let s_38_115: u64 = ((s_38_109) | (s_38_114));
        // D s_38_116: cmpl s_38_114
        let s_38_116: u64 = !s_38_114;
        // D s_38_117: and s_38_109 s_38_116
        let s_38_117: u64 = ((s_38_109) & (s_38_116));
        // D s_38_118: select s_38_113 s_38_115 s_38_117
        let s_38_118: u64 = if s_38_113 { s_38_115 } else { s_38_117 };
        // D s_38_119: cast trunc s_38_118 -> u8
        let s_38_119: bool = ((s_38_118) != 0);
        // C s_38_120: const #16999u : u32
        let s_38_120: u32 = 16999;
        // D s_38_121: read-reg s_38_120:u8
        let s_38_121: bool = {
            let value = state.read_register::<bool>(s_38_120 as isize);
            tracer.read_register(s_38_120 as isize, value);
            value
        };
        // D s_38_122: cast zx s_38_119 -> bv
        let s_38_122: Bits = Bits::new(s_38_119 as u128, 1u16);
        // D s_38_123: cast zx s_38_121 -> bv
        let s_38_123: Bits = Bits::new(s_38_121 as u128, 1u16);
        // D s_38_124: cmp-eq s_38_122 s_38_123
        let s_38_124: bool = ((s_38_122) == (s_38_123));
        // N s_38_125: assert s_38_124
        let s_38_125: () = assert!(s_38_124);
        // C s_38_126: const #16983u : u32
        let s_38_126: u32 = 16983;
        // D s_38_127: read-reg s_38_126:u8
        let s_38_127: u8 = {
            let value = state.read_register::<u8>(s_38_126 as isize);
            tracer.read_register(s_38_126 as isize, value);
            value
        };
        // C s_38_128: const #0s : i
        let s_38_128: i128 = 0;
        // D s_38_129: cast zx s_38_127 -> bv
        let s_38_129: Bits = Bits::new(s_38_127 as u128, 5u16);
        // D s_38_130: read-var spsr:bv
        let s_38_130: Bits = fn_state.spsr;
        // C s_38_131: const #4s : i
        let s_38_131: i128 = 4;
        // C s_38_132: const #1u : u64
        let s_38_132: u64 = 1;
        // C s_38_133: cast zx s_38_132 -> bv
        let s_38_133: Bits = Bits::new(s_38_132 as u128, 64u16);
        // C s_38_134: lsl s_38_133 s_38_131
        let s_38_134: Bits = s_38_133 << s_38_131;
        // C s_38_135: sub s_38_134 s_38_133
        let s_38_135: Bits = ((s_38_134) - (s_38_133));
        // D s_38_136: and s_38_129 s_38_135
        let s_38_136: Bits = ((s_38_129) & (s_38_135));
        // D s_38_137: lsl s_38_136 s_38_128
        let s_38_137: Bits = s_38_136 << s_38_128;
        // C s_38_138: lsl s_38_135 s_38_128
        let s_38_138: Bits = s_38_135 << s_38_128;
        // C s_38_139: cmpl s_38_138
        let s_38_139: Bits = !s_38_138;
        // D s_38_140: and s_38_130 s_38_139
        let s_38_140: Bits = ((s_38_130) & (s_38_139));
        // D s_38_141: or s_38_140 s_38_137
        let s_38_141: Bits = ((s_38_140) | (s_38_137));
        // D s_38_142: write-var spsr <= s_38_141
        fn_state.spsr = s_38_141;
        // N s_38_143: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_39_0: const #16991u : u32
        let s_39_0: u32 = 16991;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: bool = {
            let value = state.read_register::<bool>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call Bit(s_39_1)
        let s_39_2: bool = Bit(state, tracer, s_39_1);
        // C s_39_3: const #21s : i
        let s_39_3: i128 = 21;
        // D s_39_4: read-var spsr:bv
        let s_39_4: Bits = fn_state.spsr;
        // C s_39_5: const #1u : u64
        let s_39_5: u64 = 1;
        // D s_39_6: bit-insert s_39_4 s_39_4 s_39_3 s_39_5
        let s_39_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_39_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_39_4.length(),
            );
            (s_39_4 & mask) | (s_39_4 << s_39_3)
        };
        // D s_39_7: write-var spsr <= s_39_6
        fn_state.spsr = s_39_6;
        // N s_39_8: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#4419 <= s_40_0
        fn_state.gs_4419 = s_40_0;
        // N s_40_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_41_0: read-var targetELState:u32
        let s_41_0: u32 = fn_state.targetELState;
        // C s_41_1: const #0u : u32
        let s_41_1: u32 = 0;
        // D s_41_2: cmp-eq s_41_0 s_41_1
        let s_41_2: bool = ((s_41_0) == (s_41_1));
        // N s_41_3: branch s_41_2 b43 b42
        if s_41_2 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_42_0: const #16973u : u32
        let s_42_0: u32 = 16973;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: bool = {
            let value = state.read_register::<bool>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call Bit(s_42_1)
        let s_42_2: bool = Bit(state, tracer, s_42_1);
        // C s_42_3: const #24s : i
        let s_42_3: i128 = 24;
        // D s_42_4: read-var spsr:bv
        let s_42_4: Bits = fn_state.spsr;
        // C s_42_5: const #1u : u64
        let s_42_5: u64 = 1;
        // D s_42_6: bit-insert s_42_4 s_42_4 s_42_3 s_42_5
        let s_42_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_42_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_42_4.length(),
            );
            (s_42_4 & mask) | (s_42_4 << s_42_3)
        };
        // D s_42_7: write-var spsr <= s_42_6
        fn_state.spsr = s_42_6;
        // N s_42_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_43_0: const #16973u : u32
        let s_43_0: u32 = 16973;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: bool = {
            let value = state.read_register::<bool>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call Bit(s_43_1)
        let s_43_2: bool = Bit(state, tracer, s_43_1);
        // C s_43_3: const #21s : i
        let s_43_3: i128 = 21;
        // D s_43_4: read-var spsr:bv
        let s_43_4: Bits = fn_state.spsr;
        // C s_43_5: const #1u : u64
        let s_43_5: u64 = 1;
        // D s_43_6: bit-insert s_43_4 s_43_4 s_43_3 s_43_5
        let s_43_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_43_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_43_4.length(),
            );
            (s_43_4 & mask) | (s_43_4 << s_43_3)
        };
        // D s_43_7: write-var spsr <= s_43_6
        fn_state.spsr = s_43_6;
        // N s_43_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_44_0: const #16992u : u32
        let s_44_0: u32 = 16992;
        // D s_44_1: read-reg s_44_0:u8
        let s_44_1: bool = {
            let value = state.read_register::<bool>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call Bit(s_44_1)
        let s_44_2: bool = Bit(state, tracer, s_44_1);
        // C s_44_3: const #23s : i
        let s_44_3: i128 = 23;
        // D s_44_4: read-var spsr:bv
        let s_44_4: Bits = fn_state.spsr;
        // C s_44_5: const #1u : u64
        let s_44_5: u64 = 1;
        // D s_44_6: bit-insert s_44_4 s_44_4 s_44_3 s_44_5
        let s_44_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_44_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_44_4.length(),
            );
            (s_44_4 & mask) | (s_44_4 << s_44_3)
        };
        // D s_44_7: write-var spsr <= s_44_6
        fn_state.spsr = s_44_6;
        // N s_44_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_45_0: const #16985u : u32
        let s_45_0: u32 = 16985;
        // D s_45_1: read-reg s_45_0:u8
        let s_45_1: bool = {
            let value = state.read_register::<bool>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call Bit(s_45_1)
        let s_45_2: bool = Bit(state, tracer, s_45_1);
        // C s_45_3: const #22s : i
        let s_45_3: i128 = 22;
        // D s_45_4: read-var spsr:bv
        let s_45_4: Bits = fn_state.spsr;
        // C s_45_5: const #1u : u64
        let s_45_5: u64 = 1;
        // D s_45_6: bit-insert s_45_4 s_45_4 s_45_3 s_45_5
        let s_45_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_45_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_45_4.length(),
            );
            (s_45_4 & mask) | (s_45_4 << s_45_3)
        };
        // D s_45_7: write-var spsr <= s_45_6
        fn_state.spsr = s_45_6;
        // N s_45_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_46_0: const #32s : i
        let s_46_0: i128 = 32;
        // D s_46_1: read-var Nshadow#54:i64
        let s_46_1: i64 = fn_state.Nshadow_54;
        // D s_46_2: cast zx s_46_1 -> i
        let s_46_2: i128 = (i128::try_from(s_46_1).unwrap());
        // D s_46_3: cmp-eq s_46_2 s_46_0
        let s_46_3: bool = ((s_46_2) == (s_46_0));
        // N s_46_4: assert s_46_3
        let s_46_4: () = assert!(s_46_3);
        // N s_46_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_47_0: read-var targetELState:u32
        let s_47_0: u32 = fn_state.targetELState;
        // C s_47_1: const #0u : u32
        let s_47_1: u32 = 0;
        // D s_47_2: cmp-eq s_47_0 s_47_1
        let s_47_2: bool = ((s_47_0) == (s_47_1));
        // D s_47_3: write-var gs#4378 <= s_47_2
        fn_state.gs_4378 = s_47_2;
        // N s_47_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
