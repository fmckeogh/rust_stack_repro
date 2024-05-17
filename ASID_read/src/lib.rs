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
use u_get_TCR_EL2_Type_A1::*;
use u_get_TCR_EL1_Type_A1::*;
use u_get_HCR_EL2_Type_E2H::*;
use TTBCR_read::*;
use u_get_TTBR1_EL2_Type_ASID::*;
use TTBR0_read::*;
use u_get_TTBR0_Type_ASID::*;
use TTBR0_EL1_read::*;
use u_get_TTBR1_Type_ASID::*;
use u_get_CONTEXTIDR_Type_ASID::*;
use TTBR1_read::*;
use u_get_TTBR0_EL1_Type_ASID::*;
use TTBR0_EL2_read::*;
use ELUsingAArch32::*;
use u_get_TTBR1_EL1_Type_ASID::*;
use u_get_TTBR0_EL2_Type_ASID::*;
use u_get_TTBCR_Type_EAE::*;
use TTBR1_EL1_read::*;
use EL2Enabled::*;
use CONTEXTIDR_read::*;
use u_get_TTBCR_Type_A1::*;
use u_get_HCR_EL2_Type_TGE::*;
use common::*;
pub fn ASID_read<T: Tracer>(state: &mut State, tracer: &T, gs_8263: ()) -> u16 {
    #[derive(Default)]
    struct FunctionState {
        return_value: u16,
        gs_8264: bool,
        gs_8265: bool,
        gs_8263: (),
    }
    let fn_state = FunctionState {
        gs_8263,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call EL2Enabled(s_0_0)
        let s_0_1: bool = EL2Enabled(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b19 b1
        if s_0_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#8264 <= s_1_0
        fn_state.gs_8264 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_2_0: read-var gs#8264:u8
        let s_2_0: bool = fn_state.gs_8264;
        // N s_2_1: branch s_2_0 b18 b3
        if s_2_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#8265 <= s_3_0
        fn_state.gs_8265 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_4_0: read-var gs#8265:u8
        let s_4_0: bool = fn_state.gs_8265;
        // N s_4_1: branch s_4_0 b15 b5
        if s_4_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_5_0: const #440u : u32
        let s_5_0: u32 = 440;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call ELUsingAArch32(s_5_1)
        let s_5_2: bool = ELUsingAArch32(state, tracer, s_5_1);
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b12 b6
        if s_5_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call TTBCR_read(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_6_0);
        // S s_6_2: call _get_TTBCR_Type_EAE(s_6_1)
        let s_6_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_6_1);
        // S s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #0u : u8
        let s_6_4: bool = false;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // S s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b11 b7
        if s_6_6 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call TTBCR_read(s_7_0)
        let s_7_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_7_0);
        // S s_7_2: call _get_TTBCR_Type_A1(s_7_1)
        let s_7_2: bool = u_get_TTBCR_Type_A1(state, tracer, s_7_1);
        // S s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // C s_7_4: const #1u : u8
        let s_7_4: bool = true;
        // C s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // S s_7_6: cmp-eq s_7_3 s_7_5
        let s_7_6: bool = ((s_7_3) == (s_7_5));
        // N s_7_7: branch s_7_6 b10 b8
        if s_7_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call TTBR0_read(s_8_0)
        let s_8_1: ProductType5c790c8ef59cc8b2 = TTBR0_read(state, tracer, s_8_0);
        // S s_8_2: call _get_TTBR0_Type_ASID(s_8_1)
        let s_8_2: u8 = u_get_TTBR0_Type_ASID(state, tracer, s_8_1);
        // C s_8_3: const #16s : i
        let s_8_3: i128 = 16;
        // S s_8_4: cast zx s_8_2 -> bv
        let s_8_4: Bits = Bits::new(s_8_2 as u128, 8u16);
        // D s_8_5: bits-cast zx s_8_4 -> bv length s_8_3
        let s_8_5: Bits = s_8_4.zero_extend(s_8_3);
        // D s_8_6: cast reint s_8_5 -> u16
        let s_8_6: u16 = (s_8_5.value() as u16);
        // D s_8_7: write-var return_value <= s_8_6
        fn_state.return_value = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_9_0: read-var return_value:u16
        let s_9_0: u16 = fn_state.return_value;
        // N s_9_1: return s_9_0
        return s_9_0;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call TTBR1_read(s_10_0)
        let s_10_1: ProductType5c790c8ef59cc8b2 = TTBR1_read(state, tracer, s_10_0);
        // S s_10_2: call _get_TTBR1_Type_ASID(s_10_1)
        let s_10_2: u8 = u_get_TTBR1_Type_ASID(state, tracer, s_10_1);
        // C s_10_3: const #16s : i
        let s_10_3: i128 = 16;
        // S s_10_4: cast zx s_10_2 -> bv
        let s_10_4: Bits = Bits::new(s_10_2 as u128, 8u16);
        // D s_10_5: bits-cast zx s_10_4 -> bv length s_10_3
        let s_10_5: Bits = s_10_4.zero_extend(s_10_3);
        // D s_10_6: cast reint s_10_5 -> u16
        let s_10_6: u16 = (s_10_5.value() as u16);
        // D s_10_7: write-var return_value <= s_10_6
        fn_state.return_value = s_10_6;
        // N s_10_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call CONTEXTIDR_read(s_11_0)
        let s_11_1: ProductType700c18a878c5601b = CONTEXTIDR_read(state, tracer, s_11_0);
        // S s_11_2: call _get_CONTEXTIDR_Type_ASID(s_11_1)
        let s_11_2: u8 = u_get_CONTEXTIDR_Type_ASID(state, tracer, s_11_1);
        // C s_11_3: const #16s : i
        let s_11_3: i128 = 16;
        // S s_11_4: cast zx s_11_2 -> bv
        let s_11_4: Bits = Bits::new(s_11_2 as u128, 8u16);
        // D s_11_5: bits-cast zx s_11_4 -> bv length s_11_3
        let s_11_5: Bits = s_11_4.zero_extend(s_11_3);
        // D s_11_6: cast reint s_11_5 -> u16
        let s_11_6: u16 = (s_11_5.value() as u16);
        // D s_11_7: write-var return_value <= s_11_6
        fn_state.return_value = s_11_6;
        // N s_11_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_12_0: const #22392u : u32
        let s_12_0: u32 = 22392;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_TCR_EL1_Type_A1(s_12_1)
        let s_12_2: bool = u_get_TCR_EL1_Type_A1(state, tracer, s_12_1);
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #1u : u8
        let s_12_4: bool = true;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // N s_12_7: branch s_12_6 b14 b13
        if s_12_6 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call TTBR0_EL1_read(s_13_0)
        let s_13_1: ProductType782ac6922b48c20d = TTBR0_EL1_read(state, tracer, s_13_0);
        // S s_13_2: call _get_TTBR0_EL1_Type_ASID(s_13_1)
        let s_13_2: u16 = u_get_TTBR0_EL1_Type_ASID(state, tracer, s_13_1);
        // D s_13_3: write-var return_value <= s_13_2
        fn_state.return_value = s_13_2;
        // N s_13_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call TTBR1_EL1_read(s_14_0)
        let s_14_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_14_0);
        // S s_14_2: call _get_TTBR1_EL1_Type_ASID(s_14_1)
        let s_14_2: u16 = u_get_TTBR1_EL1_Type_ASID(state, tracer, s_14_1);
        // D s_14_3: write-var return_value <= s_14_2
        fn_state.return_value = s_14_2;
        // N s_14_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_15_0: const #12816u : u32
        let s_15_0: u32 = 12816;
        // D s_15_1: read-reg s_15_0:struct
        let s_15_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call _get_TCR_EL2_Type_A1(s_15_1)
        let s_15_2: bool = u_get_TCR_EL2_Type_A1(state, tracer, s_15_1);
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // C s_15_4: const #1u : u8
        let s_15_4: bool = true;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 1u16);
        // D s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // N s_15_7: branch s_15_6 b17 b16
        if s_15_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call TTBR0_EL2_read(s_16_0)
        let s_16_1: ProductType782ac6922b48c20d = TTBR0_EL2_read(state, tracer, s_16_0);
        // S s_16_2: call _get_TTBR0_EL2_Type_ASID(s_16_1)
        let s_16_2: u16 = u_get_TTBR0_EL2_Type_ASID(state, tracer, s_16_1);
        // D s_16_3: write-var return_value <= s_16_2
        fn_state.return_value = s_16_2;
        // N s_16_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_17_0: const #18432u : u32
        let s_17_0: u32 = 18432;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType782ac6922b48c20d = {
            let value = state
                .read_register::<ProductType782ac6922b48c20d>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_TTBR1_EL2_Type_ASID(s_17_1)
        let s_17_2: u16 = u_get_TTBR1_EL2_Type_ASID(state, tracer, s_17_1);
        // D s_17_3: write-var return_value <= s_17_2
        fn_state.return_value = s_17_2;
        // N s_17_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_18_0: const #102552u : u32
        let s_18_0: u32 = 102552;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_HCR_EL2_Type_E2H(s_18_1)
        let s_18_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_18_1);
        // C s_18_3: const #102552u : u32
        let s_18_3: u32 = 102552;
        // D s_18_4: read-reg s_18_3:struct
        let s_18_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_3 as isize);
            tracer.read_register(s_18_3 as isize, value);
            value
        };
        // D s_18_5: call _get_HCR_EL2_Type_TGE(s_18_4)
        let s_18_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_18_4);
        // D s_18_6: cast zx s_18_2 -> bv
        let s_18_6: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_7: cast zx s_18_5 -> bv
        let s_18_7: Bits = Bits::new(s_18_5 as u128, 1u16);
        // D s_18_8: cast reint s_18_6 -> u128
        let s_18_8: u128 = (s_18_6.value() as u128);
        // D s_18_9: size-of s_18_6
        let s_18_9: u16 = s_18_6.length();
        // D s_18_10: cast reint s_18_7 -> u128
        let s_18_10: u128 = (s_18_7.value() as u128);
        // D s_18_11: size-of s_18_7
        let s_18_11: u16 = s_18_7.length();
        // D s_18_12: lsl s_18_8 s_18_11
        let s_18_12: u128 = s_18_8 << s_18_11;
        // D s_18_13: or s_18_12 s_18_10
        let s_18_13: u128 = ((s_18_12) | (s_18_10));
        // D s_18_14: add s_18_9 s_18_11
        let s_18_14: u16 = (s_18_9 + s_18_11);
        // D s_18_15: create-bits s_18_13 s_18_14
        let s_18_15: Bits = Bits::new(s_18_13, s_18_14);
        // D s_18_16: cast reint s_18_15 -> u8
        let s_18_16: u8 = (s_18_15.value() as u8);
        // D s_18_17: cast zx s_18_16 -> bv
        let s_18_17: Bits = Bits::new(s_18_16 as u128, 2u16);
        // C s_18_18: const #3u : u8
        let s_18_18: u8 = 3;
        // C s_18_19: cast zx s_18_18 -> bv
        let s_18_19: Bits = Bits::new(s_18_18 as u128, 2u16);
        // D s_18_20: cmp-eq s_18_17 s_18_19
        let s_18_20: bool = ((s_18_17) == (s_18_19));
        // D s_18_21: write-var gs#8265 <= s_18_20
        fn_state.gs_8265 = s_18_20;
        // N s_18_22: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_19_0: const #432u : u32
        let s_19_0: u32 = 432;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call ELUsingAArch32(s_19_1)
        let s_19_2: bool = ELUsingAArch32(state, tracer, s_19_1);
        // D s_19_3: not s_19_2
        let s_19_3: bool = !s_19_2;
        // D s_19_4: write-var gs#8264 <= s_19_3
        fn_state.gs_8264 = s_19_3;
        // N s_19_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
