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
use EL2Enabled::*;
use Halted::*;
use TTBCR_read::*;
use u_get_HCR_EL2_Type_VSE::*;
use ExternalDebugInterruptsDisabled::*;
use Zeros::*;
use Mk_VDISR_EL2_Type::*;
use ELUsingAArch32::*;
use Bit::*;
use u_get_TTBCR_Type_EAE::*;
use Mk_VDISR_Type::*;
use VDISR_write::*;
use u_get_HCR_EL2_Type_AMO::*;
use u_get_HCR_EL2_Type_TGE::*;
use VDFSR_read::*;
use common::*;
pub fn AArch64_vESBOperation<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_24415: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_24421: bool,
        gs_24416: bool,
        vSEI_pending: bool,
        gs_24419: bool,
        gs_24420: bool,
        vmasked: bool,
        target: u32,
        gs_24423: bool,
        ga_19030: ProductType700c18a878c5601b,
        gs_24417: bool,
        ga_19033: ProductType700c18a878c5601b,
        gs_24422: bool,
        gs_24415: (),
    }
    let fn_state = FunctionState {
        gs_24415,
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
        // D s_1_7: write-var gs#24416 <= s_1_6
        fn_state.gs_24416 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#24416:u8
        let s_2_0: bool = fn_state.gs_24416;
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
        // D s_3_1: write-var gs#24417 <= s_3_0
        fn_state.gs_24417 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#24417:u8
        let s_4_0: bool = fn_state.gs_24417;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #102552u : u32
        let s_4_2: u32 = 102552;
        // D s_4_3: read-reg s_4_2:struct
        let s_4_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: call _get_HCR_EL2_Type_TGE(s_4_3)
        let s_4_4: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_4_3);
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // C s_4_6: const #0u : u8
        let s_4_6: bool = false;
        // C s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 1u16);
        // D s_4_8: cmp-eq s_4_5 s_4_7
        let s_4_8: bool = ((s_4_5) == (s_4_7));
        // N s_4_9: branch s_4_8 b27 b5
        if s_4_8 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#24419 <= s_5_0
        fn_state.gs_24419 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#24419:u8
        let s_6_0: bool = fn_state.gs_24419;
        // N s_6_1: branch s_6_0 b26 b7
        if s_6_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#24420 <= s_7_0
        fn_state.gs_24420 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#24420:u8
        let s_8_0: bool = fn_state.gs_24420;
        // D s_8_1: write-var vSEI_pending <= s_8_0
        fn_state.vSEI_pending = s_8_0;
        // C s_8_2: const #() : ()
        let s_8_2: () = ();
        // S s_8_3: call Halted(s_8_2)
        let s_8_3: bool = Halted(state, tracer, s_8_2);
        // N s_8_4: branch s_8_3 b25 b9
        if s_8_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #440u : u32
        let s_9_0: u32 = 440;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call ExternalDebugInterruptsDisabled(s_9_1)
        let s_9_2: bool = ExternalDebugInterruptsDisabled(state, tracer, s_9_1);
        // D s_9_3: write-var gs#24421 <= s_9_2
        fn_state.gs_24421 = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#24421:u8
        let s_10_0: bool = fn_state.gs_24421;
        // N s_10_1: branch s_10_0 b24 b11
        if s_10_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #16968u : u32
        let s_11_0: u32 = 16968;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: bool = {
            let value = state.read_register::<bool>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 1u16);
        // C s_11_3: const #1u : u8
        let s_11_3: bool = true;
        // C s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 1u16);
        // D s_11_5: cmp-eq s_11_2 s_11_4
        let s_11_5: bool = ((s_11_2) == (s_11_4));
        // D s_11_6: write-var gs#24422 <= s_11_5
        fn_state.gs_24422 = s_11_5;
        // N s_11_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#24422:u8
        let s_12_0: bool = fn_state.gs_24422;
        // D s_12_1: write-var vmasked <= s_12_0
        fn_state.vmasked = s_12_0;
        // D s_12_2: read-var vSEI_pending:u8
        let s_12_2: bool = fn_state.vSEI_pending;
        // N s_12_3: branch s_12_2 b23 b13
        if s_12_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#24423 <= s_13_0
        fn_state.gs_24423 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#24423:u8
        let s_14_0: bool = fn_state.gs_24423;
        // N s_14_1: branch s_14_0 b16 b15
        if s_14_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #440u : u32
        let s_16_0: u32 = 440;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call ELUsingAArch32(s_16_1)
        let s_16_2: bool = ELUsingAArch32(state, tracer, s_16_1);
        // N s_16_3: branch s_16_2 b19 b17
        if s_16_2 {
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
        // C s_17_0: const #64s : i
        let s_17_0: i128 = 64;
        // S s_17_1: call Zeros(s_17_0)
        let s_17_1: Bits = Zeros(state, tracer, s_17_0);
        // S s_17_2: cast reint s_17_1 -> u64
        let s_17_2: u64 = (s_17_1.value() as u64);
        // C s_17_3: const #1u : u8
        let s_17_3: bool = true;
        // S s_17_4: call Bit(s_17_3)
        let s_17_4: bool = Bit(state, tracer, s_17_3);
        // C s_17_5: const #31s : i
        let s_17_5: i128 = 31;
        // S s_17_6: cast zx s_17_2 -> bv
        let s_17_6: Bits = Bits::new(s_17_2 as u128, 64u16);
        // C s_17_7: const #1u : u64
        let s_17_7: u64 = 1;
        // D s_17_8: bit-insert s_17_6 s_17_6 s_17_5 s_17_7
        let s_17_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_17_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_17_6.length(),
            );
            (s_17_6 & mask) | (s_17_6 << s_17_5)
        };
        // D s_17_9: cast reint s_17_8 -> u64
        let s_17_9: u64 = (s_17_8.value() as u64);
        // C s_17_10: const #100824u : u32
        let s_17_10: u32 = 100824;
        // D s_17_11: read-reg s_17_10:u64
        let s_17_11: u64 = {
            let value = state.read_register::<u64>(s_17_10 as isize);
            tracer.read_register(s_17_10 as isize, value);
            value
        };
        // C s_17_12: const #0s : i
        let s_17_12: i128 = 0;
        // D s_17_13: cast zx s_17_11 -> bv
        let s_17_13: Bits = Bits::new(s_17_11 as u128, 64u16);
        // C s_17_14: const #1s : i64
        let s_17_14: i64 = 1;
        // C s_17_15: cast zx s_17_14 -> i
        let s_17_15: i128 = (i128::try_from(s_17_14).unwrap());
        // C s_17_16: const #24s : i
        let s_17_16: i128 = 24;
        // C s_17_17: add s_17_16 s_17_15
        let s_17_17: i128 = (s_17_16 + s_17_15);
        // D s_17_18: bit-extract s_17_13 s_17_12 s_17_17
        let s_17_18: Bits = (Bits::new(
            ((s_17_13) >> (s_17_12)).value(),
            u16::try_from(s_17_17).unwrap(),
        ));
        // D s_17_19: cast reint s_17_18 -> u25
        let s_17_19: u32 = (s_17_18.value() as u32);
        // C s_17_20: const #0s : i
        let s_17_20: i128 = 0;
        // D s_17_21: cast zx s_17_9 -> bv
        let s_17_21: Bits = Bits::new(s_17_9 as u128, 64u16);
        // D s_17_22: cast zx s_17_19 -> bv
        let s_17_22: Bits = Bits::new(s_17_19 as u128, 25u16);
        // C s_17_23: const #24s : i
        let s_17_23: i128 = 24;
        // C s_17_24: const #1u : u64
        let s_17_24: u64 = 1;
        // C s_17_25: cast zx s_17_24 -> bv
        let s_17_25: Bits = Bits::new(s_17_24 as u128, 64u16);
        // C s_17_26: lsl s_17_25 s_17_23
        let s_17_26: Bits = s_17_25 << s_17_23;
        // C s_17_27: sub s_17_26 s_17_25
        let s_17_27: Bits = ((s_17_26) - (s_17_25));
        // D s_17_28: and s_17_22 s_17_27
        let s_17_28: Bits = ((s_17_22) & (s_17_27));
        // D s_17_29: lsl s_17_28 s_17_20
        let s_17_29: Bits = s_17_28 << s_17_20;
        // C s_17_30: lsl s_17_27 s_17_20
        let s_17_30: Bits = s_17_27 << s_17_20;
        // C s_17_31: cmpl s_17_30
        let s_17_31: Bits = !s_17_30;
        // D s_17_32: and s_17_21 s_17_31
        let s_17_32: Bits = ((s_17_21) & (s_17_31));
        // D s_17_33: or s_17_32 s_17_29
        let s_17_33: Bits = ((s_17_32) | (s_17_29));
        // D s_17_34: cast reint s_17_33 -> u64
        let s_17_34: u64 = (s_17_33.value() as u64);
        // D s_17_35: call Mk_VDISR_EL2_Type(s_17_34)
        let s_17_35: ProductType5c790c8ef59cc8b2 = Mk_VDISR_EL2_Type(
            state,
            tracer,
            s_17_34,
        );
        // C s_17_36: const #19328u : u32
        let s_17_36: u32 = 19328;
        // N s_17_37: write-reg s_17_36 <= s_17_35
        let s_17_37: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_17_36 as isize, s_17_35);
            tracer.write_register(s_17_36 as isize, s_17_35);
        };
        // N s_17_38: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #102552u : u32
        let s_18_0: u32 = 102552;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // C s_18_2: const #102552u : u32
        let s_18_2: u32 = 102552;
        // N s_18_3: write-reg s_18_2 <= s_18_1
        let s_18_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_18_2 as isize, s_18_1);
            tracer.write_register(s_18_2 as isize, s_18_1);
        };
        // N s_18_4: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #32s : i
        let s_19_0: i128 = 32;
        // S s_19_1: call Zeros(s_19_0)
        let s_19_1: Bits = Zeros(state, tracer, s_19_0);
        // S s_19_2: cast reint s_19_1 -> u32
        let s_19_2: u32 = (s_19_1.value() as u32);
        // D s_19_3: write-var target <= s_19_2
        fn_state.target = s_19_2;
        // C s_19_4: const #1u : u8
        let s_19_4: bool = true;
        // S s_19_5: call Bit(s_19_4)
        let s_19_5: bool = Bit(state, tracer, s_19_4);
        // C s_19_6: const #31s : i
        let s_19_6: i128 = 31;
        // D s_19_7: read-var target:u32
        let s_19_7: u32 = fn_state.target;
        // D s_19_8: cast zx s_19_7 -> bv
        let s_19_8: Bits = Bits::new(s_19_7 as u128, 32u16);
        // C s_19_9: const #1u : u64
        let s_19_9: u64 = 1;
        // D s_19_10: bit-insert s_19_8 s_19_8 s_19_6 s_19_9
        let s_19_10: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_19_9 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_19_8.length(),
            );
            (s_19_8 & mask) | (s_19_8 << s_19_6)
        };
        // D s_19_11: cast reint s_19_10 -> u32
        let s_19_11: u32 = (s_19_10.value() as u32);
        // D s_19_12: write-var target <= s_19_11
        fn_state.target = s_19_11;
        // C s_19_13: const #() : ()
        let s_19_13: () = ();
        // S s_19_14: call VDFSR_read(s_19_13)
        let s_19_14: ProductType700c18a878c5601b = VDFSR_read(state, tracer, s_19_13);
        // D s_19_15: write-var ga#19030 <= s_19_14
        fn_state.ga_19030 = s_19_14;
        // D s_19_16: read-var ga#19030.0:struct
        let s_19_16: u32 = fn_state.ga_19030._0;
        // C s_19_17: const #14s : i
        let s_19_17: i128 = 14;
        // D s_19_18: cast zx s_19_16 -> bv
        let s_19_18: Bits = Bits::new(s_19_16 as u128, 32u16);
        // C s_19_19: const #1s : i64
        let s_19_19: i64 = 1;
        // C s_19_20: cast zx s_19_19 -> i
        let s_19_20: i128 = (i128::try_from(s_19_19).unwrap());
        // C s_19_21: const #1s : i
        let s_19_21: i128 = 1;
        // C s_19_22: add s_19_21 s_19_20
        let s_19_22: i128 = (s_19_21 + s_19_20);
        // D s_19_23: bit-extract s_19_18 s_19_17 s_19_22
        let s_19_23: Bits = (Bits::new(
            ((s_19_18) >> (s_19_17)).value(),
            u16::try_from(s_19_22).unwrap(),
        ));
        // D s_19_24: cast reint s_19_23 -> u8
        let s_19_24: u8 = (s_19_23.value() as u8);
        // C s_19_25: const #14s : i
        let s_19_25: i128 = 14;
        // D s_19_26: read-var target:u32
        let s_19_26: u32 = fn_state.target;
        // D s_19_27: cast zx s_19_26 -> bv
        let s_19_27: Bits = Bits::new(s_19_26 as u128, 32u16);
        // D s_19_28: cast zx s_19_24 -> bv
        let s_19_28: Bits = Bits::new(s_19_24 as u128, 2u16);
        // C s_19_29: const #1s : i
        let s_19_29: i128 = 1;
        // C s_19_30: const #1u : u64
        let s_19_30: u64 = 1;
        // C s_19_31: cast zx s_19_30 -> bv
        let s_19_31: Bits = Bits::new(s_19_30 as u128, 64u16);
        // C s_19_32: lsl s_19_31 s_19_29
        let s_19_32: Bits = s_19_31 << s_19_29;
        // C s_19_33: sub s_19_32 s_19_31
        let s_19_33: Bits = ((s_19_32) - (s_19_31));
        // D s_19_34: and s_19_28 s_19_33
        let s_19_34: Bits = ((s_19_28) & (s_19_33));
        // D s_19_35: lsl s_19_34 s_19_25
        let s_19_35: Bits = s_19_34 << s_19_25;
        // C s_19_36: lsl s_19_33 s_19_25
        let s_19_36: Bits = s_19_33 << s_19_25;
        // C s_19_37: cmpl s_19_36
        let s_19_37: Bits = !s_19_36;
        // D s_19_38: and s_19_27 s_19_37
        let s_19_38: Bits = ((s_19_27) & (s_19_37));
        // D s_19_39: or s_19_38 s_19_35
        let s_19_39: Bits = ((s_19_38) | (s_19_35));
        // D s_19_40: cast reint s_19_39 -> u32
        let s_19_40: u32 = (s_19_39.value() as u32);
        // D s_19_41: write-var target <= s_19_40
        fn_state.target = s_19_40;
        // C s_19_42: const #() : ()
        let s_19_42: () = ();
        // S s_19_43: call VDFSR_read(s_19_42)
        let s_19_43: ProductType700c18a878c5601b = VDFSR_read(state, tracer, s_19_42);
        // D s_19_44: write-var ga#19033 <= s_19_43
        fn_state.ga_19033 = s_19_43;
        // D s_19_45: read-var ga#19033.0:struct
        let s_19_45: u32 = fn_state.ga_19033._0;
        // C s_19_46: const #12s : i
        let s_19_46: i128 = 12;
        // D s_19_47: cast zx s_19_45 -> bv
        let s_19_47: Bits = Bits::new(s_19_45 as u128, 32u16);
        // C s_19_48: const #1u : u64
        let s_19_48: u64 = 1;
        // D s_19_49: bit-extract s_19_47 s_19_46 s_19_48
        let s_19_49: Bits = (Bits::new(
            ((s_19_47) >> (s_19_46)).value(),
            u16::try_from(s_19_48).unwrap(),
        ));
        // D s_19_50: cast reint s_19_49 -> u8
        let s_19_50: bool = ((s_19_49.value()) != 0);
        // C s_19_51: const #0s : i
        let s_19_51: i128 = 0;
        // C s_19_52: const #0u : u64
        let s_19_52: u64 = 0;
        // D s_19_53: cast zx s_19_50 -> u64
        let s_19_53: u64 = (s_19_50 as u64);
        // C s_19_54: const #1u : u64
        let s_19_54: u64 = 1;
        // D s_19_55: and s_19_53 s_19_54
        let s_19_55: u64 = ((s_19_53) & (s_19_54));
        // D s_19_56: cmp-eq s_19_55 s_19_54
        let s_19_56: bool = ((s_19_55) == (s_19_54));
        // D s_19_57: lsl s_19_53 s_19_51
        let s_19_57: u64 = s_19_53 << s_19_51;
        // D s_19_58: or s_19_52 s_19_57
        let s_19_58: u64 = ((s_19_52) | (s_19_57));
        // D s_19_59: cmpl s_19_57
        let s_19_59: u64 = !s_19_57;
        // D s_19_60: and s_19_52 s_19_59
        let s_19_60: u64 = ((s_19_52) & (s_19_59));
        // D s_19_61: select s_19_56 s_19_58 s_19_60
        let s_19_61: u64 = if s_19_56 { s_19_58 } else { s_19_60 };
        // D s_19_62: cast trunc s_19_61 -> u8
        let s_19_62: bool = ((s_19_61) != 0);
        // D s_19_63: call Bit(s_19_62)
        let s_19_63: bool = Bit(state, tracer, s_19_62);
        // C s_19_64: const #12s : i
        let s_19_64: i128 = 12;
        // D s_19_65: read-var target:u32
        let s_19_65: u32 = fn_state.target;
        // D s_19_66: cast zx s_19_65 -> bv
        let s_19_66: Bits = Bits::new(s_19_65 as u128, 32u16);
        // C s_19_67: const #1u : u64
        let s_19_67: u64 = 1;
        // D s_19_68: bit-insert s_19_66 s_19_66 s_19_64 s_19_67
        let s_19_68: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_19_67 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_19_66.length(),
            );
            (s_19_66 & mask) | (s_19_66 << s_19_64)
        };
        // D s_19_69: cast reint s_19_68 -> u32
        let s_19_69: u32 = (s_19_68.value() as u32);
        // D s_19_70: write-var target <= s_19_69
        fn_state.target = s_19_69;
        // C s_19_71: const #() : ()
        let s_19_71: () = ();
        // S s_19_72: call TTBCR_read(s_19_71)
        let s_19_72: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_19_71);
        // S s_19_73: call _get_TTBCR_Type_EAE(s_19_72)
        let s_19_73: bool = u_get_TTBCR_Type_EAE(state, tracer, s_19_72);
        // S s_19_74: call Bit(s_19_73)
        let s_19_74: bool = Bit(state, tracer, s_19_73);
        // C s_19_75: const #9s : i
        let s_19_75: i128 = 9;
        // D s_19_76: read-var target:u32
        let s_19_76: u32 = fn_state.target;
        // D s_19_77: cast zx s_19_76 -> bv
        let s_19_77: Bits = Bits::new(s_19_76 as u128, 32u16);
        // C s_19_78: const #1u : u64
        let s_19_78: u64 = 1;
        // D s_19_79: bit-insert s_19_77 s_19_77 s_19_75 s_19_78
        let s_19_79: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_19_78 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_19_77.length(),
            );
            (s_19_77 & mask) | (s_19_77 << s_19_75)
        };
        // D s_19_80: cast reint s_19_79 -> u32
        let s_19_80: u32 = (s_19_79.value() as u32);
        // D s_19_81: write-var target <= s_19_80
        fn_state.target = s_19_80;
        // C s_19_82: const #() : ()
        let s_19_82: () = ();
        // S s_19_83: call TTBCR_read(s_19_82)
        let s_19_83: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_19_82);
        // S s_19_84: call _get_TTBCR_Type_EAE(s_19_83)
        let s_19_84: bool = u_get_TTBCR_Type_EAE(state, tracer, s_19_83);
        // S s_19_85: cast zx s_19_84 -> bv
        let s_19_85: Bits = Bits::new(s_19_84 as u128, 1u16);
        // C s_19_86: const #1u : u8
        let s_19_86: bool = true;
        // C s_19_87: cast zx s_19_86 -> bv
        let s_19_87: Bits = Bits::new(s_19_86 as u128, 1u16);
        // S s_19_88: cmp-eq s_19_85 s_19_87
        let s_19_88: bool = ((s_19_85) == (s_19_87));
        // N s_19_89: branch s_19_88 b22 b20
        if s_19_88 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #22u : u8
        let s_20_0: u8 = 22;
        // C s_20_1: const #4s : i
        let s_20_1: i128 = 4;
        // C s_20_2: cast zx s_20_0 -> bv
        let s_20_2: Bits = Bits::new(s_20_0 as u128, 5u16);
        // C s_20_3: const #1s : i64
        let s_20_3: i64 = 1;
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_5: const #0s : i
        let s_20_5: i128 = 0;
        // C s_20_6: add s_20_5 s_20_4
        let s_20_6: i128 = (s_20_5 + s_20_4);
        // D s_20_7: bit-extract s_20_2 s_20_1 s_20_6
        let s_20_7: Bits = (Bits::new(
            ((s_20_2) >> (s_20_1)).value(),
            u16::try_from(s_20_6).unwrap(),
        ));
        // D s_20_8: cast reint s_20_7 -> u8
        let s_20_8: bool = ((s_20_7.value()) != 0);
        // C s_20_9: const #10s : i
        let s_20_9: i128 = 10;
        // D s_20_10: read-var target:u32
        let s_20_10: u32 = fn_state.target;
        // D s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 32u16);
        // D s_20_12: cast zx s_20_8 -> bv
        let s_20_12: Bits = Bits::new(s_20_8 as u128, 1u16);
        // C s_20_13: const #0s : i
        let s_20_13: i128 = 0;
        // C s_20_14: const #1u : u64
        let s_20_14: u64 = 1;
        // C s_20_15: cast zx s_20_14 -> bv
        let s_20_15: Bits = Bits::new(s_20_14 as u128, 64u16);
        // C s_20_16: lsl s_20_15 s_20_13
        let s_20_16: Bits = s_20_15 << s_20_13;
        // C s_20_17: sub s_20_16 s_20_15
        let s_20_17: Bits = ((s_20_16) - (s_20_15));
        // D s_20_18: and s_20_12 s_20_17
        let s_20_18: Bits = ((s_20_12) & (s_20_17));
        // D s_20_19: lsl s_20_18 s_20_9
        let s_20_19: Bits = s_20_18 << s_20_9;
        // C s_20_20: lsl s_20_17 s_20_9
        let s_20_20: Bits = s_20_17 << s_20_9;
        // C s_20_21: cmpl s_20_20
        let s_20_21: Bits = !s_20_20;
        // D s_20_22: and s_20_11 s_20_21
        let s_20_22: Bits = ((s_20_11) & (s_20_21));
        // D s_20_23: or s_20_22 s_20_19
        let s_20_23: Bits = ((s_20_22) | (s_20_19));
        // D s_20_24: cast reint s_20_23 -> u32
        let s_20_24: u32 = (s_20_23.value() as u32);
        // D s_20_25: write-var target <= s_20_24
        fn_state.target = s_20_24;
        // C s_20_26: const #0s : i
        let s_20_26: i128 = 0;
        // C s_20_27: cast zx s_20_0 -> bv
        let s_20_27: Bits = Bits::new(s_20_0 as u128, 5u16);
        // C s_20_28: const #1s : i64
        let s_20_28: i64 = 1;
        // C s_20_29: cast zx s_20_28 -> i
        let s_20_29: i128 = (i128::try_from(s_20_28).unwrap());
        // C s_20_30: const #3s : i
        let s_20_30: i128 = 3;
        // C s_20_31: add s_20_30 s_20_29
        let s_20_31: i128 = (s_20_30 + s_20_29);
        // D s_20_32: bit-extract s_20_27 s_20_26 s_20_31
        let s_20_32: Bits = (Bits::new(
            ((s_20_27) >> (s_20_26)).value(),
            u16::try_from(s_20_31).unwrap(),
        ));
        // D s_20_33: cast reint s_20_32 -> u8
        let s_20_33: u8 = (s_20_32.value() as u8);
        // C s_20_34: const #0s : i
        let s_20_34: i128 = 0;
        // D s_20_35: read-var target:u32
        let s_20_35: u32 = fn_state.target;
        // D s_20_36: cast zx s_20_35 -> bv
        let s_20_36: Bits = Bits::new(s_20_35 as u128, 32u16);
        // D s_20_37: cast zx s_20_33 -> bv
        let s_20_37: Bits = Bits::new(s_20_33 as u128, 4u16);
        // C s_20_38: const #3s : i
        let s_20_38: i128 = 3;
        // C s_20_39: const #1u : u64
        let s_20_39: u64 = 1;
        // C s_20_40: cast zx s_20_39 -> bv
        let s_20_40: Bits = Bits::new(s_20_39 as u128, 64u16);
        // C s_20_41: lsl s_20_40 s_20_38
        let s_20_41: Bits = s_20_40 << s_20_38;
        // C s_20_42: sub s_20_41 s_20_40
        let s_20_42: Bits = ((s_20_41) - (s_20_40));
        // D s_20_43: and s_20_37 s_20_42
        let s_20_43: Bits = ((s_20_37) & (s_20_42));
        // D s_20_44: lsl s_20_43 s_20_34
        let s_20_44: Bits = s_20_43 << s_20_34;
        // C s_20_45: lsl s_20_42 s_20_34
        let s_20_45: Bits = s_20_42 << s_20_34;
        // C s_20_46: cmpl s_20_45
        let s_20_46: Bits = !s_20_45;
        // D s_20_47: and s_20_36 s_20_46
        let s_20_47: Bits = ((s_20_36) & (s_20_46));
        // D s_20_48: or s_20_47 s_20_44
        let s_20_48: Bits = ((s_20_47) | (s_20_44));
        // D s_20_49: cast reint s_20_48 -> u32
        let s_20_49: u32 = (s_20_48.value() as u32);
        // D s_20_50: write-var target <= s_20_49
        fn_state.target = s_20_49;
        // N s_20_51: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var target:u32
        let s_21_0: u32 = fn_state.target;
        // D s_21_1: call Mk_VDISR_Type(s_21_0)
        let s_21_1: ProductType700c18a878c5601b = Mk_VDISR_Type(state, tracer, s_21_0);
        // D s_21_2: call VDISR_write(s_21_1)
        let s_21_2: () = VDISR_write(state, tracer, s_21_1);
        // N s_21_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0s : i
        let s_22_0: i128 = 0;
        // D s_22_1: read-var target:u32
        let s_22_1: u32 = fn_state.target;
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 32u16);
        // C s_22_3: const #17u : u8
        let s_22_3: u8 = 17;
        // C s_22_4: cast zx s_22_3 -> bv
        let s_22_4: Bits = Bits::new(s_22_3 as u128, 6u16);
        // C s_22_5: const #5s : i
        let s_22_5: i128 = 5;
        // C s_22_6: const #1u : u64
        let s_22_6: u64 = 1;
        // C s_22_7: cast zx s_22_6 -> bv
        let s_22_7: Bits = Bits::new(s_22_6 as u128, 64u16);
        // C s_22_8: lsl s_22_7 s_22_5
        let s_22_8: Bits = s_22_7 << s_22_5;
        // C s_22_9: sub s_22_8 s_22_7
        let s_22_9: Bits = ((s_22_8) - (s_22_7));
        // C s_22_10: and s_22_4 s_22_9
        let s_22_10: Bits = ((s_22_4) & (s_22_9));
        // C s_22_11: lsl s_22_10 s_22_0
        let s_22_11: Bits = s_22_10 << s_22_0;
        // C s_22_12: lsl s_22_9 s_22_0
        let s_22_12: Bits = s_22_9 << s_22_0;
        // C s_22_13: cmpl s_22_12
        let s_22_13: Bits = !s_22_12;
        // D s_22_14: and s_22_2 s_22_13
        let s_22_14: Bits = ((s_22_2) & (s_22_13));
        // D s_22_15: or s_22_14 s_22_11
        let s_22_15: Bits = ((s_22_14) | (s_22_11));
        // D s_22_16: cast reint s_22_15 -> u32
        let s_22_16: u32 = (s_22_15.value() as u32);
        // D s_22_17: write-var target <= s_22_16
        fn_state.target = s_22_16;
        // N s_22_18: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var vmasked:u8
        let s_23_0: bool = fn_state.vmasked;
        // D s_23_1: write-var gs#24423 <= s_23_0
        fn_state.gs_24423 = s_23_0;
        // N s_23_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#24422 <= s_24_0
        fn_state.gs_24422 = s_24_0;
        // N s_24_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#24421 <= s_25_0
        fn_state.gs_24421 = s_25_0;
        // N s_25_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #102552u : u32
        let s_26_0: u32 = 102552;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_HCR_EL2_Type_VSE(s_26_1)
        let s_26_2: bool = u_get_HCR_EL2_Type_VSE(state, tracer, s_26_1);
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // C s_26_4: const #1u : u8
        let s_26_4: bool = true;
        // C s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 1u16);
        // D s_26_6: cmp-eq s_26_3 s_26_5
        let s_26_6: bool = ((s_26_3) == (s_26_5));
        // D s_26_7: write-var gs#24420 <= s_26_6
        fn_state.gs_24420 = s_26_6;
        // N s_26_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #102552u : u32
        let s_27_0: u32 = 102552;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_HCR_EL2_Type_AMO(s_27_1)
        let s_27_2: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_27_1);
        // D s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // C s_27_4: const #1u : u8
        let s_27_4: bool = true;
        // C s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 1u16);
        // D s_27_6: cmp-eq s_27_3 s_27_5
        let s_27_6: bool = ((s_27_3) == (s_27_5));
        // D s_27_7: write-var gs#24419 <= s_27_6
        fn_state.gs_24419 = s_27_6;
        // N s_27_8: jump b6
        return block_6(state, tracer, fn_state);
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
        // D s_28_2: write-var gs#24417 <= s_28_1
        fn_state.gs_24417 = s_28_1;
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
        // D s_29_1: write-var gs#24416 <= s_29_0
        fn_state.gs_24416 = s_29_0;
        // N s_29_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
