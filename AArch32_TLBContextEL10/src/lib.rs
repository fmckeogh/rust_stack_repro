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
use CONTEXTIDR_NS_read::*;
use AArch32_EL2Enabled::*;
use HaveAArch32EL::*;
use TTBCR_read::*;
use u_get_TTBCR_Type_T1SZ::*;
use u_get_TTBR0_Type_CnP::*;
use TTBR0_read::*;
use u_get_TTBR1_Type_ASID::*;
use AArch32_GetVARange::*;
use u_get_TTBR0_Type_ASID::*;
use u_get_TTBR1_Type_CnP::*;
use u_get_CONTEXTIDR_Type_ASID::*;
use TTBR1_read::*;
use u_get_VTTBR_Type_VMID::*;
use u_get_TTBCR_Type_T0SZ::*;
use TTBCR_NS_read::*;
use u_get_TTBCR_Type_EAE::*;
use CONTEXTIDR_read::*;
use HaveCommonNotPrivateTransExt::*;
use u_get_TTBCR_Type_A1::*;
use common::*;
pub fn AArch32_TLBContextEL10<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ss: u32,
    va: u32,
) -> ProductTypec0d0fb0603850c4c {
    #[derive(Default)]
    struct FunctionState {
        ttbr1: ProductType5c790c8ef59cc8b2,
        gs_27833: bool,
        tlbcontext: ProductTypec0d0fb0603850c4c,
        contextidr: ProductType700c18a878c5601b,
        ga_21491: u8,
        ttbr0: ProductType5c790c8ef59cc8b2,
        ttbcr: ProductType700c18a878c5601b,
        ss: u32,
        va: u32,
    }
    let fn_state = FunctionState {
        ss,
        va,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call HaveAArch32EL(s_0_1)
        let s_0_2: bool = HaveAArch32EL(state, tracer, s_0_1);
        // N s_0_3: branch s_0_2 b20 b1
        if s_0_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call TTBCR_read(s_1_0)
        let s_1_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_1_0);
        // D s_1_2: write-var ttbcr <= s_1_1
        fn_state.ttbcr = s_1_1;
        // C s_1_3: const #() : ()
        let s_1_3: () = ();
        // S s_1_4: call TTBR0_read(s_1_3)
        let s_1_4: ProductType5c790c8ef59cc8b2 = TTBR0_read(state, tracer, s_1_3);
        // D s_1_5: write-var ttbr0 <= s_1_4
        fn_state.ttbr0 = s_1_4;
        // C s_1_6: const #() : ()
        let s_1_6: () = ();
        // S s_1_7: call TTBR1_read(s_1_6)
        let s_1_7: ProductType5c790c8ef59cc8b2 = TTBR1_read(state, tracer, s_1_6);
        // D s_1_8: write-var ttbr1 <= s_1_7
        fn_state.ttbr1 = s_1_7;
        // C s_1_9: const #() : ()
        let s_1_9: () = ();
        // S s_1_10: call CONTEXTIDR_read(s_1_9)
        let s_1_10: ProductType700c18a878c5601b = CONTEXTIDR_read(state, tracer, s_1_9);
        // D s_1_11: write-var contextidr <= s_1_10
        fn_state.contextidr = s_1_10;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_2_0: read-var ss:u32
        let s_2_0: u32 = fn_state.ss;
        // D s_2_1: write-var tlbcontext.11 <= s_2_0
        fn_state.tlbcontext._11 = s_2_0;
        // C s_2_2: const #4u : u32
        let s_2_2: u32 = 4;
        // D s_2_3: write-var tlbcontext.10 <= s_2_2
        fn_state.tlbcontext._10 = s_2_2;
        // D s_2_4: read-var ss:u32
        let s_2_4: u32 = fn_state.ss;
        // D s_2_5: call AArch32_EL2Enabled(s_2_4)
        let s_2_5: bool = AArch32_EL2Enabled(state, tracer, s_2_4);
        // N s_2_6: branch s_2_5 b19 b3
        if s_2_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_4_0: read-var ttbcr:struct
        let s_4_0: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_4_1: call _get_TTBCR_Type_EAE(s_4_0)
        let s_4_1: bool = u_get_TTBCR_Type_EAE(state, tracer, s_4_0);
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 1u16);
        // C s_4_3: const #1u : u8
        let s_4_3: bool = true;
        // C s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 1u16);
        // D s_4_5: cmp-eq s_4_2 s_4_4
        let s_4_5: bool = ((s_4_2) == (s_4_4));
        // N s_4_6: branch s_4_5 b15 b5
        if s_4_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_5_0: read-var contextidr:struct
        let s_5_0: ProductType700c18a878c5601b = fn_state.contextidr;
        // D s_5_1: call _get_CONTEXTIDR_Type_ASID(s_5_0)
        let s_5_1: u8 = u_get_CONTEXTIDR_Type_ASID(state, tracer, s_5_0);
        // C s_5_2: const #16s : i
        let s_5_2: i128 = 16;
        // D s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 8u16);
        // D s_5_4: bits-cast zx s_5_3 -> bv length s_5_2
        let s_5_4: Bits = s_5_3.zero_extend(s_5_2);
        // D s_5_5: cast reint s_5_4 -> u16
        let s_5_5: u16 = (s_5_4.value() as u16);
        // D s_5_6: write-var tlbcontext.0 <= s_5_5
        fn_state.tlbcontext._0 = s_5_5;
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_6_0: const #0u : u32
        let s_6_0: u32 = 0;
        // D s_6_1: write-var tlbcontext.12 <= s_6_0
        fn_state.tlbcontext._12 = s_6_0;
        // C s_6_2: const #64s : i
        let s_6_2: i128 = 64;
        // D s_6_3: read-var va:u32
        let s_6_3: u32 = fn_state.va;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 32u16);
        // D s_6_5: bits-cast zx s_6_4 -> bv length s_6_2
        let s_6_5: Bits = s_6_4.zero_extend(s_6_2);
        // D s_6_6: cast reint s_6_5 -> u64
        let s_6_6: u64 = (s_6_5.value() as u64);
        // D s_6_7: write-var tlbcontext.2 <= s_6_6
        fn_state.tlbcontext._2 = s_6_6;
        // C s_6_8: const #() : ()
        let s_6_8: () = ();
        // S s_6_9: call HaveCommonNotPrivateTransExt(s_6_8)
        let s_6_9: bool = HaveCommonNotPrivateTransExt(state, tracer, s_6_8);
        // N s_6_10: branch s_6_9 b14 b7
        if s_6_9 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#27833 <= s_7_0
        fn_state.gs_27833 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_8_0: read-var gs#27833:u8
        let s_8_0: bool = fn_state.gs_27833;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var tlbcontext.1 <= s_9_0
        fn_state.tlbcontext._1 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_10_0: read-var tlbcontext:struct
        let s_10_0: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_11_0: read-var ttbcr:struct
        let s_11_0: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_11_1: call _get_TTBCR_Type_T0SZ(s_11_0)
        let s_11_1: u8 = u_get_TTBCR_Type_T0SZ(state, tracer, s_11_0);
        // D s_11_2: read-var ttbcr:struct
        let s_11_2: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_11_3: call _get_TTBCR_Type_T1SZ(s_11_2)
        let s_11_3: u8 = u_get_TTBCR_Type_T1SZ(state, tracer, s_11_2);
        // D s_11_4: read-var va:u32
        let s_11_4: u32 = fn_state.va;
        // D s_11_5: call AArch32_GetVARange(s_11_4, s_11_1, s_11_3)
        let s_11_5: u32 = AArch32_GetVARange(state, tracer, s_11_4, s_11_1, s_11_3);
        // C s_11_6: const #0u : u32
        let s_11_6: u32 = 0;
        // D s_11_7: cmp-eq s_11_5 s_11_6
        let s_11_7: bool = ((s_11_5) == (s_11_6));
        // N s_11_8: branch s_11_7 b13 b12
        if s_11_7 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_12_0: read-var ttbr1:struct
        let s_12_0: ProductType5c790c8ef59cc8b2 = fn_state.ttbr1;
        // D s_12_1: call _get_TTBR1_Type_CnP(s_12_0)
        let s_12_1: bool = u_get_TTBR1_Type_CnP(state, tracer, s_12_0);
        // D s_12_2: write-var tlbcontext.1 <= s_12_1
        fn_state.tlbcontext._1 = s_12_1;
        // N s_12_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_13_0: read-var ttbr0:struct
        let s_13_0: ProductType5c790c8ef59cc8b2 = fn_state.ttbr0;
        // D s_13_1: call _get_TTBR0_Type_CnP(s_13_0)
        let s_13_1: bool = u_get_TTBR0_Type_CnP(state, tracer, s_13_0);
        // D s_13_2: write-var tlbcontext.1 <= s_13_1
        fn_state.tlbcontext._1 = s_13_1;
        // N s_13_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_14_0: read-var ttbcr:struct
        let s_14_0: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_14_1: call _get_TTBCR_Type_EAE(s_14_0)
        let s_14_1: bool = u_get_TTBCR_Type_EAE(state, tracer, s_14_0);
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 1u16);
        // C s_14_3: const #1u : u8
        let s_14_3: bool = true;
        // C s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 1u16);
        // D s_14_5: cmp-eq s_14_2 s_14_4
        let s_14_5: bool = ((s_14_2) == (s_14_4));
        // D s_14_6: write-var gs#27833 <= s_14_5
        fn_state.gs_27833 = s_14_5;
        // N s_14_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_15_0: read-var ttbcr:struct
        let s_15_0: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_15_1: call _get_TTBCR_Type_A1(s_15_0)
        let s_15_1: bool = u_get_TTBCR_Type_A1(state, tracer, s_15_0);
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 1u16);
        // C s_15_3: const #0u : u8
        let s_15_3: bool = false;
        // C s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 1u16);
        // D s_15_5: cmp-eq s_15_2 s_15_4
        let s_15_5: bool = ((s_15_2) == (s_15_4));
        // N s_15_6: branch s_15_5 b18 b16
        if s_15_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_16_0: read-var ttbr1:struct
        let s_16_0: ProductType5c790c8ef59cc8b2 = fn_state.ttbr1;
        // D s_16_1: call _get_TTBR1_Type_ASID(s_16_0)
        let s_16_1: u8 = u_get_TTBR1_Type_ASID(state, tracer, s_16_0);
        // D s_16_2: write-var ga#21491 <= s_16_1
        fn_state.ga_21491 = s_16_1;
        // N s_16_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_17_0: const #16s : i
        let s_17_0: i128 = 16;
        // D s_17_1: read-var ga#21491:u8
        let s_17_1: u8 = fn_state.ga_21491;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 8u16);
        // D s_17_3: bits-cast zx s_17_2 -> bv length s_17_0
        let s_17_3: Bits = s_17_2.zero_extend(s_17_0);
        // D s_17_4: cast reint s_17_3 -> u16
        let s_17_4: u16 = (s_17_3.value() as u16);
        // D s_17_5: write-var tlbcontext.0 <= s_17_4
        fn_state.tlbcontext._0 = s_17_4;
        // N s_17_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_18_0: read-var ttbr0:struct
        let s_18_0: ProductType5c790c8ef59cc8b2 = fn_state.ttbr0;
        // D s_18_1: call _get_TTBR0_Type_ASID(s_18_0)
        let s_18_1: u8 = u_get_TTBR0_Type_ASID(state, tracer, s_18_0);
        // D s_18_2: write-var ga#21491 <= s_18_1
        fn_state.ga_21491 = s_18_1;
        // N s_18_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_19_0: const #22408u : u32
        let s_19_0: u32 = 22408;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_VTTBR_Type_VMID(s_19_1)
        let s_19_2: u8 = u_get_VTTBR_Type_VMID(state, tracer, s_19_1);
        // C s_19_3: const #16s : i
        let s_19_3: i128 = 16;
        // D s_19_4: cast zx s_19_2 -> bv
        let s_19_4: Bits = Bits::new(s_19_2 as u128, 8u16);
        // D s_19_5: bits-cast zx s_19_4 -> bv length s_19_3
        let s_19_5: Bits = s_19_4.zero_extend(s_19_3);
        // D s_19_6: cast reint s_19_5 -> u16
        let s_19_6: u16 = (s_19_5.value() as u16);
        // D s_19_7: write-var tlbcontext.13 <= s_19_6
        fn_state.tlbcontext._13 = s_19_6;
        // N s_19_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call TTBCR_NS_read(s_20_0)
        let s_20_1: ProductType700c18a878c5601b = TTBCR_NS_read(state, tracer, s_20_0);
        // D s_20_2: write-var ttbcr <= s_20_1
        fn_state.ttbcr = s_20_1;
        // C s_20_3: const #11616u : u32
        let s_20_3: u32 = 11616;
        // D s_20_4: read-reg s_20_3:struct
        let s_20_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_3 as isize);
            tracer.read_register(s_20_3 as isize, value);
            value
        };
        // D s_20_5: write-var ttbr0 <= s_20_4
        fn_state.ttbr0 = s_20_4;
        // C s_20_6: const #20024u : u32
        let s_20_6: u32 = 20024;
        // D s_20_7: read-reg s_20_6:struct
        let s_20_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_6 as isize);
            tracer.read_register(s_20_6 as isize, value);
            value
        };
        // D s_20_8: write-var ttbr1 <= s_20_7
        fn_state.ttbr1 = s_20_7;
        // C s_20_9: const #() : ()
        let s_20_9: () = ();
        // S s_20_10: call CONTEXTIDR_NS_read(s_20_9)
        let s_20_10: ProductType700c18a878c5601b = CONTEXTIDR_NS_read(
            state,
            tracer,
            s_20_9,
        );
        // D s_20_11: write-var contextidr <= s_20_10
        fn_state.contextidr = s_20_10;
        // N s_20_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}
