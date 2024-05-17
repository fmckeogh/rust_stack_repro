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
use u_get_HCR_Type_AMO::*;
use u__IMPDEF_bit::*;
use TTBCR_read::*;
use u_get_VDFSR_Type_AET::*;
use Zeros::*;
use ThisInstrAddr::*;
use u__UNKNOWN_bits::*;
use EncodeSDFSC::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_EL2_Type_AMO::*;
use EL2Enabled::*;
use ClearPendingVirtualSError::*;
use VDFSR_read::*;
use DFAR_write::*;
use HCR_read::*;
use Mk_DFSR_Type::*;
use DFSR_write::*;
use AArch64_TakeVirtualSErrorException::*;
use u_get_VSESR_EL2_Type_ExT::*;
use u_get_VDFSR_Type_ExT::*;
use u_get_HCR_Type_TGE::*;
use ELUsingAArch32::*;
use EncodeLDFSC::*;
use Bit::*;
use u_get_TTBCR_Type_EAE::*;
use u_get_VSESR_EL2_Type_AET::*;
use HaveRASExt::*;
use u__UNKNOWN_integer::*;
use AArch32_EnterMode::*;
use common::*;
pub fn AArch32_TakeVirtualSErrorException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31975: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        level: i128,
        fsr: u32,
        gs_31978: bool,
        gs_31977: bool,
        gs_31981: bool,
        preferred_exception_return: u32,
        vect_offset: i64,
        gs_31976: bool,
        gs_31982: bool,
        gs_31975: (),
    }
    let fn_state = FunctionState {
        gs_31975,
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
        // N s_0_7: branch s_0_6 b29 b1
        if s_0_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #440u : u32
        let s_1_3: u32 = 440;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // D s_1_7: write-var gs#31976 <= s_1_6
        fn_state.gs_31976 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31976:u8
        let s_2_0: bool = fn_state.gs_31976;
        // N s_2_1: branch s_2_0 b28 b3
        if s_2_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#31977 <= s_3_0
        fn_state.gs_31977 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#31977:u8
        let s_4_0: bool = fn_state.gs_31977;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #432u : u32
        let s_4_2: u32 = 432;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: call ELUsingAArch32(s_4_3)
        let s_4_4: bool = ELUsingAArch32(state, tracer, s_4_3);
        // N s_4_5: branch s_4_4 b24 b5
        if s_4_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #102552u : u32
        let s_5_0: u32 = 102552;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_HCR_EL2_Type_TGE(s_5_1)
        let s_5_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_5_1);
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // C s_5_4: const #0u : u8
        let s_5_4: bool = false;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // D s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // N s_5_7: branch s_5_6 b23 b6
        if s_5_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#31981 <= s_6_0
        fn_state.gs_31981 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#31981:u8
        let s_7_0: bool = fn_state.gs_31981;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #16975u : u32
        let s_8_0: u32 = 16975;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 2u16);
        // C s_8_3: const #448u : u32
        let s_8_3: u32 = 448;
        // D s_8_4: read-reg s_8_3:u8
        let s_8_4: u8 = {
            let value = state.read_register::<u8>(s_8_3 as isize);
            tracer.read_register(s_8_3 as isize, value);
            value
        };
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 2u16);
        // D s_8_6: cmp-eq s_8_2 s_8_5
        let s_8_6: bool = ((s_8_2) == (s_8_5));
        // N s_8_7: branch s_8_6 b22 b9
        if s_8_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#31978 <= s_9_0
        fn_state.gs_31978 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#31978:u8
        let s_10_0: bool = fn_state.gs_31978;
        // N s_10_1: branch s_10_0 b21 b11
        if s_10_0 {
            return block_21(state, tracer, fn_state);
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
        // C s_12_0: const #32s : i64
        let s_12_0: i64 = 32;
        // C s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // S s_12_2: call ThisInstrAddr(s_12_1)
        let s_12_2: Bits = ThisInstrAddr(state, tracer, s_12_1);
        // S s_12_3: cast reint s_12_2 -> u32
        let s_12_3: u32 = (s_12_2.value() as u32);
        // D s_12_4: write-var preferred_exception_return <= s_12_3
        fn_state.preferred_exception_return = s_12_3;
        // C s_12_5: const #16u : u8
        let s_12_5: u8 = 16;
        // C s_12_6: cast zx s_12_5 -> bv
        let s_12_6: Bits = Bits::new(s_12_5 as u128, 8u16);
        // C s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (s_12_6.value() as i128);
        // C s_12_8: cast reint s_12_7 -> i64
        let s_12_8: i64 = (s_12_7 as i64);
        // D s_12_9: write-var vect_offset <= s_12_8
        fn_state.vect_offset = s_12_8;
        // C s_12_10: const #32s : i64
        let s_12_10: i64 = 32;
        // C s_12_11: cast zx s_12_10 -> i
        let s_12_11: i128 = (i128::try_from(s_12_10).unwrap());
        // S s_12_12: call __UNKNOWN_bits(s_12_11)
        let s_12_12: Bits = u__UNKNOWN_bits(state, tracer, s_12_11);
        // C s_12_13: const #() : ()
        let s_12_13: () = ();
        // S s_12_14: call __UNKNOWN_integer(s_12_13)
        let s_12_14: i128 = u__UNKNOWN_integer(state, tracer, s_12_13);
        // D s_12_15: write-var level <= s_12_14
        fn_state.level = s_12_14;
        // C s_12_16: const #32s : i
        let s_12_16: i128 = 32;
        // S s_12_17: call Zeros(s_12_16)
        let s_12_17: Bits = Zeros(state, tracer, s_12_16);
        // S s_12_18: cast reint s_12_17 -> u32
        let s_12_18: u32 = (s_12_17.value() as u32);
        // D s_12_19: write-var fsr <= s_12_18
        fn_state.fsr = s_12_18;
        // C s_12_20: const #() : ()
        let s_12_20: () = ();
        // S s_12_21: call HaveRASExt(s_12_20)
        let s_12_21: bool = HaveRASExt(state, tracer, s_12_20);
        // N s_12_22: branch s_12_21 b18 b13
        if s_12_21 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #"Virtual External abort type" : str
        let s_13_0: &'static str = "Virtual External abort type";
        // S s_13_1: call __IMPDEF_bit(s_13_0)
        let s_13_1: bool = u__IMPDEF_bit(state, tracer, s_13_0);
        // S s_13_2: call Bit(s_13_1)
        let s_13_2: bool = Bit(state, tracer, s_13_1);
        // C s_13_3: const #12s : i
        let s_13_3: i128 = 12;
        // D s_13_4: read-var fsr:u32
        let s_13_4: u32 = fn_state.fsr;
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 32u16);
        // C s_13_6: const #1u : u64
        let s_13_6: u64 = 1;
        // D s_13_7: bit-insert s_13_5 s_13_5 s_13_3 s_13_6
        let s_13_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_13_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_13_5.length(),
            );
            (s_13_5 & mask) | (s_13_5 << s_13_3)
        };
        // D s_13_8: cast reint s_13_7 -> u32
        let s_13_8: u32 = (s_13_7.value() as u32);
        // D s_13_9: write-var fsr <= s_13_8
        fn_state.fsr = s_13_8;
        // N s_13_10: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call TTBCR_read(s_14_0)
        let s_14_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_14_0);
        // S s_14_2: call _get_TTBCR_Type_EAE(s_14_1)
        let s_14_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_14_1);
        // S s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #1u : u8
        let s_14_4: bool = true;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // S s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // N s_14_7: branch s_14_6 b17 b15
        if s_14_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // S s_15_1: call Bit(s_15_0)
        let s_15_1: bool = Bit(state, tracer, s_15_0);
        // C s_15_2: const #9s : i
        let s_15_2: i128 = 9;
        // D s_15_3: read-var fsr:u32
        let s_15_3: u32 = fn_state.fsr;
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 32u16);
        // C s_15_5: const #1u : u64
        let s_15_5: u64 = 1;
        // D s_15_6: bit-insert s_15_4 s_15_4 s_15_2 s_15_5
        let s_15_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_15_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_15_4.length(),
            );
            (s_15_4 & mask) | (s_15_4 << s_15_2)
        };
        // D s_15_7: cast reint s_15_6 -> u32
        let s_15_7: u32 = (s_15_6.value() as u32);
        // D s_15_8: write-var fsr <= s_15_7
        fn_state.fsr = s_15_7;
        // C s_15_9: const #15u : u32
        let s_15_9: u32 = 15;
        // D s_15_10: read-var level:i
        let s_15_10: i128 = fn_state.level;
        // D s_15_11: call EncodeSDFSC(s_15_9, s_15_10)
        let s_15_11: u8 = EncodeSDFSC(state, tracer, s_15_9, s_15_10);
        // C s_15_12: const #4s : i
        let s_15_12: i128 = 4;
        // D s_15_13: cast zx s_15_11 -> bv
        let s_15_13: Bits = Bits::new(s_15_11 as u128, 5u16);
        // C s_15_14: const #1s : i64
        let s_15_14: i64 = 1;
        // C s_15_15: cast zx s_15_14 -> i
        let s_15_15: i128 = (i128::try_from(s_15_14).unwrap());
        // C s_15_16: const #0s : i
        let s_15_16: i128 = 0;
        // C s_15_17: add s_15_16 s_15_15
        let s_15_17: i128 = (s_15_16 + s_15_15);
        // D s_15_18: bit-extract s_15_13 s_15_12 s_15_17
        let s_15_18: Bits = (Bits::new(
            ((s_15_13) >> (s_15_12)).value(),
            u16::try_from(s_15_17).unwrap(),
        ));
        // D s_15_19: cast reint s_15_18 -> u8
        let s_15_19: bool = ((s_15_18.value()) != 0);
        // C s_15_20: const #10s : i
        let s_15_20: i128 = 10;
        // D s_15_21: read-var fsr:u32
        let s_15_21: u32 = fn_state.fsr;
        // D s_15_22: cast zx s_15_21 -> bv
        let s_15_22: Bits = Bits::new(s_15_21 as u128, 32u16);
        // D s_15_23: cast zx s_15_19 -> bv
        let s_15_23: Bits = Bits::new(s_15_19 as u128, 1u16);
        // C s_15_24: const #0s : i
        let s_15_24: i128 = 0;
        // C s_15_25: const #1u : u64
        let s_15_25: u64 = 1;
        // C s_15_26: cast zx s_15_25 -> bv
        let s_15_26: Bits = Bits::new(s_15_25 as u128, 64u16);
        // C s_15_27: lsl s_15_26 s_15_24
        let s_15_27: Bits = s_15_26 << s_15_24;
        // C s_15_28: sub s_15_27 s_15_26
        let s_15_28: Bits = ((s_15_27) - (s_15_26));
        // D s_15_29: and s_15_23 s_15_28
        let s_15_29: Bits = ((s_15_23) & (s_15_28));
        // D s_15_30: lsl s_15_29 s_15_20
        let s_15_30: Bits = s_15_29 << s_15_20;
        // C s_15_31: lsl s_15_28 s_15_20
        let s_15_31: Bits = s_15_28 << s_15_20;
        // C s_15_32: cmpl s_15_31
        let s_15_32: Bits = !s_15_31;
        // D s_15_33: and s_15_22 s_15_32
        let s_15_33: Bits = ((s_15_22) & (s_15_32));
        // D s_15_34: or s_15_33 s_15_30
        let s_15_34: Bits = ((s_15_33) | (s_15_30));
        // D s_15_35: cast reint s_15_34 -> u32
        let s_15_35: u32 = (s_15_34.value() as u32);
        // D s_15_36: write-var fsr <= s_15_35
        fn_state.fsr = s_15_35;
        // C s_15_37: const #0s : i
        let s_15_37: i128 = 0;
        // D s_15_38: cast zx s_15_11 -> bv
        let s_15_38: Bits = Bits::new(s_15_11 as u128, 5u16);
        // C s_15_39: const #1s : i64
        let s_15_39: i64 = 1;
        // C s_15_40: cast zx s_15_39 -> i
        let s_15_40: i128 = (i128::try_from(s_15_39).unwrap());
        // C s_15_41: const #3s : i
        let s_15_41: i128 = 3;
        // C s_15_42: add s_15_41 s_15_40
        let s_15_42: i128 = (s_15_41 + s_15_40);
        // D s_15_43: bit-extract s_15_38 s_15_37 s_15_42
        let s_15_43: Bits = (Bits::new(
            ((s_15_38) >> (s_15_37)).value(),
            u16::try_from(s_15_42).unwrap(),
        ));
        // D s_15_44: cast reint s_15_43 -> u8
        let s_15_44: u8 = (s_15_43.value() as u8);
        // C s_15_45: const #0s : i
        let s_15_45: i128 = 0;
        // D s_15_46: read-var fsr:u32
        let s_15_46: u32 = fn_state.fsr;
        // D s_15_47: cast zx s_15_46 -> bv
        let s_15_47: Bits = Bits::new(s_15_46 as u128, 32u16);
        // D s_15_48: cast zx s_15_44 -> bv
        let s_15_48: Bits = Bits::new(s_15_44 as u128, 4u16);
        // C s_15_49: const #3s : i
        let s_15_49: i128 = 3;
        // C s_15_50: const #1u : u64
        let s_15_50: u64 = 1;
        // C s_15_51: cast zx s_15_50 -> bv
        let s_15_51: Bits = Bits::new(s_15_50 as u128, 64u16);
        // C s_15_52: lsl s_15_51 s_15_49
        let s_15_52: Bits = s_15_51 << s_15_49;
        // C s_15_53: sub s_15_52 s_15_51
        let s_15_53: Bits = ((s_15_52) - (s_15_51));
        // D s_15_54: and s_15_48 s_15_53
        let s_15_54: Bits = ((s_15_48) & (s_15_53));
        // D s_15_55: lsl s_15_54 s_15_45
        let s_15_55: Bits = s_15_54 << s_15_45;
        // C s_15_56: lsl s_15_53 s_15_45
        let s_15_56: Bits = s_15_53 << s_15_45;
        // C s_15_57: cmpl s_15_56
        let s_15_57: Bits = !s_15_56;
        // D s_15_58: and s_15_47 s_15_57
        let s_15_58: Bits = ((s_15_47) & (s_15_57));
        // D s_15_59: or s_15_58 s_15_55
        let s_15_59: Bits = ((s_15_58) | (s_15_55));
        // D s_15_60: cast reint s_15_59 -> u32
        let s_15_60: u32 = (s_15_59.value() as u32);
        // D s_15_61: write-var fsr <= s_15_60
        fn_state.fsr = s_15_60;
        // N s_15_62: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var fsr:u32
        let s_16_0: u32 = fn_state.fsr;
        // D s_16_1: call Mk_DFSR_Type(s_16_0)
        let s_16_1: ProductType700c18a878c5601b = Mk_DFSR_Type(state, tracer, s_16_0);
        // D s_16_2: call DFSR_write(s_16_1)
        let s_16_2: () = DFSR_write(state, tracer, s_16_1);
        // C s_16_3: const #32s : i64
        let s_16_3: i64 = 32;
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // S s_16_5: call __UNKNOWN_bits(s_16_4)
        let s_16_5: Bits = u__UNKNOWN_bits(state, tracer, s_16_4);
        // S s_16_6: cast reint s_16_5 -> u32
        let s_16_6: u32 = (s_16_5.value() as u32);
        // S s_16_7: call DFAR_write(s_16_6)
        let s_16_7: () = DFAR_write(state, tracer, s_16_6);
        // C s_16_8: const #() : ()
        let s_16_8: () = ();
        // S s_16_9: call ClearPendingVirtualSError(s_16_8)
        let s_16_9: () = ClearPendingVirtualSError(state, tracer, s_16_8);
        // C s_16_10: const #8s : i64
        let s_16_10: i64 = 8;
        // C s_16_11: cast zx s_16_10 -> i
        let s_16_11: i128 = (i128::try_from(s_16_10).unwrap());
        // D s_16_12: read-var vect_offset:i64
        let s_16_12: i64 = fn_state.vect_offset;
        // D s_16_13: cast zx s_16_12 -> i
        let s_16_13: i128 = (i128::try_from(s_16_12).unwrap());
        // C s_16_14: const #392u : u32
        let s_16_14: u32 = 392;
        // D s_16_15: read-reg s_16_14:u8
        let s_16_15: u8 = {
            let value = state.read_register::<u8>(s_16_14 as isize);
            tracer.read_register(s_16_14 as isize, value);
            value
        };
        // D s_16_16: read-var preferred_exception_return:u32
        let s_16_16: u32 = fn_state.preferred_exception_return;
        // D s_16_17: call AArch32_EnterMode(s_16_15, s_16_16, s_16_11, s_16_13)
        let s_16_17: () = AArch32_EnterMode(
            state,
            tracer,
            s_16_15,
            s_16_16,
            s_16_11,
            s_16_13,
        );
        // N s_16_18: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // S s_17_1: call Bit(s_17_0)
        let s_17_1: bool = Bit(state, tracer, s_17_0);
        // C s_17_2: const #9s : i
        let s_17_2: i128 = 9;
        // D s_17_3: read-var fsr:u32
        let s_17_3: u32 = fn_state.fsr;
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 32u16);
        // C s_17_5: const #1u : u64
        let s_17_5: u64 = 1;
        // D s_17_6: bit-insert s_17_4 s_17_4 s_17_2 s_17_5
        let s_17_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_17_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_17_4.length(),
            );
            (s_17_4 & mask) | (s_17_4 << s_17_2)
        };
        // D s_17_7: cast reint s_17_6 -> u32
        let s_17_7: u32 = (s_17_6.value() as u32);
        // D s_17_8: write-var fsr <= s_17_7
        fn_state.fsr = s_17_7;
        // C s_17_9: const #15u : u32
        let s_17_9: u32 = 15;
        // D s_17_10: read-var level:i
        let s_17_10: i128 = fn_state.level;
        // D s_17_11: call EncodeLDFSC(s_17_9, s_17_10)
        let s_17_11: u8 = EncodeLDFSC(state, tracer, s_17_9, s_17_10);
        // C s_17_12: const #0s : i
        let s_17_12: i128 = 0;
        // D s_17_13: read-var fsr:u32
        let s_17_13: u32 = fn_state.fsr;
        // D s_17_14: cast zx s_17_13 -> bv
        let s_17_14: Bits = Bits::new(s_17_13 as u128, 32u16);
        // D s_17_15: cast zx s_17_11 -> bv
        let s_17_15: Bits = Bits::new(s_17_11 as u128, 6u16);
        // C s_17_16: const #5s : i
        let s_17_16: i128 = 5;
        // C s_17_17: const #1u : u64
        let s_17_17: u64 = 1;
        // C s_17_18: cast zx s_17_17 -> bv
        let s_17_18: Bits = Bits::new(s_17_17 as u128, 64u16);
        // C s_17_19: lsl s_17_18 s_17_16
        let s_17_19: Bits = s_17_18 << s_17_16;
        // C s_17_20: sub s_17_19 s_17_18
        let s_17_20: Bits = ((s_17_19) - (s_17_18));
        // D s_17_21: and s_17_15 s_17_20
        let s_17_21: Bits = ((s_17_15) & (s_17_20));
        // D s_17_22: lsl s_17_21 s_17_12
        let s_17_22: Bits = s_17_21 << s_17_12;
        // C s_17_23: lsl s_17_20 s_17_12
        let s_17_23: Bits = s_17_20 << s_17_12;
        // C s_17_24: cmpl s_17_23
        let s_17_24: Bits = !s_17_23;
        // D s_17_25: and s_17_14 s_17_24
        let s_17_25: Bits = ((s_17_14) & (s_17_24));
        // D s_17_26: or s_17_25 s_17_22
        let s_17_26: Bits = ((s_17_25) | (s_17_22));
        // D s_17_27: cast reint s_17_26 -> u32
        let s_17_27: u32 = (s_17_26.value() as u32);
        // D s_17_28: write-var fsr <= s_17_27
        fn_state.fsr = s_17_27;
        // N s_17_29: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #432u : u32
        let s_18_0: u32 = 432;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call ELUsingAArch32(s_18_1)
        let s_18_2: bool = ELUsingAArch32(state, tracer, s_18_1);
        // N s_18_3: branch s_18_2 b20 b19
        if s_18_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #100824u : u32
        let s_19_0: u32 = 100824;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_VSESR_EL2_Type_AET(s_19_1)
        let s_19_2: u8 = u_get_VSESR_EL2_Type_AET(state, tracer, s_19_1);
        // C s_19_3: const #14s : i
        let s_19_3: i128 = 14;
        // D s_19_4: read-var fsr:u32
        let s_19_4: u32 = fn_state.fsr;
        // D s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 32u16);
        // D s_19_6: cast zx s_19_2 -> bv
        let s_19_6: Bits = Bits::new(s_19_2 as u128, 2u16);
        // C s_19_7: const #1s : i
        let s_19_7: i128 = 1;
        // C s_19_8: const #1u : u64
        let s_19_8: u64 = 1;
        // C s_19_9: cast zx s_19_8 -> bv
        let s_19_9: Bits = Bits::new(s_19_8 as u128, 64u16);
        // C s_19_10: lsl s_19_9 s_19_7
        let s_19_10: Bits = s_19_9 << s_19_7;
        // C s_19_11: sub s_19_10 s_19_9
        let s_19_11: Bits = ((s_19_10) - (s_19_9));
        // D s_19_12: and s_19_6 s_19_11
        let s_19_12: Bits = ((s_19_6) & (s_19_11));
        // D s_19_13: lsl s_19_12 s_19_3
        let s_19_13: Bits = s_19_12 << s_19_3;
        // C s_19_14: lsl s_19_11 s_19_3
        let s_19_14: Bits = s_19_11 << s_19_3;
        // C s_19_15: cmpl s_19_14
        let s_19_15: Bits = !s_19_14;
        // D s_19_16: and s_19_5 s_19_15
        let s_19_16: Bits = ((s_19_5) & (s_19_15));
        // D s_19_17: or s_19_16 s_19_13
        let s_19_17: Bits = ((s_19_16) | (s_19_13));
        // D s_19_18: cast reint s_19_17 -> u32
        let s_19_18: u32 = (s_19_17.value() as u32);
        // D s_19_19: write-var fsr <= s_19_18
        fn_state.fsr = s_19_18;
        // C s_19_20: const #100824u : u32
        let s_19_20: u32 = 100824;
        // D s_19_21: read-reg s_19_20:struct
        let s_19_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_20 as isize);
            tracer.read_register(s_19_20 as isize, value);
            value
        };
        // D s_19_22: call _get_VSESR_EL2_Type_ExT(s_19_21)
        let s_19_22: bool = u_get_VSESR_EL2_Type_ExT(state, tracer, s_19_21);
        // D s_19_23: call Bit(s_19_22)
        let s_19_23: bool = Bit(state, tracer, s_19_22);
        // C s_19_24: const #12s : i
        let s_19_24: i128 = 12;
        // D s_19_25: read-var fsr:u32
        let s_19_25: u32 = fn_state.fsr;
        // D s_19_26: cast zx s_19_25 -> bv
        let s_19_26: Bits = Bits::new(s_19_25 as u128, 32u16);
        // C s_19_27: const #1u : u64
        let s_19_27: u64 = 1;
        // D s_19_28: bit-insert s_19_26 s_19_26 s_19_24 s_19_27
        let s_19_28: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_19_27 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_19_26.length(),
            );
            (s_19_26 & mask) | (s_19_26 << s_19_24)
        };
        // D s_19_29: cast reint s_19_28 -> u32
        let s_19_29: u32 = (s_19_28.value() as u32);
        // D s_19_30: write-var fsr <= s_19_29
        fn_state.fsr = s_19_29;
        // N s_19_31: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call VDFSR_read(s_20_0)
        let s_20_1: ProductType700c18a878c5601b = VDFSR_read(state, tracer, s_20_0);
        // S s_20_2: call _get_VDFSR_Type_AET(s_20_1)
        let s_20_2: u8 = u_get_VDFSR_Type_AET(state, tracer, s_20_1);
        // C s_20_3: const #14s : i
        let s_20_3: i128 = 14;
        // D s_20_4: read-var fsr:u32
        let s_20_4: u32 = fn_state.fsr;
        // D s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 32u16);
        // S s_20_6: cast zx s_20_2 -> bv
        let s_20_6: Bits = Bits::new(s_20_2 as u128, 2u16);
        // C s_20_7: const #1s : i
        let s_20_7: i128 = 1;
        // C s_20_8: const #1u : u64
        let s_20_8: u64 = 1;
        // C s_20_9: cast zx s_20_8 -> bv
        let s_20_9: Bits = Bits::new(s_20_8 as u128, 64u16);
        // C s_20_10: lsl s_20_9 s_20_7
        let s_20_10: Bits = s_20_9 << s_20_7;
        // C s_20_11: sub s_20_10 s_20_9
        let s_20_11: Bits = ((s_20_10) - (s_20_9));
        // S s_20_12: and s_20_6 s_20_11
        let s_20_12: Bits = ((s_20_6) & (s_20_11));
        // S s_20_13: lsl s_20_12 s_20_3
        let s_20_13: Bits = s_20_12 << s_20_3;
        // C s_20_14: lsl s_20_11 s_20_3
        let s_20_14: Bits = s_20_11 << s_20_3;
        // C s_20_15: cmpl s_20_14
        let s_20_15: Bits = !s_20_14;
        // D s_20_16: and s_20_5 s_20_15
        let s_20_16: Bits = ((s_20_5) & (s_20_15));
        // D s_20_17: or s_20_16 s_20_13
        let s_20_17: Bits = ((s_20_16) | (s_20_13));
        // D s_20_18: cast reint s_20_17 -> u32
        let s_20_18: u32 = (s_20_17.value() as u32);
        // D s_20_19: write-var fsr <= s_20_18
        fn_state.fsr = s_20_18;
        // C s_20_20: const #() : ()
        let s_20_20: () = ();
        // S s_20_21: call VDFSR_read(s_20_20)
        let s_20_21: ProductType700c18a878c5601b = VDFSR_read(state, tracer, s_20_20);
        // S s_20_22: call _get_VDFSR_Type_ExT(s_20_21)
        let s_20_22: bool = u_get_VDFSR_Type_ExT(state, tracer, s_20_21);
        // S s_20_23: call Bit(s_20_22)
        let s_20_23: bool = Bit(state, tracer, s_20_22);
        // C s_20_24: const #12s : i
        let s_20_24: i128 = 12;
        // D s_20_25: read-var fsr:u32
        let s_20_25: u32 = fn_state.fsr;
        // D s_20_26: cast zx s_20_25 -> bv
        let s_20_26: Bits = Bits::new(s_20_25 as u128, 32u16);
        // C s_20_27: const #1u : u64
        let s_20_27: u64 = 1;
        // D s_20_28: bit-insert s_20_26 s_20_26 s_20_24 s_20_27
        let s_20_28: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_20_27 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_20_26.length(),
            );
            (s_20_26 & mask) | (s_20_26 << s_20_24)
        };
        // D s_20_29: cast reint s_20_28 -> u32
        let s_20_29: u32 = (s_20_28.value() as u32);
        // D s_20_30: write-var fsr <= s_20_29
        fn_state.fsr = s_20_29;
        // N s_20_31: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call AArch64_TakeVirtualSErrorException(s_21_0)
        let s_21_1: () = AArch64_TakeVirtualSErrorException(state, tracer, s_21_0);
        // N s_21_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #440u : u32
        let s_22_0: u32 = 440;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call ELUsingAArch32(s_22_1)
        let s_22_2: bool = ELUsingAArch32(state, tracer, s_22_1);
        // D s_22_3: not s_22_2
        let s_22_3: bool = !s_22_2;
        // D s_22_4: write-var gs#31978 <= s_22_3
        fn_state.gs_31978 = s_22_3;
        // N s_22_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #102552u : u32
        let s_23_0: u32 = 102552;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_HCR_EL2_Type_AMO(s_23_1)
        let s_23_2: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // D s_23_7: write-var gs#31981 <= s_23_6
        fn_state.gs_31981 = s_23_6;
        // N s_23_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HCR_read(s_24_0)
        let s_24_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_24_0);
        // S s_24_2: call _get_HCR_Type_TGE(s_24_1)
        let s_24_2: bool = u_get_HCR_Type_TGE(state, tracer, s_24_1);
        // S s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #0u : u8
        let s_24_4: bool = false;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // S s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // N s_24_7: branch s_24_6 b27 b25
        if s_24_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#31982 <= s_25_0
        fn_state.gs_31982 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#31982:u8
        let s_26_0: bool = fn_state.gs_31982;
        // N s_26_1: assert s_26_0
        let s_26_1: () = assert!(s_26_0);
        // N s_26_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call HCR_read(s_27_0)
        let s_27_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_27_0);
        // S s_27_2: call _get_HCR_Type_AMO(s_27_1)
        let s_27_2: bool = u_get_HCR_Type_AMO(state, tracer, s_27_1);
        // S s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // C s_27_4: const #1u : u8
        let s_27_4: bool = true;
        // C s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 1u16);
        // S s_27_6: cmp-eq s_27_3 s_27_5
        let s_27_6: bool = ((s_27_3) == (s_27_5));
        // D s_27_7: write-var gs#31982 <= s_27_6
        fn_state.gs_31982 = s_27_6;
        // N s_27_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EL2Enabled(s_28_0)
        let s_28_1: bool = EL2Enabled(state, tracer, s_28_0);
        // D s_28_2: write-var gs#31977 <= s_28_1
        fn_state.gs_31977 = s_28_1;
        // N s_28_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#31976 <= s_29_0
        fn_state.gs_31976 = s_29_0;
        // N s_29_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
