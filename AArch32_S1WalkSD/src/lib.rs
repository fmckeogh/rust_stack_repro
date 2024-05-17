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
use u_get_TTBCR_Type_PD1::*;
use u_get_TTBR1_Type_RGN::*;
use RemapRegsHaveResetValues::*;
use HaveAArch32EL::*;
use AArch32_EL2Enabled::*;
use TTBCR_read::*;
use u_get_SCTLR_Type_TRE::*;
use AArch32_RemappedTEXDecode::*;
use SCTLR_NS_read::*;
use u__IMPDEF_MemoryAttributes::*;
use Zeros::*;
use u_get_TTBR0_Type_S::*;
use S1TLBCache::*;
use u_get_TTBR0_Type_IRGN::*;
use AArch32_GetS1TLBContext::*;
use u_get_TTBCR_Type_N::*;
use SCTLR_read__2::*;
use ELStateUsingAArch32::*;
use u_get_TTBR0_Type_NOS::*;
use u_get_TTBR0_Type_TTB0::*;
use NormalNCMemAttr::*;
use AArch32_DecodeDescriptorTypeSD::*;
use u_get_HCR_EL2_Type_VM::*;
use u_get_TTBR1_Type_IRGN::*;
use AArch32_TranslationSizeSD::*;
use u_get_SCTLR_Type_AFE::*;
use u__UNKNOWN_TTWState::*;
use FetchDescriptor::*;
use AArch32_S2Translate::*;
use place_subrange::*;
use AArch32_S1DCacheEnabled::*;
use HCR_read::*;
use TTBR0_read::*;
use CreateAccDescS1TTW::*;
use u__IMPDEF_boolean::*;
use is_zero_subrange::*;
use EffectiveShareability::*;
use u_get_TTBCR_Type_PD0::*;
use u_get_TTBR0_Type_RGN::*;
use u_get_HCR_Type_VM::*;
use u_get_TTBR1_Type_NOS::*;
use S1TLBLookup::*;
use TTBR1_read::*;
use u_get_TTBR1_Type_S::*;
use AArch32_DefaultTEXDecode::*;
use u_get_TTBR1_Type_TTB1::*;
use place_slice::*;
use TTBCR_NS_read::*;
use u_get_TTBCR_Type_EAE::*;
use u_get_SCTLR_Type_EE::*;
use WalkMemAttrs::*;
use common::*;
pub fn AArch32_S1WalkSD<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    regime: u32,
    accdesc: ProductType9878976b5bcce9c9,
    va: u32,
) -> ProductType201519a0f62623dc {
    #[derive(Default)]
    struct FunctionState {
        pxn: bool,
        domain: u8,
        tlbcontext: ProductTypec0d0fb0603850c4c,
        baseaddress: ProductTypeda0231e9dc169f81,
        gs_28756: bool,
        c: bool,
        gs_28717: bool,
        nos: bool,
        ttbr1: ProductType5c790c8ef59cc8b2,
        gs_28959: bool,
        varange: u32,
        translation_info: ProductTypeb525737120e184b3,
        b: bool,
        walkaddress: ProductTypece7c66ccb2cab13e,
        ga_22353: ProductTypef170cab34335b70c,
        tlbrecord: ProductTypee47dd77b186df56e,
        gs_454058: ProductTypeb4cea7287e2eb9d6,
        ga_22252: bool,
        nG: bool,
        ns: bool,
        ttbr0: ProductType5c790c8ef59cc8b2,
        xn: bool,
        s: bool,
        irgn: u8,
        descriptor: u32,
        walkaccess: ProductType9878976b5bcce9c9,
        pd: bool,
        gs_28960: bool,
        fault: ProductType1d757adad216cdef,
        s2fault: ProductType1d757adad216cdef,
        finalwalkstateshadow_536: ProductType96e7acababe246a1,
        ttb: u32,
        tex: u8,
        gs_454060: ProductTypeb4cea7287e2eb9d6,
        return_value: ProductType201519a0f62623dc,
        tre: bool,
        ga_22257: ProductTypef170cab34335b70c,
        ga_22263: ProductTypeda0231e9dc169f81,
        ga_22266: ProductTypeda0231e9dc169f81,
        s2walkaddress: ProductTypece7c66ccb2cab13e,
        ttbcr: ProductType700c18a878c5601b,
        ee: bool,
        sctlr: ProductType700c18a878c5601b,
        ap: u8,
        ga_22245: ProductTypef170cab34335b70c,
        ga_22223: ProductTypee47dd77b186df56e,
        gs_28757: bool,
        afe: bool,
        n: i64,
        index: u32,
        nshadow_535: i64,
        walkstate: ProductType96e7acababe246a1,
        ga_22272: ProductTypedc31059ca7e2391c,
        gs_28755: bool,
        rgn: u8,
        tlbentry: ProductTypeeb828c17bbe5e68,
        ga_22286: u32,
        gs_28776: bool,
        fault_in: ProductType1d757adad216cdef,
        regime: u32,
        accdesc: ProductType9878976b5bcce9c9,
        va: u32,
    }
    let fn_state = FunctionState {
        fault_in,
        regime,
        accdesc,
        va,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_0_0: read-var fault_in:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_1: write-var fault <= s_0_0
        fn_state.fault = s_0_0;
        // C s_0_2: const #19088u : u32
        let s_0_2: u32 = 19088;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: bool = {
            let value = state.read_register::<bool>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // N s_0_4: branch s_0_3 b85 b1
        if s_0_3 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_2_0: read-var regime:u32
        let s_2_0: u32 = fn_state.regime;
        // C s_2_1: const #1u : u32
        let s_2_1: u32 = 1;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b84 b3
        if s_2_2 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_3_0: const #424u : u32
        let s_3_0: u32 = 424;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call HaveAArch32EL(s_3_1)
        let s_3_2: bool = HaveAArch32EL(state, tracer, s_3_1);
        // N s_3_3: branch s_3_2 b83 b4
        if s_3_2 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call SCTLR_read__2(s_4_0)
        let s_4_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_4_0);
        // D s_4_2: write-var sctlr <= s_4_1
        fn_state.sctlr = s_4_1;
        // C s_4_3: const #() : ()
        let s_4_3: () = ();
        // S s_4_4: call TTBCR_read(s_4_3)
        let s_4_4: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_4_3);
        // D s_4_5: write-var ttbcr <= s_4_4
        fn_state.ttbcr = s_4_4;
        // C s_4_6: const #() : ()
        let s_4_6: () = ();
        // S s_4_7: call TTBR0_read(s_4_6)
        let s_4_7: ProductType5c790c8ef59cc8b2 = TTBR0_read(state, tracer, s_4_6);
        // D s_4_8: write-var ttbr0 <= s_4_7
        fn_state.ttbr0 = s_4_7;
        // C s_4_9: const #() : ()
        let s_4_9: () = ();
        // S s_4_10: call TTBR1_read(s_4_9)
        let s_4_10: ProductType5c790c8ef59cc8b2 = TTBR1_read(state, tracer, s_4_9);
        // D s_4_11: write-var ttbr1 <= s_4_10
        fn_state.ttbr1 = s_4_10;
        // N s_4_12: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_5_0: read-var ttbcr:struct
        let s_5_0: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_5_1: call _get_TTBCR_Type_EAE(s_5_0)
        let s_5_1: bool = u_get_TTBCR_Type_EAE(state, tracer, s_5_0);
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 1u16);
        // C s_5_3: const #0u : u8
        let s_5_3: bool = false;
        // C s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 1u16);
        // D s_5_5: cmp-eq s_5_2 s_5_4
        let s_5_5: bool = ((s_5_2) == (s_5_4));
        // N s_5_6: assert s_5_5
        let s_5_6: () = assert!(s_5_5);
        // D s_5_7: read-var sctlr:struct
        let s_5_7: ProductType700c18a878c5601b = fn_state.sctlr;
        // D s_5_8: call _get_SCTLR_Type_EE(s_5_7)
        let s_5_8: bool = u_get_SCTLR_Type_EE(state, tracer, s_5_7);
        // D s_5_9: write-var ee <= s_5_8
        fn_state.ee = s_5_8;
        // D s_5_10: read-var sctlr:struct
        let s_5_10: ProductType700c18a878c5601b = fn_state.sctlr;
        // D s_5_11: call _get_SCTLR_Type_AFE(s_5_10)
        let s_5_11: bool = u_get_SCTLR_Type_AFE(state, tracer, s_5_10);
        // D s_5_12: write-var afe <= s_5_11
        fn_state.afe = s_5_11;
        // D s_5_13: read-var sctlr:struct
        let s_5_13: ProductType700c18a878c5601b = fn_state.sctlr;
        // D s_5_14: call _get_SCTLR_Type_TRE(s_5_13)
        let s_5_14: bool = u_get_SCTLR_Type_TRE(state, tracer, s_5_13);
        // D s_5_15: write-var tre <= s_5_14
        fn_state.tre = s_5_14;
        // D s_5_16: read-var ttbcr:struct
        let s_5_16: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_5_17: call _get_TTBCR_Type_N(s_5_16)
        let s_5_17: u8 = u_get_TTBCR_Type_N(state, tracer, s_5_16);
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 3u16);
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (s_5_18.value() as i128);
        // D s_5_20: cast reint s_5_19 -> i64
        let s_5_20: i64 = (s_5_19 as i64);
        // D s_5_21: write-var n <= s_5_20
        fn_state.n = s_5_20;
        // C s_5_22: const #0s : i
        let s_5_22: i128 = 0;
        // D s_5_23: read-var n:i64
        let s_5_23: i64 = fn_state.n;
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: cmp-eq s_5_24 s_5_22
        let s_5_25: bool = ((s_5_24) == (s_5_22));
        // N s_5_26: branch s_5_25 b82 b6
        if s_5_25 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_6_0: const #32s : i
        let s_6_0: i128 = 32;
        // D s_6_1: read-var n:i64
        let s_6_1: i64 = fn_state.n;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: sub s_6_0 s_6_2
        let s_6_3: i128 = ((s_6_0) - (s_6_2));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // C s_6_5: const #31s : i
        let s_6_5: i128 = 31;
        // D s_6_6: read-var va:u32
        let s_6_6: u32 = fn_state.va;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 32u16);
        // D s_6_8: cast zx s_6_4 -> i
        let s_6_8: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_9: call is_zero_subrange(s_6_7, s_6_5, s_6_8)
        let s_6_9: bool = is_zero_subrange(state, tracer, s_6_7, s_6_5, s_6_8);
        // D s_6_10: write-var gs#28717 <= s_6_9
        fn_state.gs_28717 = s_6_9;
        // N s_6_11: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_7_0: read-var gs#28717:u8
        let s_7_0: bool = fn_state.gs_28717;
        // N s_7_1: branch s_7_0 b81 b8
        if s_7_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_8_0: const #0s : i64
        let s_8_0: i64 = 0;
        // D s_8_1: write-var n <= s_8_0
        fn_state.n = s_8_0;
        // D s_8_2: read-var ttbr1:struct
        let s_8_2: ProductType5c790c8ef59cc8b2 = fn_state.ttbr1;
        // D s_8_3: call _get_TTBR1_Type_TTB1(s_8_2)
        let s_8_3: u32 = u_get_TTBR1_Type_TTB1(state, tracer, s_8_2);
        // C s_8_4: const #7s : i
        let s_8_4: i128 = 7;
        // S s_8_5: call Zeros(s_8_4)
        let s_8_5: Bits = Zeros(state, tracer, s_8_4);
        // S s_8_6: cast reint s_8_5 -> u8
        let s_8_6: u8 = (s_8_5.value() as u8);
        // D s_8_7: cast zx s_8_3 -> bv
        let s_8_7: Bits = Bits::new(s_8_3 as u128, 25u16);
        // S s_8_8: cast zx s_8_6 -> bv
        let s_8_8: Bits = Bits::new(s_8_6 as u128, 7u16);
        // D s_8_9: cast reint s_8_7 -> u128
        let s_8_9: u128 = (s_8_7.value() as u128);
        // D s_8_10: size-of s_8_7
        let s_8_10: u16 = s_8_7.length();
        // S s_8_11: cast reint s_8_8 -> u128
        let s_8_11: u128 = (s_8_8.value() as u128);
        // D s_8_12: size-of s_8_8
        let s_8_12: u16 = s_8_8.length();
        // D s_8_13: lsl s_8_9 s_8_12
        let s_8_13: u128 = s_8_9 << s_8_12;
        // D s_8_14: or s_8_13 s_8_11
        let s_8_14: u128 = ((s_8_13) | (s_8_11));
        // D s_8_15: add s_8_10 s_8_12
        let s_8_15: u16 = (s_8_10 + s_8_12);
        // D s_8_16: create-bits s_8_14 s_8_15
        let s_8_16: Bits = Bits::new(s_8_14, s_8_15);
        // D s_8_17: cast reint s_8_16 -> u32
        let s_8_17: u32 = (s_8_16.value() as u32);
        // D s_8_18: write-var ttb <= s_8_17
        fn_state.ttb = s_8_17;
        // D s_8_19: read-var ttbcr:struct
        let s_8_19: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_8_20: call _get_TTBCR_Type_PD1(s_8_19)
        let s_8_20: bool = u_get_TTBCR_Type_PD1(state, tracer, s_8_19);
        // D s_8_21: write-var pd <= s_8_20
        fn_state.pd = s_8_20;
        // D s_8_22: read-var ttbr1:struct
        let s_8_22: ProductType5c790c8ef59cc8b2 = fn_state.ttbr1;
        // D s_8_23: call _get_TTBR1_Type_IRGN(s_8_22)
        let s_8_23: u8 = u_get_TTBR1_Type_IRGN(state, tracer, s_8_22);
        // D s_8_24: write-var irgn <= s_8_23
        fn_state.irgn = s_8_23;
        // D s_8_25: read-var ttbr1:struct
        let s_8_25: ProductType5c790c8ef59cc8b2 = fn_state.ttbr1;
        // D s_8_26: call _get_TTBR1_Type_RGN(s_8_25)
        let s_8_26: u8 = u_get_TTBR1_Type_RGN(state, tracer, s_8_25);
        // D s_8_27: write-var rgn <= s_8_26
        fn_state.rgn = s_8_26;
        // D s_8_28: read-var ttbr1:struct
        let s_8_28: ProductType5c790c8ef59cc8b2 = fn_state.ttbr1;
        // D s_8_29: call _get_TTBR1_Type_S(s_8_28)
        let s_8_29: bool = u_get_TTBR1_Type_S(state, tracer, s_8_28);
        // D s_8_30: write-var s <= s_8_29
        fn_state.s = s_8_29;
        // D s_8_31: read-var ttbr1:struct
        let s_8_31: ProductType5c790c8ef59cc8b2 = fn_state.ttbr1;
        // D s_8_32: call _get_TTBR1_Type_NOS(s_8_31)
        let s_8_32: bool = u_get_TTBR1_Type_NOS(state, tracer, s_8_31);
        // D s_8_33: write-var nos <= s_8_32
        fn_state.nos = s_8_32;
        // C s_8_34: const #1u : u32
        let s_8_34: u32 = 1;
        // D s_8_35: write-var varange <= s_8_34
        fn_state.varange = s_8_34;
        // N s_8_36: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_9_0: read-var n:i64
        let s_9_0: i64 = fn_state.n;
        // D s_9_1: write-var nshadow#535 <= s_9_0
        fn_state.nshadow_535 = s_9_0;
        // D s_9_2: read-var pd:u8
        let s_9_2: bool = fn_state.pd;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #1u : u8
        let s_9_4: bool = true;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // N s_9_7: branch s_9_6 b80 b10
        if s_9_6 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_10_0: read-var accdesc.25:struct
        let s_10_0: u32 = fn_state.accdesc._25;
        // C s_10_1: const #3u : u32
        let s_10_1: u32 = 3;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b79 b11
        if s_10_2 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_11_0: const #0u : u32
        let s_11_0: u32 = 0;
        // D s_11_1: write-var baseaddress.1 <= s_11_0
        fn_state.baseaddress._1 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_12_0: const #14s : i
        let s_12_0: i128 = 14;
        // D s_12_1: read-var nshadow#535:i64
        let s_12_1: i64 = fn_state.nshadow_535;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: sub s_12_0 s_12_2
        let s_12_3: i128 = ((s_12_0) - (s_12_2));
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // C s_12_5: const #14s : i
        let s_12_5: i128 = 14;
        // D s_12_6: read-var nshadow#535:i64
        let s_12_6: i64 = fn_state.nshadow_535;
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_8: sub s_12_5 s_12_7
        let s_12_8: i128 = ((s_12_5) - (s_12_7));
        // D s_12_9: cast reint s_12_8 -> i64
        let s_12_9: i64 = (s_12_8 as i64);
        // C s_12_10: const #56s : i
        let s_12_10: i128 = 56;
        // C s_12_11: const #31s : i
        let s_12_11: i128 = 31;
        // D s_12_12: read-var ttb:u32
        let s_12_12: u32 = fn_state.ttb;
        // D s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 32u16);
        // D s_12_14: cast zx s_12_4 -> i
        let s_12_14: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_15: cast zx s_12_9 -> i
        let s_12_15: i128 = (i128::try_from(s_12_9).unwrap());
        // D s_12_16: call place_subrange(s_12_10, s_12_13, s_12_11, s_12_14, s_12_15)
        let s_12_16: Bits = place_subrange(
            state,
            tracer,
            s_12_10,
            s_12_13,
            s_12_11,
            s_12_14,
            s_12_15,
        );
        // D s_12_17: cast reint s_12_16 -> u56
        let s_12_17: u64 = (s_12_16.value() as u64);
        // D s_12_18: write-var baseaddress.0 <= s_12_17
        fn_state.baseaddress._0 = s_12_17;
        // D s_12_19: read-var baseaddress:struct
        let s_12_19: ProductTypeda0231e9dc169f81 = fn_state.baseaddress;
        // D s_12_20: write-var walkstate.0 <= s_12_19
        fn_state.walkstate._0 = s_12_19;
        // C s_12_21: const #1u : u8
        let s_12_21: bool = true;
        // D s_12_22: write-var walkstate.8 <= s_12_21
        fn_state.walkstate._8 = s_12_21;
        // D s_12_23: read-var s:u8
        let s_12_23: bool = fn_state.s;
        // D s_12_24: cast zx s_12_23 -> bv
        let s_12_24: Bits = Bits::new(s_12_23 as u128, 1u16);
        // D s_12_25: read-var nos:u8
        let s_12_25: bool = fn_state.nos;
        // D s_12_26: cast zx s_12_25 -> bv
        let s_12_26: Bits = Bits::new(s_12_25 as u128, 1u16);
        // D s_12_27: cast reint s_12_24 -> u128
        let s_12_27: u128 = (s_12_24.value() as u128);
        // D s_12_28: size-of s_12_24
        let s_12_28: u16 = s_12_24.length();
        // D s_12_29: cast reint s_12_26 -> u128
        let s_12_29: u128 = (s_12_26.value() as u128);
        // D s_12_30: size-of s_12_26
        let s_12_30: u16 = s_12_26.length();
        // D s_12_31: lsl s_12_27 s_12_30
        let s_12_31: u128 = s_12_27 << s_12_30;
        // D s_12_32: or s_12_31 s_12_29
        let s_12_32: u128 = ((s_12_31) | (s_12_29));
        // D s_12_33: add s_12_28 s_12_30
        let s_12_33: u16 = (s_12_28 + s_12_30);
        // D s_12_34: create-bits s_12_32 s_12_33
        let s_12_34: Bits = Bits::new(s_12_32, s_12_33);
        // D s_12_35: cast reint s_12_34 -> u8
        let s_12_35: u8 = (s_12_34.value() as u8);
        // D s_12_36: read-var irgn:u8
        let s_12_36: u8 = fn_state.irgn;
        // D s_12_37: read-var rgn:u8
        let s_12_37: u8 = fn_state.rgn;
        // D s_12_38: call WalkMemAttrs(s_12_35, s_12_36, s_12_37)
        let s_12_38: ProductTypef170cab34335b70c = WalkMemAttrs(
            state,
            tracer,
            s_12_35,
            s_12_36,
            s_12_37,
        );
        // D s_12_39: write-var walkstate.7 <= s_12_38
        fn_state.walkstate._7 = s_12_38;
        // C s_12_40: const #1s : i64
        let s_12_40: i64 = 1;
        // C s_12_41: cast zx s_12_40 -> i
        let s_12_41: i128 = (i128::try_from(s_12_40).unwrap());
        // D s_12_42: write-var walkstate.6 <= s_12_41
        fn_state.walkstate._6 = s_12_41;
        // C s_12_43: const #1u : u8
        let s_12_43: bool = true;
        // D s_12_44: write-var walkstate.5 <= s_12_43
        fn_state.walkstate._5 = s_12_43;
        // C s_12_45: const #64s : i
        let s_12_45: i128 = 64;
        // D s_12_46: read-var va:u32
        let s_12_46: u32 = fn_state.va;
        // D s_12_47: cast zx s_12_46 -> bv
        let s_12_47: Bits = Bits::new(s_12_46 as u128, 32u16);
        // D s_12_48: bits-cast zx s_12_47 -> bv length s_12_45
        let s_12_48: Bits = s_12_47.zero_extend(s_12_45);
        // D s_12_49: cast reint s_12_48 -> u64
        let s_12_49: u64 = (s_12_48.value() as u64);
        // D s_12_50: write-var walkaddress.7 <= s_12_49
        fn_state.walkaddress._7 = s_12_49;
        // D s_12_51: read-var regime:u32
        let s_12_51: u32 = fn_state.regime;
        // D s_12_52: call AArch32_S1DCacheEnabled(s_12_51)
        let s_12_52: bool = AArch32_S1DCacheEnabled(state, tracer, s_12_51);
        // D s_12_53: not s_12_52
        let s_12_53: bool = !s_12_52;
        // N s_12_54: branch s_12_53 b78 b13
        if s_12_53 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_13_0: read-var walkstate.7:struct
        let s_13_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_13_1: write-var walkaddress.2 <= s_13_0
        fn_state.walkaddress._2 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_14_0: read-var regime:u32
        let s_14_0: u32 = fn_state.regime;
        // C s_14_1: const #4u : u32
        let s_14_1: u32 = 4;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // N s_14_3: branch s_14_2 b77 b15
        if s_14_2 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#28755 <= s_15_0
        fn_state.gs_28755 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_16_0: read-var gs#28755:u8
        let s_16_0: bool = fn_state.gs_28755;
        // N s_16_1: branch s_16_0 b73 b17
        if s_16_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#28756 <= s_17_0
        fn_state.gs_28756 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_18_0: read-var gs#28756:u8
        let s_18_0: bool = fn_state.gs_28756;
        // N s_18_1: branch s_18_0 b72 b19
        if s_18_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#28757 <= s_19_0
        fn_state.gs_28757 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_20_0: read-var gs#28757:u8
        let s_20_0: bool = fn_state.gs_28757;
        // N s_20_1: branch s_20_0 b71 b21
        if s_20_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_21_0: read-var walkaddress.2:struct
        let s_21_0: ProductTypef170cab34335b70c = fn_state.walkaddress._2;
        // D s_21_1: call EffectiveShareability(s_21_0)
        let s_21_1: u32 = EffectiveShareability(state, tracer, s_21_0);
        // D s_21_2: write-var walkaddress.2.5 <= s_21_1
        fn_state.walkaddress._2._5 = s_21_1;
        // N s_21_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_23_0: read-var walkstate.6:struct
        let s_23_0: i128 = fn_state.walkstate._6;
        // D s_23_1: write-var fault.9 <= s_23_0
        fn_state.fault._9 = s_23_0;
        // D s_23_2: read-var walkstate.6:struct
        let s_23_2: i128 = fn_state.walkstate._6;
        // C s_23_3: const #1s : i
        let s_23_3: i128 = 1;
        // D s_23_4: cmp-eq s_23_2 s_23_3
        let s_23_4: bool = ((s_23_2) == (s_23_3));
        // N s_23_5: branch s_23_4 b70 b24
        if s_23_4 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_24_0: const #32s : i
        let s_24_0: i128 = 32;
        // C s_24_1: const #19s : i
        let s_24_1: i128 = 19;
        // C s_24_2: const #12s : i
        let s_24_2: i128 = 12;
        // C s_24_3: const #2s : i
        let s_24_3: i128 = 2;
        // D s_24_4: read-var va:u32
        let s_24_4: u32 = fn_state.va;
        // D s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 32u16);
        // D s_24_6: call place_subrange(s_24_0, s_24_5, s_24_1, s_24_2, s_24_3)
        let s_24_6: Bits = place_subrange(
            state,
            tracer,
            s_24_0,
            s_24_5,
            s_24_1,
            s_24_2,
            s_24_3,
        );
        // D s_24_7: cast reint s_24_6 -> u32
        let s_24_7: u32 = (s_24_6.value() as u32);
        // D s_24_8: write-var index <= s_24_7
        fn_state.index = s_24_7;
        // N s_24_9: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_25_0: read-var walkstate.0:struct
        let s_25_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_25_1: write-var ga#22263 <= s_25_0
        fn_state.ga_22263 = s_25_0;
        // D s_25_2: read-var ga#22263.0:struct
        let s_25_2: u64 = fn_state.ga_22263._0;
        // C s_25_3: const #56s : i
        let s_25_3: i128 = 56;
        // D s_25_4: read-var index:u32
        let s_25_4: u32 = fn_state.index;
        // D s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 32u16);
        // D s_25_6: bits-cast zx s_25_5 -> bv length s_25_3
        let s_25_6: Bits = s_25_5.zero_extend(s_25_3);
        // D s_25_7: cast reint s_25_6 -> u56
        let s_25_7: u64 = (s_25_6.value() as u64);
        // D s_25_8: cast zx s_25_2 -> bv
        let s_25_8: Bits = Bits::new(s_25_2 as u128, 56u16);
        // D s_25_9: cast zx s_25_7 -> bv
        let s_25_9: Bits = Bits::new(s_25_7 as u128, 56u16);
        // D s_25_10: or s_25_8 s_25_9
        let s_25_10: Bits = ((s_25_8) | (s_25_9));
        // D s_25_11: cast reint s_25_10 -> u56
        let s_25_11: u64 = (s_25_10.value() as u64);
        // D s_25_12: write-var walkaddress.3.0 <= s_25_11
        fn_state.walkaddress._3._0 = s_25_11;
        // D s_25_13: read-var walkstate.0:struct
        let s_25_13: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_25_14: write-var ga#22266 <= s_25_13
        fn_state.ga_22266 = s_25_13;
        // D s_25_15: read-var ga#22266.1:struct
        let s_25_15: u32 = fn_state.ga_22266._1;
        // D s_25_16: write-var walkaddress.3.1 <= s_25_15
        fn_state.walkaddress._3._1 = s_25_15;
        // D s_25_17: read-var walkstate.6:struct
        let s_25_17: i128 = fn_state.walkstate._6;
        // C s_25_18: const #1s : i64
        let s_25_18: i64 = 1;
        // C s_25_19: cast zx s_25_18 -> i
        let s_25_19: i128 = (i128::try_from(s_25_18).unwrap());
        // D s_25_20: cmp-eq s_25_17 s_25_19
        let s_25_20: bool = ((s_25_17) == (s_25_19));
        // D s_25_21: read-var varange:u32
        let s_25_21: u32 = fn_state.varange;
        // D s_25_22: read-var accdesc:struct
        let s_25_22: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_25_23: call CreateAccDescS1TTW(s_25_20, s_25_21, s_25_22)
        let s_25_23: ProductType9878976b5bcce9c9 = CreateAccDescS1TTW(
            state,
            tracer,
            s_25_20,
            s_25_21,
            s_25_22,
        );
        // D s_25_24: write-var walkaccess <= s_25_23
        fn_state.walkaccess = s_25_23;
        // C s_25_25: const #() : ()
        let s_25_25: () = ();
        // D s_25_26: create-sum enum = 0:"s_25_25"
        let s_25_26: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_25_25);
        // C s_25_27: const #() : ()
        let s_25_27: () = ();
        // D s_25_28: create-sum enum = 0:"s_25_27"
        let s_25_28: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_25_27);
        // D s_25_29: read-var walkaddress.7:struct
        let s_25_29: u64 = fn_state.walkaddress._7;
        // D s_25_30: read-var walkstate.6:struct
        let s_25_30: i128 = fn_state.walkstate._6;
        // D s_25_31: create-sum enum = 1:"s_25_30"
        let s_25_31: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_1(s_25_30);
        // C s_25_32: const #() : ()
        let s_25_32: () = ();
        // D s_25_33: create-sum enum = 0:"s_25_32"
        let s_25_33: SumType3cca557f9e907281 = SumType3cca557f9e907281::_0(s_25_32);
        // C s_25_34: const #() : ()
        let s_25_34: () = ();
        // D s_25_35: create-sum enum = 0:"s_25_34"
        let s_25_35: SumTypefc0aa8a49e605a17 = SumTypefc0aa8a49e605a17::_0(s_25_34);
        // C s_25_36: const #() : ()
        let s_25_36: () = ();
        // D s_25_37: create-sum enum = 0:"s_25_36"
        let s_25_37: SumType3436044442b382d9 = SumType3436044442b382d9::_0(s_25_36);
        // D s_25_38: read-var walkaddress.2:struct
        let s_25_38: ProductTypef170cab34335b70c = fn_state.walkaddress._2;
        // D s_25_39: read-var regime:u32
        let s_25_39: u32 = fn_state.regime;
        // D s_25_40: create-product struct = ["s_25_28", "s_25_38", "s_25_39", "s_25_31", "s_25_35", "s_25_33", "s_25_37", "s_25_29", "s_25_26"]
        let s_25_40: ProductTypeb525737120e184b3 = ProductTypeb525737120e184b3 {
            _0: s_25_28,
            _1: s_25_38,
            _2: s_25_39,
            _3: s_25_31,
            _4: s_25_35,
            _5: s_25_33,
            _6: s_25_37,
            _7: s_25_29,
            _8: s_25_26,
        };
        // D s_25_41: write-var translation_info <= s_25_40
        fn_state.translation_info = s_25_40;
        // D s_25_42: read-var regime:u32
        let s_25_42: u32 = fn_state.regime;
        // C s_25_43: const #4u : u32
        let s_25_43: u32 = 4;
        // D s_25_44: cmp-eq s_25_42 s_25_43
        let s_25_44: bool = ((s_25_42) == (s_25_43));
        // N s_25_45: branch s_25_44 b69 b26
        if s_25_44 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#28776 <= s_26_0
        fn_state.gs_28776 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_27_0: read-var gs#28776:u8
        let s_27_0: bool = fn_state.gs_28776;
        // N s_27_1: branch s_27_0 b66 b28
        if s_27_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_28_0: const #32s : i64
        let s_28_0: i64 = 32;
        // D s_28_1: read-var ee:u8
        let s_28_1: bool = fn_state.ee;
        // D s_28_2: read-var walkaddress:struct
        let s_28_2: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // D s_28_3: read-var walkaccess:struct
        let s_28_3: ProductType9878976b5bcce9c9 = fn_state.walkaccess;
        // D s_28_4: read-var fault:struct
        let s_28_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_28_5: read-var translation_info:struct
        let s_28_5: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_28_6: call FetchDescriptor(s_28_1, s_28_2, s_28_3, s_28_4, s_28_0, s_28_5)
        let s_28_6: ProductTypeb4cea7287e2eb9d6 = FetchDescriptor(
            state,
            tracer,
            s_28_1,
            s_28_2,
            s_28_3,
            s_28_4,
            s_28_0,
            s_28_5,
        );
        // D s_28_7: write-var gs#454058 <= s_28_6
        fn_state.gs_454058 = s_28_6;
        // D s_28_8: read-var gs#454058.0:struct
        let s_28_8: ProductType1d757adad216cdef = fn_state.gs_454058._0;
        // D s_28_9: read-var gs#454058.1:struct
        let s_28_9: Bits = fn_state.gs_454058._1;
        // D s_28_10: cast reint s_28_9 -> u32
        let s_28_10: u32 = (s_28_9.value() as u32);
        // D s_28_11: write-var fault <= s_28_8
        fn_state.fault = s_28_8;
        // D s_28_12: write-var descriptor <= s_28_10
        fn_state.descriptor = s_28_10;
        // N s_28_13: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_29_0: read-var fault.16:struct
        let s_29_0: u32 = fn_state.fault._16;
        // C s_29_1: const #0u : u32
        let s_29_1: u32 = 0;
        // D s_29_2: cmp-eq s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) == (s_29_1));
        // N s_29_3: branch s_29_2 b65 b30
        if s_29_2 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_30_0: read-var walkstate.6:struct
        let s_30_0: i128 = fn_state.walkstate._6;
        // D s_30_1: read-var descriptor:u32
        let s_30_1: u32 = fn_state.descriptor;
        // D s_30_2: call AArch32_DecodeDescriptorTypeSD(s_30_1, s_30_0)
        let s_30_2: u32 = AArch32_DecodeDescriptorTypeSD(state, tracer, s_30_1, s_30_0);
        // D s_30_3: write-var walkstate.12 <= s_30_2
        fn_state.walkstate._12 = s_30_2;
        // D s_30_4: read-var walkstate.12:struct
        let s_30_4: u32 = fn_state.walkstate._12;
        // D s_30_5: write-var ga#22286 <= s_30_4
        fn_state.ga_22286 = s_30_4;
        // C s_30_6: const #1u : u32
        let s_30_6: u32 = 1;
        // D s_30_7: read-var ga#22286:u32
        let s_30_7: u32 = fn_state.ga_22286;
        // D s_30_8: cmp-eq s_30_6 s_30_7
        let s_30_8: bool = ((s_30_6) == (s_30_7));
        // D s_30_9: not s_30_8
        let s_30_9: bool = !s_30_8;
        // N s_30_10: branch s_30_9 b33 b31
        if s_30_9 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_31_0: read-var domain:u8
        let s_31_0: u8 = fn_state.domain;
        // D s_31_1: write-var fault.4 <= s_31_0
        fn_state.fault._4 = s_31_0;
        // C s_31_2: const #6u : u32
        let s_31_2: u32 = 6;
        // D s_31_3: write-var fault.16 <= s_31_2
        fn_state.fault._16 = s_31_2;
        // C s_31_4: const #() : ()
        let s_31_4: () = ();
        // S s_31_5: call __UNKNOWN_TTWState(s_31_4)
        let s_31_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_31_4,
        );
        // D s_31_6: read-var fault:struct
        let s_31_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_31_7: create-product struct = ["s_31_6", "s_31_5"]
        let s_31_7: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_31_6,
            _1: s_31_5,
        };
        // D s_31_8: write-var return_value <= s_31_7
        fn_state.return_value = s_31_7;
        // N s_31_9: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_32_0: read-var return_value:struct
        let s_32_0: ProductType201519a0f62623dc = fn_state.return_value;
        // N s_32_1: return s_32_0
        return s_32_0;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_33_0: const #0u : u32
        let s_33_0: u32 = 0;
        // D s_33_1: read-var ga#22286:u32
        let s_33_1: u32 = fn_state.ga_22286;
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // D s_33_3: not s_33_2
        let s_33_3: bool = !s_33_2;
        // N s_33_4: branch s_33_3 b56 b34
        if s_33_3 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_34_0: const #5s : i
        let s_34_0: i128 = 5;
        // D s_34_1: read-var descriptor:u32
        let s_34_1: u32 = fn_state.descriptor;
        // D s_34_2: cast zx s_34_1 -> bv
        let s_34_2: Bits = Bits::new(s_34_1 as u128, 32u16);
        // C s_34_3: const #1s : i64
        let s_34_3: i64 = 1;
        // C s_34_4: cast zx s_34_3 -> i
        let s_34_4: i128 = (i128::try_from(s_34_3).unwrap());
        // C s_34_5: const #3s : i
        let s_34_5: i128 = 3;
        // C s_34_6: add s_34_5 s_34_4
        let s_34_6: i128 = (s_34_5 + s_34_4);
        // D s_34_7: bit-extract s_34_2 s_34_0 s_34_6
        let s_34_7: Bits = (Bits::new(
            ((s_34_2) >> (s_34_0)).value(),
            u16::try_from(s_34_6).unwrap(),
        ));
        // D s_34_8: cast reint s_34_7 -> u8
        let s_34_8: u8 = (s_34_7.value() as u8);
        // D s_34_9: write-var domain <= s_34_8
        fn_state.domain = s_34_8;
        // C s_34_10: const #3s : i
        let s_34_10: i128 = 3;
        // D s_34_11: read-var descriptor:u32
        let s_34_11: u32 = fn_state.descriptor;
        // D s_34_12: cast zx s_34_11 -> bv
        let s_34_12: Bits = Bits::new(s_34_11 as u128, 32u16);
        // C s_34_13: const #1u : u64
        let s_34_13: u64 = 1;
        // D s_34_14: bit-extract s_34_12 s_34_10 s_34_13
        let s_34_14: Bits = (Bits::new(
            ((s_34_12) >> (s_34_10)).value(),
            u16::try_from(s_34_13).unwrap(),
        ));
        // D s_34_15: cast reint s_34_14 -> u8
        let s_34_15: bool = ((s_34_14.value()) != 0);
        // C s_34_16: const #0s : i
        let s_34_16: i128 = 0;
        // C s_34_17: const #0u : u64
        let s_34_17: u64 = 0;
        // D s_34_18: cast zx s_34_15 -> u64
        let s_34_18: u64 = (s_34_15 as u64);
        // C s_34_19: const #1u : u64
        let s_34_19: u64 = 1;
        // D s_34_20: and s_34_18 s_34_19
        let s_34_20: u64 = ((s_34_18) & (s_34_19));
        // D s_34_21: cmp-eq s_34_20 s_34_19
        let s_34_21: bool = ((s_34_20) == (s_34_19));
        // D s_34_22: lsl s_34_18 s_34_16
        let s_34_22: u64 = s_34_18 << s_34_16;
        // D s_34_23: or s_34_17 s_34_22
        let s_34_23: u64 = ((s_34_17) | (s_34_22));
        // D s_34_24: cmpl s_34_22
        let s_34_24: u64 = !s_34_22;
        // D s_34_25: and s_34_17 s_34_24
        let s_34_25: u64 = ((s_34_17) & (s_34_24));
        // D s_34_26: select s_34_21 s_34_23 s_34_25
        let s_34_26: u64 = if s_34_21 { s_34_23 } else { s_34_25 };
        // D s_34_27: cast trunc s_34_26 -> u8
        let s_34_27: bool = ((s_34_26) != 0);
        // D s_34_28: write-var ns <= s_34_27
        fn_state.ns = s_34_27;
        // C s_34_29: const #2s : i
        let s_34_29: i128 = 2;
        // D s_34_30: read-var descriptor:u32
        let s_34_30: u32 = fn_state.descriptor;
        // D s_34_31: cast zx s_34_30 -> bv
        let s_34_31: Bits = Bits::new(s_34_30 as u128, 32u16);
        // C s_34_32: const #1u : u64
        let s_34_32: u64 = 1;
        // D s_34_33: bit-extract s_34_31 s_34_29 s_34_32
        let s_34_33: Bits = (Bits::new(
            ((s_34_31) >> (s_34_29)).value(),
            u16::try_from(s_34_32).unwrap(),
        ));
        // D s_34_34: cast reint s_34_33 -> u8
        let s_34_34: bool = ((s_34_33.value()) != 0);
        // C s_34_35: const #0s : i
        let s_34_35: i128 = 0;
        // C s_34_36: const #0u : u64
        let s_34_36: u64 = 0;
        // D s_34_37: cast zx s_34_34 -> u64
        let s_34_37: u64 = (s_34_34 as u64);
        // C s_34_38: const #1u : u64
        let s_34_38: u64 = 1;
        // D s_34_39: and s_34_37 s_34_38
        let s_34_39: u64 = ((s_34_37) & (s_34_38));
        // D s_34_40: cmp-eq s_34_39 s_34_38
        let s_34_40: bool = ((s_34_39) == (s_34_38));
        // D s_34_41: lsl s_34_37 s_34_35
        let s_34_41: u64 = s_34_37 << s_34_35;
        // D s_34_42: or s_34_36 s_34_41
        let s_34_42: u64 = ((s_34_36) | (s_34_41));
        // D s_34_43: cmpl s_34_41
        let s_34_43: u64 = !s_34_41;
        // D s_34_44: and s_34_36 s_34_43
        let s_34_44: u64 = ((s_34_36) & (s_34_43));
        // D s_34_45: select s_34_40 s_34_42 s_34_44
        let s_34_45: u64 = if s_34_40 { s_34_42 } else { s_34_44 };
        // D s_34_46: cast trunc s_34_45 -> u8
        let s_34_46: bool = ((s_34_45) != 0);
        // D s_34_47: write-var pxn <= s_34_46
        fn_state.pxn = s_34_46;
        // C s_34_48: const #56s : i
        let s_34_48: i128 = 56;
        // C s_34_49: const #31s : i
        let s_34_49: i128 = 31;
        // C s_34_50: const #10s : i
        let s_34_50: i128 = 10;
        // C s_34_51: const #10s : i
        let s_34_51: i128 = 10;
        // D s_34_52: read-var descriptor:u32
        let s_34_52: u32 = fn_state.descriptor;
        // D s_34_53: cast zx s_34_52 -> bv
        let s_34_53: Bits = Bits::new(s_34_52 as u128, 32u16);
        // D s_34_54: call place_subrange(s_34_48, s_34_53, s_34_49, s_34_50, s_34_51)
        let s_34_54: Bits = place_subrange(
            state,
            tracer,
            s_34_48,
            s_34_53,
            s_34_49,
            s_34_50,
            s_34_51,
        );
        // D s_34_55: cast reint s_34_54 -> u56
        let s_34_55: u64 = (s_34_54.value() as u64);
        // D s_34_56: write-var walkstate.0.0 <= s_34_55
        fn_state.walkstate._0._0 = s_34_55;
        // C s_34_57: const #2s : i
        let s_34_57: i128 = 2;
        // D s_34_58: write-var walkstate.6 <= s_34_57
        fn_state.walkstate._6 = s_34_57;
        // N s_34_59: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_35_0: read-var walkstate.12:struct
        let s_35_0: u32 = fn_state.walkstate._12;
        // C s_35_1: const #0u : u32
        let s_35_1: u32 = 0;
        // D s_35_2: cmp-eq s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) == (s_35_1));
        // N s_35_3: branch s_35_2 b36 b23
        if s_35_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_36_0: read-var afe:u8
        let s_36_0: bool = fn_state.afe;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 1u16);
        // C s_36_2: const #1u : u8
        let s_36_2: bool = true;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // N s_36_5: branch s_36_4 b55 b37
        if s_36_4 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#28959 <= s_37_0
        fn_state.gs_28959 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_38_0: read-var gs#28959:u8
        let s_38_0: bool = fn_state.gs_28959;
        // N s_38_1: branch s_38_0 b54 b39
        if s_38_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_39_0: read-var tre:u8
        let s_39_0: bool = fn_state.tre;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // N s_39_5: branch s_39_4 b53 b40
        if s_39_4 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call RemapRegsHaveResetValues(s_40_0)
        let s_40_1: bool = RemapRegsHaveResetValues(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b52 b41
        if s_40_1 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_41_0: const #"" : str
        let s_41_0: &'static str = "";
        // S s_41_1: call __IMPDEF_MemoryAttributes(s_41_0)
        let s_41_1: ProductTypef170cab34335b70c = u__IMPDEF_MemoryAttributes(
            state,
            tracer,
            s_41_0,
        );
        // D s_41_2: write-var walkstate.7 <= s_41_1
        fn_state.walkstate._7 = s_41_1;
        // N s_41_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_42_0: read-var ap:u8
        let s_42_0: u8 = fn_state.ap;
        // D s_42_1: write-var walkstate.9.0 <= s_42_0
        fn_state.walkstate._9._0 = s_42_0;
        // D s_42_2: read-var xn:u8
        let s_42_2: bool = fn_state.xn;
        // D s_42_3: write-var walkstate.9.17 <= s_42_2
        fn_state.walkstate._9._17 = s_42_2;
        // D s_42_4: read-var pxn:u8
        let s_42_4: bool = fn_state.pxn;
        // D s_42_5: write-var walkstate.9.5 <= s_42_4
        fn_state.walkstate._9._5 = s_42_4;
        // D s_42_6: read-var domain:u8
        let s_42_6: u8 = fn_state.domain;
        // D s_42_7: write-var walkstate.3 <= s_42_6
        fn_state.walkstate._3 = s_42_6;
        // D s_42_8: read-var nG:u8
        let s_42_8: bool = fn_state.nG;
        // D s_42_9: write-var walkstate.8 <= s_42_8
        fn_state.walkstate._8 = s_42_8;
        // D s_42_10: read-var accdesc.25:struct
        let s_42_10: u32 = fn_state.accdesc._25;
        // C s_42_11: const #3u : u32
        let s_42_11: u32 = 3;
        // D s_42_12: cmp-eq s_42_10 s_42_11
        let s_42_12: bool = ((s_42_10) == (s_42_11));
        // N s_42_13: branch s_42_12 b51 b43
        if s_42_12 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#28960 <= s_43_0
        fn_state.gs_28960 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_44_0: read-var gs#28960:u8
        let s_44_0: bool = fn_state.gs_28960;
        // N s_44_1: branch s_44_0 b50 b45
        if s_44_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_45_0: const #0u : u32
        let s_45_0: u32 = 0;
        // D s_45_1: write-var walkstate.0.1 <= s_45_0
        fn_state.walkstate._0._1 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_46_0: const #19088u : u32
        let s_46_0: u32 = 19088;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: bool = {
            let value = state.read_register::<bool>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // N s_46_2: branch s_46_1 b49 b47
        if s_46_1 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_47_0: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_48_0: read-var fault:struct
        let s_48_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_48_1: read-var walkstate:struct
        let s_48_1: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_48_2: create-product struct = ["s_48_0", "s_48_1"]
        let s_48_2: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_48_0,
            _1: s_48_1,
        };
        // D s_48_3: write-var return_value <= s_48_2
        fn_state.return_value = s_48_2;
        // N s_48_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_49_0: read-var walkstate.7:struct
        let s_49_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_49_1: write-var ga#22353 <= s_49_0
        fn_state.ga_22353 = s_49_0;
        // D s_49_2: read-var ga#22353.7:struct
        let s_49_2: bool = fn_state.ga_22353._7;
        // D s_49_3: write-var tlbcontext.14 <= s_49_2
        fn_state.tlbcontext._14 = s_49_2;
        // D s_49_4: read-var walkstate.8:struct
        let s_49_4: bool = fn_state.walkstate._8;
        // D s_49_5: write-var tlbcontext.9 <= s_49_4
        fn_state.tlbcontext._9 = s_49_4;
        // D s_49_6: read-var tlbcontext:struct
        let s_49_6: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_49_7: write-var tlbrecord.1 <= s_49_6
        fn_state.tlbrecord._1 = s_49_6;
        // D s_49_8: read-var walkstate:struct
        let s_49_8: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_49_9: write-var tlbrecord.5 <= s_49_8
        fn_state.tlbrecord._5 = s_49_8;
        // D s_49_10: read-var walkstate.12:struct
        let s_49_10: u32 = fn_state.walkstate._12;
        // D s_49_11: call AArch32_TranslationSizeSD(s_49_10)
        let s_49_11: i128 = AArch32_TranslationSizeSD(state, tracer, s_49_10);
        // D s_49_12: write-var tlbrecord.0 <= s_49_11
        fn_state.tlbrecord._0 = s_49_11;
        // C s_49_13: const #0s : i
        let s_49_13: i128 = 0;
        // D s_49_14: write-var tlbrecord.2 <= s_49_13
        fn_state.tlbrecord._2 = s_49_13;
        // D s_49_15: read-var tlbrecord:struct
        let s_49_15: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_49_16: call S1TLBCache(s_49_15)
        let s_49_16: () = S1TLBCache(state, tracer, s_49_15);
        // N s_49_17: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_50_0: const #1u : u32
        let s_50_0: u32 = 1;
        // D s_50_1: write-var walkstate.0.1 <= s_50_0
        fn_state.walkstate._0._1 = s_50_0;
        // N s_50_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_51_0: read-var ns:u8
        let s_51_0: bool = fn_state.ns;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #0u : u8
        let s_51_2: bool = false;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#28960 <= s_51_4
        fn_state.gs_28960 = s_51_4;
        // N s_51_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_52_0: read-var tex:u8
        let s_52_0: u8 = fn_state.tex;
        // D s_52_1: read-var c:u8
        let s_52_1: bool = fn_state.c;
        // D s_52_2: read-var b:u8
        let s_52_2: bool = fn_state.b;
        // D s_52_3: read-var s:u8
        let s_52_3: bool = fn_state.s;
        // D s_52_4: call AArch32_DefaultTEXDecode(s_52_0, s_52_1, s_52_2, s_52_3)
        let s_52_4: ProductTypef170cab34335b70c = AArch32_DefaultTEXDecode(
            state,
            tracer,
            s_52_0,
            s_52_1,
            s_52_2,
            s_52_3,
        );
        // D s_52_5: write-var walkstate.7 <= s_52_4
        fn_state.walkstate._7 = s_52_4;
        // N s_52_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_53_0: read-var regime:u32
        let s_53_0: u32 = fn_state.regime;
        // D s_53_1: read-var tex:u8
        let s_53_1: u8 = fn_state.tex;
        // D s_53_2: read-var c:u8
        let s_53_2: bool = fn_state.c;
        // D s_53_3: read-var b:u8
        let s_53_3: bool = fn_state.b;
        // D s_53_4: read-var s:u8
        let s_53_4: bool = fn_state.s;
        // D s_53_5: call AArch32_RemappedTEXDecode(s_53_0, s_53_1, s_53_2, s_53_3, s_53_4)
        let s_53_5: ProductTypef170cab34335b70c = AArch32_RemappedTEXDecode(
            state,
            tracer,
            s_53_0,
            s_53_1,
            s_53_2,
            s_53_3,
            s_53_4,
        );
        // D s_53_6: write-var walkstate.7 <= s_53_5
        fn_state.walkstate._7 = s_53_5;
        // N s_53_7: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_54_0: read-var domain:u8
        let s_54_0: u8 = fn_state.domain;
        // D s_54_1: write-var fault.4 <= s_54_0
        fn_state.fault._4 = s_54_0;
        // C s_54_2: const #1u : u32
        let s_54_2: u32 = 1;
        // D s_54_3: write-var fault.16 <= s_54_2
        fn_state.fault._16 = s_54_2;
        // C s_54_4: const #() : ()
        let s_54_4: () = ();
        // S s_54_5: call __UNKNOWN_TTWState(s_54_4)
        let s_54_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_54_4,
        );
        // D s_54_6: read-var fault:struct
        let s_54_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_54_7: create-product struct = ["s_54_6", "s_54_5"]
        let s_54_7: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_54_6,
            _1: s_54_5,
        };
        // D s_54_8: write-var return_value <= s_54_7
        fn_state.return_value = s_54_7;
        // N s_54_9: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_55_0: const #0s : i
        let s_55_0: i128 = 0;
        // D s_55_1: read-var ap:u8
        let s_55_1: u8 = fn_state.ap;
        // D s_55_2: cast zx s_55_1 -> bv
        let s_55_2: Bits = Bits::new(s_55_1 as u128, 3u16);
        // C s_55_3: const #1u : u64
        let s_55_3: u64 = 1;
        // D s_55_4: bit-extract s_55_2 s_55_0 s_55_3
        let s_55_4: Bits = (Bits::new(
            ((s_55_2) >> (s_55_0)).value(),
            u16::try_from(s_55_3).unwrap(),
        ));
        // D s_55_5: cast reint s_55_4 -> u8
        let s_55_5: bool = ((s_55_4.value()) != 0);
        // C s_55_6: const #0s : i
        let s_55_6: i128 = 0;
        // C s_55_7: const #0u : u64
        let s_55_7: u64 = 0;
        // D s_55_8: cast zx s_55_5 -> u64
        let s_55_8: u64 = (s_55_5 as u64);
        // C s_55_9: const #1u : u64
        let s_55_9: u64 = 1;
        // D s_55_10: and s_55_8 s_55_9
        let s_55_10: u64 = ((s_55_8) & (s_55_9));
        // D s_55_11: cmp-eq s_55_10 s_55_9
        let s_55_11: bool = ((s_55_10) == (s_55_9));
        // D s_55_12: lsl s_55_8 s_55_6
        let s_55_12: u64 = s_55_8 << s_55_6;
        // D s_55_13: or s_55_7 s_55_12
        let s_55_13: u64 = ((s_55_7) | (s_55_12));
        // D s_55_14: cmpl s_55_12
        let s_55_14: u64 = !s_55_12;
        // D s_55_15: and s_55_7 s_55_14
        let s_55_15: u64 = ((s_55_7) & (s_55_14));
        // D s_55_16: select s_55_11 s_55_13 s_55_15
        let s_55_16: u64 = if s_55_11 { s_55_13 } else { s_55_15 };
        // D s_55_17: cast trunc s_55_16 -> u8
        let s_55_17: bool = ((s_55_16) != 0);
        // D s_55_18: cast zx s_55_17 -> bv
        let s_55_18: Bits = Bits::new(s_55_17 as u128, 1u16);
        // C s_55_19: const #0u : u8
        let s_55_19: bool = false;
        // C s_55_20: cast zx s_55_19 -> bv
        let s_55_20: Bits = Bits::new(s_55_19 as u128, 1u16);
        // D s_55_21: cmp-eq s_55_18 s_55_20
        let s_55_21: bool = ((s_55_18) == (s_55_20));
        // D s_55_22: write-var gs#28959 <= s_55_21
        fn_state.gs_28959 = s_55_21;
        // N s_55_23: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_56_0: const #5u : u32
        let s_56_0: u32 = 5;
        // D s_56_1: read-var ga#22286:u32
        let s_56_1: u32 = fn_state.ga_22286;
        // D s_56_2: cmp-eq s_56_0 s_56_1
        let s_56_2: bool = ((s_56_0) == (s_56_1));
        // D s_56_3: not s_56_2
        let s_56_3: bool = !s_56_2;
        // N s_56_4: branch s_56_3 b58 b57
        if s_56_3 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_57_0: const #11s : i
        let s_57_0: i128 = 11;
        // D s_57_1: read-var descriptor:u32
        let s_57_1: u32 = fn_state.descriptor;
        // D s_57_2: cast zx s_57_1 -> bv
        let s_57_2: Bits = Bits::new(s_57_1 as u128, 32u16);
        // C s_57_3: const #1u : u64
        let s_57_3: u64 = 1;
        // D s_57_4: bit-extract s_57_2 s_57_0 s_57_3
        let s_57_4: Bits = (Bits::new(
            ((s_57_2) >> (s_57_0)).value(),
            u16::try_from(s_57_3).unwrap(),
        ));
        // D s_57_5: cast reint s_57_4 -> u8
        let s_57_5: bool = ((s_57_4.value()) != 0);
        // C s_57_6: const #0s : i
        let s_57_6: i128 = 0;
        // C s_57_7: const #0u : u64
        let s_57_7: u64 = 0;
        // D s_57_8: cast zx s_57_5 -> u64
        let s_57_8: u64 = (s_57_5 as u64);
        // C s_57_9: const #1u : u64
        let s_57_9: u64 = 1;
        // D s_57_10: and s_57_8 s_57_9
        let s_57_10: u64 = ((s_57_8) & (s_57_9));
        // D s_57_11: cmp-eq s_57_10 s_57_9
        let s_57_11: bool = ((s_57_10) == (s_57_9));
        // D s_57_12: lsl s_57_8 s_57_6
        let s_57_12: u64 = s_57_8 << s_57_6;
        // D s_57_13: or s_57_7 s_57_12
        let s_57_13: u64 = ((s_57_7) | (s_57_12));
        // D s_57_14: cmpl s_57_12
        let s_57_14: u64 = !s_57_12;
        // D s_57_15: and s_57_7 s_57_14
        let s_57_15: u64 = ((s_57_7) & (s_57_14));
        // D s_57_16: select s_57_11 s_57_13 s_57_15
        let s_57_16: u64 = if s_57_11 { s_57_13 } else { s_57_15 };
        // D s_57_17: cast trunc s_57_16 -> u8
        let s_57_17: bool = ((s_57_16) != 0);
        // D s_57_18: write-var nG <= s_57_17
        fn_state.nG = s_57_17;
        // C s_57_19: const #10s : i
        let s_57_19: i128 = 10;
        // D s_57_20: read-var descriptor:u32
        let s_57_20: u32 = fn_state.descriptor;
        // D s_57_21: cast zx s_57_20 -> bv
        let s_57_21: Bits = Bits::new(s_57_20 as u128, 32u16);
        // C s_57_22: const #1u : u64
        let s_57_22: u64 = 1;
        // D s_57_23: bit-extract s_57_21 s_57_19 s_57_22
        let s_57_23: Bits = (Bits::new(
            ((s_57_21) >> (s_57_19)).value(),
            u16::try_from(s_57_22).unwrap(),
        ));
        // D s_57_24: cast reint s_57_23 -> u8
        let s_57_24: bool = ((s_57_23.value()) != 0);
        // C s_57_25: const #0s : i
        let s_57_25: i128 = 0;
        // C s_57_26: const #0u : u64
        let s_57_26: u64 = 0;
        // D s_57_27: cast zx s_57_24 -> u64
        let s_57_27: u64 = (s_57_24 as u64);
        // C s_57_28: const #1u : u64
        let s_57_28: u64 = 1;
        // D s_57_29: and s_57_27 s_57_28
        let s_57_29: u64 = ((s_57_27) & (s_57_28));
        // D s_57_30: cmp-eq s_57_29 s_57_28
        let s_57_30: bool = ((s_57_29) == (s_57_28));
        // D s_57_31: lsl s_57_27 s_57_25
        let s_57_31: u64 = s_57_27 << s_57_25;
        // D s_57_32: or s_57_26 s_57_31
        let s_57_32: u64 = ((s_57_26) | (s_57_31));
        // D s_57_33: cmpl s_57_31
        let s_57_33: u64 = !s_57_31;
        // D s_57_34: and s_57_26 s_57_33
        let s_57_34: u64 = ((s_57_26) & (s_57_33));
        // D s_57_35: select s_57_30 s_57_32 s_57_34
        let s_57_35: u64 = if s_57_30 { s_57_32 } else { s_57_34 };
        // D s_57_36: cast trunc s_57_35 -> u8
        let s_57_36: bool = ((s_57_35) != 0);
        // D s_57_37: write-var s <= s_57_36
        fn_state.s = s_57_36;
        // C s_57_38: const #9s : i
        let s_57_38: i128 = 9;
        // D s_57_39: read-var descriptor:u32
        let s_57_39: u32 = fn_state.descriptor;
        // D s_57_40: cast zx s_57_39 -> bv
        let s_57_40: Bits = Bits::new(s_57_39 as u128, 32u16);
        // C s_57_41: const #1u : u64
        let s_57_41: u64 = 1;
        // D s_57_42: bit-extract s_57_40 s_57_38 s_57_41
        let s_57_42: Bits = (Bits::new(
            ((s_57_40) >> (s_57_38)).value(),
            u16::try_from(s_57_41).unwrap(),
        ));
        // D s_57_43: cast reint s_57_42 -> u8
        let s_57_43: bool = ((s_57_42.value()) != 0);
        // C s_57_44: const #0s : i
        let s_57_44: i128 = 0;
        // C s_57_45: const #0u : u64
        let s_57_45: u64 = 0;
        // D s_57_46: cast zx s_57_43 -> u64
        let s_57_46: u64 = (s_57_43 as u64);
        // C s_57_47: const #1u : u64
        let s_57_47: u64 = 1;
        // D s_57_48: and s_57_46 s_57_47
        let s_57_48: u64 = ((s_57_46) & (s_57_47));
        // D s_57_49: cmp-eq s_57_48 s_57_47
        let s_57_49: bool = ((s_57_48) == (s_57_47));
        // D s_57_50: lsl s_57_46 s_57_44
        let s_57_50: u64 = s_57_46 << s_57_44;
        // D s_57_51: or s_57_45 s_57_50
        let s_57_51: u64 = ((s_57_45) | (s_57_50));
        // D s_57_52: cmpl s_57_50
        let s_57_52: u64 = !s_57_50;
        // D s_57_53: and s_57_45 s_57_52
        let s_57_53: u64 = ((s_57_45) & (s_57_52));
        // D s_57_54: select s_57_49 s_57_51 s_57_53
        let s_57_54: u64 = if s_57_49 { s_57_51 } else { s_57_53 };
        // D s_57_55: cast trunc s_57_54 -> u8
        let s_57_55: bool = ((s_57_54) != 0);
        // C s_57_56: const #4s : i
        let s_57_56: i128 = 4;
        // D s_57_57: read-var descriptor:u32
        let s_57_57: u32 = fn_state.descriptor;
        // D s_57_58: cast zx s_57_57 -> bv
        let s_57_58: Bits = Bits::new(s_57_57 as u128, 32u16);
        // C s_57_59: const #1s : i64
        let s_57_59: i64 = 1;
        // C s_57_60: cast zx s_57_59 -> i
        let s_57_60: i128 = (i128::try_from(s_57_59).unwrap());
        // C s_57_61: const #1s : i
        let s_57_61: i128 = 1;
        // C s_57_62: add s_57_61 s_57_60
        let s_57_62: i128 = (s_57_61 + s_57_60);
        // D s_57_63: bit-extract s_57_58 s_57_56 s_57_62
        let s_57_63: Bits = (Bits::new(
            ((s_57_58) >> (s_57_56)).value(),
            u16::try_from(s_57_62).unwrap(),
        ));
        // D s_57_64: cast reint s_57_63 -> u8
        let s_57_64: u8 = (s_57_63.value() as u8);
        // D s_57_65: cast zx s_57_55 -> bv
        let s_57_65: Bits = Bits::new(s_57_55 as u128, 1u16);
        // D s_57_66: cast zx s_57_64 -> bv
        let s_57_66: Bits = Bits::new(s_57_64 as u128, 2u16);
        // D s_57_67: cast reint s_57_65 -> u128
        let s_57_67: u128 = (s_57_65.value() as u128);
        // D s_57_68: size-of s_57_65
        let s_57_68: u16 = s_57_65.length();
        // D s_57_69: cast reint s_57_66 -> u128
        let s_57_69: u128 = (s_57_66.value() as u128);
        // D s_57_70: size-of s_57_66
        let s_57_70: u16 = s_57_66.length();
        // D s_57_71: lsl s_57_67 s_57_70
        let s_57_71: u128 = s_57_67 << s_57_70;
        // D s_57_72: or s_57_71 s_57_69
        let s_57_72: u128 = ((s_57_71) | (s_57_69));
        // D s_57_73: add s_57_68 s_57_70
        let s_57_73: u16 = (s_57_68 + s_57_70);
        // D s_57_74: create-bits s_57_72 s_57_73
        let s_57_74: Bits = Bits::new(s_57_72, s_57_73);
        // D s_57_75: cast reint s_57_74 -> u8
        let s_57_75: u8 = (s_57_74.value() as u8);
        // D s_57_76: write-var ap <= s_57_75
        fn_state.ap = s_57_75;
        // C s_57_77: const #6s : i
        let s_57_77: i128 = 6;
        // D s_57_78: read-var descriptor:u32
        let s_57_78: u32 = fn_state.descriptor;
        // D s_57_79: cast zx s_57_78 -> bv
        let s_57_79: Bits = Bits::new(s_57_78 as u128, 32u16);
        // C s_57_80: const #1s : i64
        let s_57_80: i64 = 1;
        // C s_57_81: cast zx s_57_80 -> i
        let s_57_81: i128 = (i128::try_from(s_57_80).unwrap());
        // C s_57_82: const #2s : i
        let s_57_82: i128 = 2;
        // C s_57_83: add s_57_82 s_57_81
        let s_57_83: i128 = (s_57_82 + s_57_81);
        // D s_57_84: bit-extract s_57_79 s_57_77 s_57_83
        let s_57_84: Bits = (Bits::new(
            ((s_57_79) >> (s_57_77)).value(),
            u16::try_from(s_57_83).unwrap(),
        ));
        // D s_57_85: cast reint s_57_84 -> u8
        let s_57_85: u8 = (s_57_84.value() as u8);
        // D s_57_86: write-var tex <= s_57_85
        fn_state.tex = s_57_85;
        // C s_57_87: const #3s : i
        let s_57_87: i128 = 3;
        // D s_57_88: read-var descriptor:u32
        let s_57_88: u32 = fn_state.descriptor;
        // D s_57_89: cast zx s_57_88 -> bv
        let s_57_89: Bits = Bits::new(s_57_88 as u128, 32u16);
        // C s_57_90: const #1u : u64
        let s_57_90: u64 = 1;
        // D s_57_91: bit-extract s_57_89 s_57_87 s_57_90
        let s_57_91: Bits = (Bits::new(
            ((s_57_89) >> (s_57_87)).value(),
            u16::try_from(s_57_90).unwrap(),
        ));
        // D s_57_92: cast reint s_57_91 -> u8
        let s_57_92: bool = ((s_57_91.value()) != 0);
        // C s_57_93: const #0s : i
        let s_57_93: i128 = 0;
        // C s_57_94: const #0u : u64
        let s_57_94: u64 = 0;
        // D s_57_95: cast zx s_57_92 -> u64
        let s_57_95: u64 = (s_57_92 as u64);
        // C s_57_96: const #1u : u64
        let s_57_96: u64 = 1;
        // D s_57_97: and s_57_95 s_57_96
        let s_57_97: u64 = ((s_57_95) & (s_57_96));
        // D s_57_98: cmp-eq s_57_97 s_57_96
        let s_57_98: bool = ((s_57_97) == (s_57_96));
        // D s_57_99: lsl s_57_95 s_57_93
        let s_57_99: u64 = s_57_95 << s_57_93;
        // D s_57_100: or s_57_94 s_57_99
        let s_57_100: u64 = ((s_57_94) | (s_57_99));
        // D s_57_101: cmpl s_57_99
        let s_57_101: u64 = !s_57_99;
        // D s_57_102: and s_57_94 s_57_101
        let s_57_102: u64 = ((s_57_94) & (s_57_101));
        // D s_57_103: select s_57_98 s_57_100 s_57_102
        let s_57_103: u64 = if s_57_98 { s_57_100 } else { s_57_102 };
        // D s_57_104: cast trunc s_57_103 -> u8
        let s_57_104: bool = ((s_57_103) != 0);
        // D s_57_105: write-var c <= s_57_104
        fn_state.c = s_57_104;
        // C s_57_106: const #2s : i
        let s_57_106: i128 = 2;
        // D s_57_107: read-var descriptor:u32
        let s_57_107: u32 = fn_state.descriptor;
        // D s_57_108: cast zx s_57_107 -> bv
        let s_57_108: Bits = Bits::new(s_57_107 as u128, 32u16);
        // C s_57_109: const #1u : u64
        let s_57_109: u64 = 1;
        // D s_57_110: bit-extract s_57_108 s_57_106 s_57_109
        let s_57_110: Bits = (Bits::new(
            ((s_57_108) >> (s_57_106)).value(),
            u16::try_from(s_57_109).unwrap(),
        ));
        // D s_57_111: cast reint s_57_110 -> u8
        let s_57_111: bool = ((s_57_110.value()) != 0);
        // C s_57_112: const #0s : i
        let s_57_112: i128 = 0;
        // C s_57_113: const #0u : u64
        let s_57_113: u64 = 0;
        // D s_57_114: cast zx s_57_111 -> u64
        let s_57_114: u64 = (s_57_111 as u64);
        // C s_57_115: const #1u : u64
        let s_57_115: u64 = 1;
        // D s_57_116: and s_57_114 s_57_115
        let s_57_116: u64 = ((s_57_114) & (s_57_115));
        // D s_57_117: cmp-eq s_57_116 s_57_115
        let s_57_117: bool = ((s_57_116) == (s_57_115));
        // D s_57_118: lsl s_57_114 s_57_112
        let s_57_118: u64 = s_57_114 << s_57_112;
        // D s_57_119: or s_57_113 s_57_118
        let s_57_119: u64 = ((s_57_113) | (s_57_118));
        // D s_57_120: cmpl s_57_118
        let s_57_120: u64 = !s_57_118;
        // D s_57_121: and s_57_113 s_57_120
        let s_57_121: u64 = ((s_57_113) & (s_57_120));
        // D s_57_122: select s_57_117 s_57_119 s_57_121
        let s_57_122: u64 = if s_57_117 { s_57_119 } else { s_57_121 };
        // D s_57_123: cast trunc s_57_122 -> u8
        let s_57_123: bool = ((s_57_122) != 0);
        // D s_57_124: write-var b <= s_57_123
        fn_state.b = s_57_123;
        // C s_57_125: const #0s : i
        let s_57_125: i128 = 0;
        // D s_57_126: read-var descriptor:u32
        let s_57_126: u32 = fn_state.descriptor;
        // D s_57_127: cast zx s_57_126 -> bv
        let s_57_127: Bits = Bits::new(s_57_126 as u128, 32u16);
        // C s_57_128: const #1u : u64
        let s_57_128: u64 = 1;
        // D s_57_129: bit-extract s_57_127 s_57_125 s_57_128
        let s_57_129: Bits = (Bits::new(
            ((s_57_127) >> (s_57_125)).value(),
            u16::try_from(s_57_128).unwrap(),
        ));
        // D s_57_130: cast reint s_57_129 -> u8
        let s_57_130: bool = ((s_57_129.value()) != 0);
        // C s_57_131: const #0s : i
        let s_57_131: i128 = 0;
        // C s_57_132: const #0u : u64
        let s_57_132: u64 = 0;
        // D s_57_133: cast zx s_57_130 -> u64
        let s_57_133: u64 = (s_57_130 as u64);
        // C s_57_134: const #1u : u64
        let s_57_134: u64 = 1;
        // D s_57_135: and s_57_133 s_57_134
        let s_57_135: u64 = ((s_57_133) & (s_57_134));
        // D s_57_136: cmp-eq s_57_135 s_57_134
        let s_57_136: bool = ((s_57_135) == (s_57_134));
        // D s_57_137: lsl s_57_133 s_57_131
        let s_57_137: u64 = s_57_133 << s_57_131;
        // D s_57_138: or s_57_132 s_57_137
        let s_57_138: u64 = ((s_57_132) | (s_57_137));
        // D s_57_139: cmpl s_57_137
        let s_57_139: u64 = !s_57_137;
        // D s_57_140: and s_57_132 s_57_139
        let s_57_140: u64 = ((s_57_132) & (s_57_139));
        // D s_57_141: select s_57_136 s_57_138 s_57_140
        let s_57_141: u64 = if s_57_136 { s_57_138 } else { s_57_140 };
        // D s_57_142: cast trunc s_57_141 -> u8
        let s_57_142: bool = ((s_57_141) != 0);
        // D s_57_143: write-var xn <= s_57_142
        fn_state.xn = s_57_142;
        // C s_57_144: const #56s : i
        let s_57_144: i128 = 56;
        // C s_57_145: const #31s : i
        let s_57_145: i128 = 31;
        // C s_57_146: const #12s : i
        let s_57_146: i128 = 12;
        // C s_57_147: const #12s : i
        let s_57_147: i128 = 12;
        // D s_57_148: read-var descriptor:u32
        let s_57_148: u32 = fn_state.descriptor;
        // D s_57_149: cast zx s_57_148 -> bv
        let s_57_149: Bits = Bits::new(s_57_148 as u128, 32u16);
        // D s_57_150: call place_subrange(s_57_144, s_57_149, s_57_145, s_57_146, s_57_147)
        let s_57_150: Bits = place_subrange(
            state,
            tracer,
            s_57_144,
            s_57_149,
            s_57_145,
            s_57_146,
            s_57_147,
        );
        // D s_57_151: cast reint s_57_150 -> u56
        let s_57_151: u64 = (s_57_150.value() as u64);
        // D s_57_152: write-var walkstate.0.0 <= s_57_151
        fn_state.walkstate._0._0 = s_57_151;
        // C s_57_153: const #0u : u8
        let s_57_153: bool = false;
        // D s_57_154: write-var walkstate.5 <= s_57_153
        fn_state.walkstate._5 = s_57_153;
        // N s_57_155: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_58_0: const #4u : u32
        let s_58_0: u32 = 4;
        // D s_58_1: read-var ga#22286:u32
        let s_58_1: u32 = fn_state.ga_22286;
        // D s_58_2: cmp-eq s_58_0 s_58_1
        let s_58_2: bool = ((s_58_0) == (s_58_1));
        // D s_58_3: not s_58_2
        let s_58_3: bool = !s_58_2;
        // N s_58_4: branch s_58_3 b60 b59
        if s_58_3 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_59_0: const #15s : i
        let s_59_0: i128 = 15;
        // D s_59_1: read-var descriptor:u32
        let s_59_1: u32 = fn_state.descriptor;
        // D s_59_2: cast zx s_59_1 -> bv
        let s_59_2: Bits = Bits::new(s_59_1 as u128, 32u16);
        // C s_59_3: const #1u : u64
        let s_59_3: u64 = 1;
        // D s_59_4: bit-extract s_59_2 s_59_0 s_59_3
        let s_59_4: Bits = (Bits::new(
            ((s_59_2) >> (s_59_0)).value(),
            u16::try_from(s_59_3).unwrap(),
        ));
        // D s_59_5: cast reint s_59_4 -> u8
        let s_59_5: bool = ((s_59_4.value()) != 0);
        // C s_59_6: const #0s : i
        let s_59_6: i128 = 0;
        // C s_59_7: const #0u : u64
        let s_59_7: u64 = 0;
        // D s_59_8: cast zx s_59_5 -> u64
        let s_59_8: u64 = (s_59_5 as u64);
        // C s_59_9: const #1u : u64
        let s_59_9: u64 = 1;
        // D s_59_10: and s_59_8 s_59_9
        let s_59_10: u64 = ((s_59_8) & (s_59_9));
        // D s_59_11: cmp-eq s_59_10 s_59_9
        let s_59_11: bool = ((s_59_10) == (s_59_9));
        // D s_59_12: lsl s_59_8 s_59_6
        let s_59_12: u64 = s_59_8 << s_59_6;
        // D s_59_13: or s_59_7 s_59_12
        let s_59_13: u64 = ((s_59_7) | (s_59_12));
        // D s_59_14: cmpl s_59_12
        let s_59_14: u64 = !s_59_12;
        // D s_59_15: and s_59_7 s_59_14
        let s_59_15: u64 = ((s_59_7) & (s_59_14));
        // D s_59_16: select s_59_11 s_59_13 s_59_15
        let s_59_16: u64 = if s_59_11 { s_59_13 } else { s_59_15 };
        // D s_59_17: cast trunc s_59_16 -> u8
        let s_59_17: bool = ((s_59_16) != 0);
        // D s_59_18: write-var xn <= s_59_17
        fn_state.xn = s_59_17;
        // C s_59_19: const #12s : i
        let s_59_19: i128 = 12;
        // D s_59_20: read-var descriptor:u32
        let s_59_20: u32 = fn_state.descriptor;
        // D s_59_21: cast zx s_59_20 -> bv
        let s_59_21: Bits = Bits::new(s_59_20 as u128, 32u16);
        // C s_59_22: const #1s : i64
        let s_59_22: i64 = 1;
        // C s_59_23: cast zx s_59_22 -> i
        let s_59_23: i128 = (i128::try_from(s_59_22).unwrap());
        // C s_59_24: const #2s : i
        let s_59_24: i128 = 2;
        // C s_59_25: add s_59_24 s_59_23
        let s_59_25: i128 = (s_59_24 + s_59_23);
        // D s_59_26: bit-extract s_59_21 s_59_19 s_59_25
        let s_59_26: Bits = (Bits::new(
            ((s_59_21) >> (s_59_19)).value(),
            u16::try_from(s_59_25).unwrap(),
        ));
        // D s_59_27: cast reint s_59_26 -> u8
        let s_59_27: u8 = (s_59_26.value() as u8);
        // D s_59_28: write-var tex <= s_59_27
        fn_state.tex = s_59_27;
        // C s_59_29: const #11s : i
        let s_59_29: i128 = 11;
        // D s_59_30: read-var descriptor:u32
        let s_59_30: u32 = fn_state.descriptor;
        // D s_59_31: cast zx s_59_30 -> bv
        let s_59_31: Bits = Bits::new(s_59_30 as u128, 32u16);
        // C s_59_32: const #1u : u64
        let s_59_32: u64 = 1;
        // D s_59_33: bit-extract s_59_31 s_59_29 s_59_32
        let s_59_33: Bits = (Bits::new(
            ((s_59_31) >> (s_59_29)).value(),
            u16::try_from(s_59_32).unwrap(),
        ));
        // D s_59_34: cast reint s_59_33 -> u8
        let s_59_34: bool = ((s_59_33.value()) != 0);
        // C s_59_35: const #0s : i
        let s_59_35: i128 = 0;
        // C s_59_36: const #0u : u64
        let s_59_36: u64 = 0;
        // D s_59_37: cast zx s_59_34 -> u64
        let s_59_37: u64 = (s_59_34 as u64);
        // C s_59_38: const #1u : u64
        let s_59_38: u64 = 1;
        // D s_59_39: and s_59_37 s_59_38
        let s_59_39: u64 = ((s_59_37) & (s_59_38));
        // D s_59_40: cmp-eq s_59_39 s_59_38
        let s_59_40: bool = ((s_59_39) == (s_59_38));
        // D s_59_41: lsl s_59_37 s_59_35
        let s_59_41: u64 = s_59_37 << s_59_35;
        // D s_59_42: or s_59_36 s_59_41
        let s_59_42: u64 = ((s_59_36) | (s_59_41));
        // D s_59_43: cmpl s_59_41
        let s_59_43: u64 = !s_59_41;
        // D s_59_44: and s_59_36 s_59_43
        let s_59_44: u64 = ((s_59_36) & (s_59_43));
        // D s_59_45: select s_59_40 s_59_42 s_59_44
        let s_59_45: u64 = if s_59_40 { s_59_42 } else { s_59_44 };
        // D s_59_46: cast trunc s_59_45 -> u8
        let s_59_46: bool = ((s_59_45) != 0);
        // D s_59_47: write-var nG <= s_59_46
        fn_state.nG = s_59_46;
        // C s_59_48: const #10s : i
        let s_59_48: i128 = 10;
        // D s_59_49: read-var descriptor:u32
        let s_59_49: u32 = fn_state.descriptor;
        // D s_59_50: cast zx s_59_49 -> bv
        let s_59_50: Bits = Bits::new(s_59_49 as u128, 32u16);
        // C s_59_51: const #1u : u64
        let s_59_51: u64 = 1;
        // D s_59_52: bit-extract s_59_50 s_59_48 s_59_51
        let s_59_52: Bits = (Bits::new(
            ((s_59_50) >> (s_59_48)).value(),
            u16::try_from(s_59_51).unwrap(),
        ));
        // D s_59_53: cast reint s_59_52 -> u8
        let s_59_53: bool = ((s_59_52.value()) != 0);
        // C s_59_54: const #0s : i
        let s_59_54: i128 = 0;
        // C s_59_55: const #0u : u64
        let s_59_55: u64 = 0;
        // D s_59_56: cast zx s_59_53 -> u64
        let s_59_56: u64 = (s_59_53 as u64);
        // C s_59_57: const #1u : u64
        let s_59_57: u64 = 1;
        // D s_59_58: and s_59_56 s_59_57
        let s_59_58: u64 = ((s_59_56) & (s_59_57));
        // D s_59_59: cmp-eq s_59_58 s_59_57
        let s_59_59: bool = ((s_59_58) == (s_59_57));
        // D s_59_60: lsl s_59_56 s_59_54
        let s_59_60: u64 = s_59_56 << s_59_54;
        // D s_59_61: or s_59_55 s_59_60
        let s_59_61: u64 = ((s_59_55) | (s_59_60));
        // D s_59_62: cmpl s_59_60
        let s_59_62: u64 = !s_59_60;
        // D s_59_63: and s_59_55 s_59_62
        let s_59_63: u64 = ((s_59_55) & (s_59_62));
        // D s_59_64: select s_59_59 s_59_61 s_59_63
        let s_59_64: u64 = if s_59_59 { s_59_61 } else { s_59_63 };
        // D s_59_65: cast trunc s_59_64 -> u8
        let s_59_65: bool = ((s_59_64) != 0);
        // D s_59_66: write-var s <= s_59_65
        fn_state.s = s_59_65;
        // C s_59_67: const #9s : i
        let s_59_67: i128 = 9;
        // D s_59_68: read-var descriptor:u32
        let s_59_68: u32 = fn_state.descriptor;
        // D s_59_69: cast zx s_59_68 -> bv
        let s_59_69: Bits = Bits::new(s_59_68 as u128, 32u16);
        // C s_59_70: const #1u : u64
        let s_59_70: u64 = 1;
        // D s_59_71: bit-extract s_59_69 s_59_67 s_59_70
        let s_59_71: Bits = (Bits::new(
            ((s_59_69) >> (s_59_67)).value(),
            u16::try_from(s_59_70).unwrap(),
        ));
        // D s_59_72: cast reint s_59_71 -> u8
        let s_59_72: bool = ((s_59_71.value()) != 0);
        // C s_59_73: const #0s : i
        let s_59_73: i128 = 0;
        // C s_59_74: const #0u : u64
        let s_59_74: u64 = 0;
        // D s_59_75: cast zx s_59_72 -> u64
        let s_59_75: u64 = (s_59_72 as u64);
        // C s_59_76: const #1u : u64
        let s_59_76: u64 = 1;
        // D s_59_77: and s_59_75 s_59_76
        let s_59_77: u64 = ((s_59_75) & (s_59_76));
        // D s_59_78: cmp-eq s_59_77 s_59_76
        let s_59_78: bool = ((s_59_77) == (s_59_76));
        // D s_59_79: lsl s_59_75 s_59_73
        let s_59_79: u64 = s_59_75 << s_59_73;
        // D s_59_80: or s_59_74 s_59_79
        let s_59_80: u64 = ((s_59_74) | (s_59_79));
        // D s_59_81: cmpl s_59_79
        let s_59_81: u64 = !s_59_79;
        // D s_59_82: and s_59_74 s_59_81
        let s_59_82: u64 = ((s_59_74) & (s_59_81));
        // D s_59_83: select s_59_78 s_59_80 s_59_82
        let s_59_83: u64 = if s_59_78 { s_59_80 } else { s_59_82 };
        // D s_59_84: cast trunc s_59_83 -> u8
        let s_59_84: bool = ((s_59_83) != 0);
        // C s_59_85: const #4s : i
        let s_59_85: i128 = 4;
        // D s_59_86: read-var descriptor:u32
        let s_59_86: u32 = fn_state.descriptor;
        // D s_59_87: cast zx s_59_86 -> bv
        let s_59_87: Bits = Bits::new(s_59_86 as u128, 32u16);
        // C s_59_88: const #1s : i64
        let s_59_88: i64 = 1;
        // C s_59_89: cast zx s_59_88 -> i
        let s_59_89: i128 = (i128::try_from(s_59_88).unwrap());
        // C s_59_90: const #1s : i
        let s_59_90: i128 = 1;
        // C s_59_91: add s_59_90 s_59_89
        let s_59_91: i128 = (s_59_90 + s_59_89);
        // D s_59_92: bit-extract s_59_87 s_59_85 s_59_91
        let s_59_92: Bits = (Bits::new(
            ((s_59_87) >> (s_59_85)).value(),
            u16::try_from(s_59_91).unwrap(),
        ));
        // D s_59_93: cast reint s_59_92 -> u8
        let s_59_93: u8 = (s_59_92.value() as u8);
        // D s_59_94: cast zx s_59_84 -> bv
        let s_59_94: Bits = Bits::new(s_59_84 as u128, 1u16);
        // D s_59_95: cast zx s_59_93 -> bv
        let s_59_95: Bits = Bits::new(s_59_93 as u128, 2u16);
        // D s_59_96: cast reint s_59_94 -> u128
        let s_59_96: u128 = (s_59_94.value() as u128);
        // D s_59_97: size-of s_59_94
        let s_59_97: u16 = s_59_94.length();
        // D s_59_98: cast reint s_59_95 -> u128
        let s_59_98: u128 = (s_59_95.value() as u128);
        // D s_59_99: size-of s_59_95
        let s_59_99: u16 = s_59_95.length();
        // D s_59_100: lsl s_59_96 s_59_99
        let s_59_100: u128 = s_59_96 << s_59_99;
        // D s_59_101: or s_59_100 s_59_98
        let s_59_101: u128 = ((s_59_100) | (s_59_98));
        // D s_59_102: add s_59_97 s_59_99
        let s_59_102: u16 = (s_59_97 + s_59_99);
        // D s_59_103: create-bits s_59_101 s_59_102
        let s_59_103: Bits = Bits::new(s_59_101, s_59_102);
        // D s_59_104: cast reint s_59_103 -> u8
        let s_59_104: u8 = (s_59_103.value() as u8);
        // D s_59_105: write-var ap <= s_59_104
        fn_state.ap = s_59_104;
        // C s_59_106: const #3s : i
        let s_59_106: i128 = 3;
        // D s_59_107: read-var descriptor:u32
        let s_59_107: u32 = fn_state.descriptor;
        // D s_59_108: cast zx s_59_107 -> bv
        let s_59_108: Bits = Bits::new(s_59_107 as u128, 32u16);
        // C s_59_109: const #1u : u64
        let s_59_109: u64 = 1;
        // D s_59_110: bit-extract s_59_108 s_59_106 s_59_109
        let s_59_110: Bits = (Bits::new(
            ((s_59_108) >> (s_59_106)).value(),
            u16::try_from(s_59_109).unwrap(),
        ));
        // D s_59_111: cast reint s_59_110 -> u8
        let s_59_111: bool = ((s_59_110.value()) != 0);
        // C s_59_112: const #0s : i
        let s_59_112: i128 = 0;
        // C s_59_113: const #0u : u64
        let s_59_113: u64 = 0;
        // D s_59_114: cast zx s_59_111 -> u64
        let s_59_114: u64 = (s_59_111 as u64);
        // C s_59_115: const #1u : u64
        let s_59_115: u64 = 1;
        // D s_59_116: and s_59_114 s_59_115
        let s_59_116: u64 = ((s_59_114) & (s_59_115));
        // D s_59_117: cmp-eq s_59_116 s_59_115
        let s_59_117: bool = ((s_59_116) == (s_59_115));
        // D s_59_118: lsl s_59_114 s_59_112
        let s_59_118: u64 = s_59_114 << s_59_112;
        // D s_59_119: or s_59_113 s_59_118
        let s_59_119: u64 = ((s_59_113) | (s_59_118));
        // D s_59_120: cmpl s_59_118
        let s_59_120: u64 = !s_59_118;
        // D s_59_121: and s_59_113 s_59_120
        let s_59_121: u64 = ((s_59_113) & (s_59_120));
        // D s_59_122: select s_59_117 s_59_119 s_59_121
        let s_59_122: u64 = if s_59_117 { s_59_119 } else { s_59_121 };
        // D s_59_123: cast trunc s_59_122 -> u8
        let s_59_123: bool = ((s_59_122) != 0);
        // D s_59_124: write-var c <= s_59_123
        fn_state.c = s_59_123;
        // C s_59_125: const #2s : i
        let s_59_125: i128 = 2;
        // D s_59_126: read-var descriptor:u32
        let s_59_126: u32 = fn_state.descriptor;
        // D s_59_127: cast zx s_59_126 -> bv
        let s_59_127: Bits = Bits::new(s_59_126 as u128, 32u16);
        // C s_59_128: const #1u : u64
        let s_59_128: u64 = 1;
        // D s_59_129: bit-extract s_59_127 s_59_125 s_59_128
        let s_59_129: Bits = (Bits::new(
            ((s_59_127) >> (s_59_125)).value(),
            u16::try_from(s_59_128).unwrap(),
        ));
        // D s_59_130: cast reint s_59_129 -> u8
        let s_59_130: bool = ((s_59_129.value()) != 0);
        // C s_59_131: const #0s : i
        let s_59_131: i128 = 0;
        // C s_59_132: const #0u : u64
        let s_59_132: u64 = 0;
        // D s_59_133: cast zx s_59_130 -> u64
        let s_59_133: u64 = (s_59_130 as u64);
        // C s_59_134: const #1u : u64
        let s_59_134: u64 = 1;
        // D s_59_135: and s_59_133 s_59_134
        let s_59_135: u64 = ((s_59_133) & (s_59_134));
        // D s_59_136: cmp-eq s_59_135 s_59_134
        let s_59_136: bool = ((s_59_135) == (s_59_134));
        // D s_59_137: lsl s_59_133 s_59_131
        let s_59_137: u64 = s_59_133 << s_59_131;
        // D s_59_138: or s_59_132 s_59_137
        let s_59_138: u64 = ((s_59_132) | (s_59_137));
        // D s_59_139: cmpl s_59_137
        let s_59_139: u64 = !s_59_137;
        // D s_59_140: and s_59_132 s_59_139
        let s_59_140: u64 = ((s_59_132) & (s_59_139));
        // D s_59_141: select s_59_136 s_59_138 s_59_140
        let s_59_141: u64 = if s_59_136 { s_59_138 } else { s_59_140 };
        // D s_59_142: cast trunc s_59_141 -> u8
        let s_59_142: bool = ((s_59_141) != 0);
        // D s_59_143: write-var b <= s_59_142
        fn_state.b = s_59_142;
        // C s_59_144: const #56s : i
        let s_59_144: i128 = 56;
        // C s_59_145: const #31s : i
        let s_59_145: i128 = 31;
        // C s_59_146: const #16s : i
        let s_59_146: i128 = 16;
        // C s_59_147: const #16s : i
        let s_59_147: i128 = 16;
        // D s_59_148: read-var descriptor:u32
        let s_59_148: u32 = fn_state.descriptor;
        // D s_59_149: cast zx s_59_148 -> bv
        let s_59_149: Bits = Bits::new(s_59_148 as u128, 32u16);
        // D s_59_150: call place_subrange(s_59_144, s_59_149, s_59_145, s_59_146, s_59_147)
        let s_59_150: Bits = place_subrange(
            state,
            tracer,
            s_59_144,
            s_59_149,
            s_59_145,
            s_59_146,
            s_59_147,
        );
        // D s_59_151: cast reint s_59_150 -> u56
        let s_59_151: u64 = (s_59_150.value() as u64);
        // D s_59_152: write-var walkstate.0.0 <= s_59_151
        fn_state.walkstate._0._0 = s_59_151;
        // C s_59_153: const #0u : u8
        let s_59_153: bool = false;
        // D s_59_154: write-var walkstate.5 <= s_59_153
        fn_state.walkstate._5 = s_59_153;
        // N s_59_155: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_60_0: const #3u : u32
        let s_60_0: u32 = 3;
        // D s_60_1: read-var ga#22286:u32
        let s_60_1: u32 = fn_state.ga_22286;
        // D s_60_2: cmp-eq s_60_0 s_60_1
        let s_60_2: bool = ((s_60_0) == (s_60_1));
        // D s_60_3: not s_60_2
        let s_60_3: bool = !s_60_2;
        // N s_60_4: branch s_60_3 b62 b61
        if s_60_3 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_61_0: const #19s : i
        let s_61_0: i128 = 19;
        // D s_61_1: read-var descriptor:u32
        let s_61_1: u32 = fn_state.descriptor;
        // D s_61_2: cast zx s_61_1 -> bv
        let s_61_2: Bits = Bits::new(s_61_1 as u128, 32u16);
        // C s_61_3: const #1u : u64
        let s_61_3: u64 = 1;
        // D s_61_4: bit-extract s_61_2 s_61_0 s_61_3
        let s_61_4: Bits = (Bits::new(
            ((s_61_2) >> (s_61_0)).value(),
            u16::try_from(s_61_3).unwrap(),
        ));
        // D s_61_5: cast reint s_61_4 -> u8
        let s_61_5: bool = ((s_61_4.value()) != 0);
        // C s_61_6: const #0s : i
        let s_61_6: i128 = 0;
        // C s_61_7: const #0u : u64
        let s_61_7: u64 = 0;
        // D s_61_8: cast zx s_61_5 -> u64
        let s_61_8: u64 = (s_61_5 as u64);
        // C s_61_9: const #1u : u64
        let s_61_9: u64 = 1;
        // D s_61_10: and s_61_8 s_61_9
        let s_61_10: u64 = ((s_61_8) & (s_61_9));
        // D s_61_11: cmp-eq s_61_10 s_61_9
        let s_61_11: bool = ((s_61_10) == (s_61_9));
        // D s_61_12: lsl s_61_8 s_61_6
        let s_61_12: u64 = s_61_8 << s_61_6;
        // D s_61_13: or s_61_7 s_61_12
        let s_61_13: u64 = ((s_61_7) | (s_61_12));
        // D s_61_14: cmpl s_61_12
        let s_61_14: u64 = !s_61_12;
        // D s_61_15: and s_61_7 s_61_14
        let s_61_15: u64 = ((s_61_7) & (s_61_14));
        // D s_61_16: select s_61_11 s_61_13 s_61_15
        let s_61_16: u64 = if s_61_11 { s_61_13 } else { s_61_15 };
        // D s_61_17: cast trunc s_61_16 -> u8
        let s_61_17: bool = ((s_61_16) != 0);
        // D s_61_18: write-var ns <= s_61_17
        fn_state.ns = s_61_17;
        // C s_61_19: const #17s : i
        let s_61_19: i128 = 17;
        // D s_61_20: read-var descriptor:u32
        let s_61_20: u32 = fn_state.descriptor;
        // D s_61_21: cast zx s_61_20 -> bv
        let s_61_21: Bits = Bits::new(s_61_20 as u128, 32u16);
        // C s_61_22: const #1u : u64
        let s_61_22: u64 = 1;
        // D s_61_23: bit-extract s_61_21 s_61_19 s_61_22
        let s_61_23: Bits = (Bits::new(
            ((s_61_21) >> (s_61_19)).value(),
            u16::try_from(s_61_22).unwrap(),
        ));
        // D s_61_24: cast reint s_61_23 -> u8
        let s_61_24: bool = ((s_61_23.value()) != 0);
        // C s_61_25: const #0s : i
        let s_61_25: i128 = 0;
        // C s_61_26: const #0u : u64
        let s_61_26: u64 = 0;
        // D s_61_27: cast zx s_61_24 -> u64
        let s_61_27: u64 = (s_61_24 as u64);
        // C s_61_28: const #1u : u64
        let s_61_28: u64 = 1;
        // D s_61_29: and s_61_27 s_61_28
        let s_61_29: u64 = ((s_61_27) & (s_61_28));
        // D s_61_30: cmp-eq s_61_29 s_61_28
        let s_61_30: bool = ((s_61_29) == (s_61_28));
        // D s_61_31: lsl s_61_27 s_61_25
        let s_61_31: u64 = s_61_27 << s_61_25;
        // D s_61_32: or s_61_26 s_61_31
        let s_61_32: u64 = ((s_61_26) | (s_61_31));
        // D s_61_33: cmpl s_61_31
        let s_61_33: u64 = !s_61_31;
        // D s_61_34: and s_61_26 s_61_33
        let s_61_34: u64 = ((s_61_26) & (s_61_33));
        // D s_61_35: select s_61_30 s_61_32 s_61_34
        let s_61_35: u64 = if s_61_30 { s_61_32 } else { s_61_34 };
        // D s_61_36: cast trunc s_61_35 -> u8
        let s_61_36: bool = ((s_61_35) != 0);
        // D s_61_37: write-var nG <= s_61_36
        fn_state.nG = s_61_36;
        // C s_61_38: const #16s : i
        let s_61_38: i128 = 16;
        // D s_61_39: read-var descriptor:u32
        let s_61_39: u32 = fn_state.descriptor;
        // D s_61_40: cast zx s_61_39 -> bv
        let s_61_40: Bits = Bits::new(s_61_39 as u128, 32u16);
        // C s_61_41: const #1u : u64
        let s_61_41: u64 = 1;
        // D s_61_42: bit-extract s_61_40 s_61_38 s_61_41
        let s_61_42: Bits = (Bits::new(
            ((s_61_40) >> (s_61_38)).value(),
            u16::try_from(s_61_41).unwrap(),
        ));
        // D s_61_43: cast reint s_61_42 -> u8
        let s_61_43: bool = ((s_61_42.value()) != 0);
        // C s_61_44: const #0s : i
        let s_61_44: i128 = 0;
        // C s_61_45: const #0u : u64
        let s_61_45: u64 = 0;
        // D s_61_46: cast zx s_61_43 -> u64
        let s_61_46: u64 = (s_61_43 as u64);
        // C s_61_47: const #1u : u64
        let s_61_47: u64 = 1;
        // D s_61_48: and s_61_46 s_61_47
        let s_61_48: u64 = ((s_61_46) & (s_61_47));
        // D s_61_49: cmp-eq s_61_48 s_61_47
        let s_61_49: bool = ((s_61_48) == (s_61_47));
        // D s_61_50: lsl s_61_46 s_61_44
        let s_61_50: u64 = s_61_46 << s_61_44;
        // D s_61_51: or s_61_45 s_61_50
        let s_61_51: u64 = ((s_61_45) | (s_61_50));
        // D s_61_52: cmpl s_61_50
        let s_61_52: u64 = !s_61_50;
        // D s_61_53: and s_61_45 s_61_52
        let s_61_53: u64 = ((s_61_45) & (s_61_52));
        // D s_61_54: select s_61_49 s_61_51 s_61_53
        let s_61_54: u64 = if s_61_49 { s_61_51 } else { s_61_53 };
        // D s_61_55: cast trunc s_61_54 -> u8
        let s_61_55: bool = ((s_61_54) != 0);
        // D s_61_56: write-var s <= s_61_55
        fn_state.s = s_61_55;
        // C s_61_57: const #15s : i
        let s_61_57: i128 = 15;
        // D s_61_58: read-var descriptor:u32
        let s_61_58: u32 = fn_state.descriptor;
        // D s_61_59: cast zx s_61_58 -> bv
        let s_61_59: Bits = Bits::new(s_61_58 as u128, 32u16);
        // C s_61_60: const #1u : u64
        let s_61_60: u64 = 1;
        // D s_61_61: bit-extract s_61_59 s_61_57 s_61_60
        let s_61_61: Bits = (Bits::new(
            ((s_61_59) >> (s_61_57)).value(),
            u16::try_from(s_61_60).unwrap(),
        ));
        // D s_61_62: cast reint s_61_61 -> u8
        let s_61_62: bool = ((s_61_61.value()) != 0);
        // C s_61_63: const #0s : i
        let s_61_63: i128 = 0;
        // C s_61_64: const #0u : u64
        let s_61_64: u64 = 0;
        // D s_61_65: cast zx s_61_62 -> u64
        let s_61_65: u64 = (s_61_62 as u64);
        // C s_61_66: const #1u : u64
        let s_61_66: u64 = 1;
        // D s_61_67: and s_61_65 s_61_66
        let s_61_67: u64 = ((s_61_65) & (s_61_66));
        // D s_61_68: cmp-eq s_61_67 s_61_66
        let s_61_68: bool = ((s_61_67) == (s_61_66));
        // D s_61_69: lsl s_61_65 s_61_63
        let s_61_69: u64 = s_61_65 << s_61_63;
        // D s_61_70: or s_61_64 s_61_69
        let s_61_70: u64 = ((s_61_64) | (s_61_69));
        // D s_61_71: cmpl s_61_69
        let s_61_71: u64 = !s_61_69;
        // D s_61_72: and s_61_64 s_61_71
        let s_61_72: u64 = ((s_61_64) & (s_61_71));
        // D s_61_73: select s_61_68 s_61_70 s_61_72
        let s_61_73: u64 = if s_61_68 { s_61_70 } else { s_61_72 };
        // D s_61_74: cast trunc s_61_73 -> u8
        let s_61_74: bool = ((s_61_73) != 0);
        // C s_61_75: const #10s : i
        let s_61_75: i128 = 10;
        // D s_61_76: read-var descriptor:u32
        let s_61_76: u32 = fn_state.descriptor;
        // D s_61_77: cast zx s_61_76 -> bv
        let s_61_77: Bits = Bits::new(s_61_76 as u128, 32u16);
        // C s_61_78: const #1s : i64
        let s_61_78: i64 = 1;
        // C s_61_79: cast zx s_61_78 -> i
        let s_61_79: i128 = (i128::try_from(s_61_78).unwrap());
        // C s_61_80: const #1s : i
        let s_61_80: i128 = 1;
        // C s_61_81: add s_61_80 s_61_79
        let s_61_81: i128 = (s_61_80 + s_61_79);
        // D s_61_82: bit-extract s_61_77 s_61_75 s_61_81
        let s_61_82: Bits = (Bits::new(
            ((s_61_77) >> (s_61_75)).value(),
            u16::try_from(s_61_81).unwrap(),
        ));
        // D s_61_83: cast reint s_61_82 -> u8
        let s_61_83: u8 = (s_61_82.value() as u8);
        // D s_61_84: cast zx s_61_74 -> bv
        let s_61_84: Bits = Bits::new(s_61_74 as u128, 1u16);
        // D s_61_85: cast zx s_61_83 -> bv
        let s_61_85: Bits = Bits::new(s_61_83 as u128, 2u16);
        // D s_61_86: cast reint s_61_84 -> u128
        let s_61_86: u128 = (s_61_84.value() as u128);
        // D s_61_87: size-of s_61_84
        let s_61_87: u16 = s_61_84.length();
        // D s_61_88: cast reint s_61_85 -> u128
        let s_61_88: u128 = (s_61_85.value() as u128);
        // D s_61_89: size-of s_61_85
        let s_61_89: u16 = s_61_85.length();
        // D s_61_90: lsl s_61_86 s_61_89
        let s_61_90: u128 = s_61_86 << s_61_89;
        // D s_61_91: or s_61_90 s_61_88
        let s_61_91: u128 = ((s_61_90) | (s_61_88));
        // D s_61_92: add s_61_87 s_61_89
        let s_61_92: u16 = (s_61_87 + s_61_89);
        // D s_61_93: create-bits s_61_91 s_61_92
        let s_61_93: Bits = Bits::new(s_61_91, s_61_92);
        // D s_61_94: cast reint s_61_93 -> u8
        let s_61_94: u8 = (s_61_93.value() as u8);
        // D s_61_95: write-var ap <= s_61_94
        fn_state.ap = s_61_94;
        // C s_61_96: const #12s : i
        let s_61_96: i128 = 12;
        // D s_61_97: read-var descriptor:u32
        let s_61_97: u32 = fn_state.descriptor;
        // D s_61_98: cast zx s_61_97 -> bv
        let s_61_98: Bits = Bits::new(s_61_97 as u128, 32u16);
        // C s_61_99: const #1s : i64
        let s_61_99: i64 = 1;
        // C s_61_100: cast zx s_61_99 -> i
        let s_61_100: i128 = (i128::try_from(s_61_99).unwrap());
        // C s_61_101: const #2s : i
        let s_61_101: i128 = 2;
        // C s_61_102: add s_61_101 s_61_100
        let s_61_102: i128 = (s_61_101 + s_61_100);
        // D s_61_103: bit-extract s_61_98 s_61_96 s_61_102
        let s_61_103: Bits = (Bits::new(
            ((s_61_98) >> (s_61_96)).value(),
            u16::try_from(s_61_102).unwrap(),
        ));
        // D s_61_104: cast reint s_61_103 -> u8
        let s_61_104: u8 = (s_61_103.value() as u8);
        // D s_61_105: write-var tex <= s_61_104
        fn_state.tex = s_61_104;
        // C s_61_106: const #5s : i
        let s_61_106: i128 = 5;
        // D s_61_107: read-var descriptor:u32
        let s_61_107: u32 = fn_state.descriptor;
        // D s_61_108: cast zx s_61_107 -> bv
        let s_61_108: Bits = Bits::new(s_61_107 as u128, 32u16);
        // C s_61_109: const #1s : i64
        let s_61_109: i64 = 1;
        // C s_61_110: cast zx s_61_109 -> i
        let s_61_110: i128 = (i128::try_from(s_61_109).unwrap());
        // C s_61_111: const #3s : i
        let s_61_111: i128 = 3;
        // C s_61_112: add s_61_111 s_61_110
        let s_61_112: i128 = (s_61_111 + s_61_110);
        // D s_61_113: bit-extract s_61_108 s_61_106 s_61_112
        let s_61_113: Bits = (Bits::new(
            ((s_61_108) >> (s_61_106)).value(),
            u16::try_from(s_61_112).unwrap(),
        ));
        // D s_61_114: cast reint s_61_113 -> u8
        let s_61_114: u8 = (s_61_113.value() as u8);
        // D s_61_115: write-var domain <= s_61_114
        fn_state.domain = s_61_114;
        // C s_61_116: const #4s : i
        let s_61_116: i128 = 4;
        // D s_61_117: read-var descriptor:u32
        let s_61_117: u32 = fn_state.descriptor;
        // D s_61_118: cast zx s_61_117 -> bv
        let s_61_118: Bits = Bits::new(s_61_117 as u128, 32u16);
        // C s_61_119: const #1u : u64
        let s_61_119: u64 = 1;
        // D s_61_120: bit-extract s_61_118 s_61_116 s_61_119
        let s_61_120: Bits = (Bits::new(
            ((s_61_118) >> (s_61_116)).value(),
            u16::try_from(s_61_119).unwrap(),
        ));
        // D s_61_121: cast reint s_61_120 -> u8
        let s_61_121: bool = ((s_61_120.value()) != 0);
        // C s_61_122: const #0s : i
        let s_61_122: i128 = 0;
        // C s_61_123: const #0u : u64
        let s_61_123: u64 = 0;
        // D s_61_124: cast zx s_61_121 -> u64
        let s_61_124: u64 = (s_61_121 as u64);
        // C s_61_125: const #1u : u64
        let s_61_125: u64 = 1;
        // D s_61_126: and s_61_124 s_61_125
        let s_61_126: u64 = ((s_61_124) & (s_61_125));
        // D s_61_127: cmp-eq s_61_126 s_61_125
        let s_61_127: bool = ((s_61_126) == (s_61_125));
        // D s_61_128: lsl s_61_124 s_61_122
        let s_61_128: u64 = s_61_124 << s_61_122;
        // D s_61_129: or s_61_123 s_61_128
        let s_61_129: u64 = ((s_61_123) | (s_61_128));
        // D s_61_130: cmpl s_61_128
        let s_61_130: u64 = !s_61_128;
        // D s_61_131: and s_61_123 s_61_130
        let s_61_131: u64 = ((s_61_123) & (s_61_130));
        // D s_61_132: select s_61_127 s_61_129 s_61_131
        let s_61_132: u64 = if s_61_127 { s_61_129 } else { s_61_131 };
        // D s_61_133: cast trunc s_61_132 -> u8
        let s_61_133: bool = ((s_61_132) != 0);
        // D s_61_134: write-var xn <= s_61_133
        fn_state.xn = s_61_133;
        // C s_61_135: const #3s : i
        let s_61_135: i128 = 3;
        // D s_61_136: read-var descriptor:u32
        let s_61_136: u32 = fn_state.descriptor;
        // D s_61_137: cast zx s_61_136 -> bv
        let s_61_137: Bits = Bits::new(s_61_136 as u128, 32u16);
        // C s_61_138: const #1u : u64
        let s_61_138: u64 = 1;
        // D s_61_139: bit-extract s_61_137 s_61_135 s_61_138
        let s_61_139: Bits = (Bits::new(
            ((s_61_137) >> (s_61_135)).value(),
            u16::try_from(s_61_138).unwrap(),
        ));
        // D s_61_140: cast reint s_61_139 -> u8
        let s_61_140: bool = ((s_61_139.value()) != 0);
        // C s_61_141: const #0s : i
        let s_61_141: i128 = 0;
        // C s_61_142: const #0u : u64
        let s_61_142: u64 = 0;
        // D s_61_143: cast zx s_61_140 -> u64
        let s_61_143: u64 = (s_61_140 as u64);
        // C s_61_144: const #1u : u64
        let s_61_144: u64 = 1;
        // D s_61_145: and s_61_143 s_61_144
        let s_61_145: u64 = ((s_61_143) & (s_61_144));
        // D s_61_146: cmp-eq s_61_145 s_61_144
        let s_61_146: bool = ((s_61_145) == (s_61_144));
        // D s_61_147: lsl s_61_143 s_61_141
        let s_61_147: u64 = s_61_143 << s_61_141;
        // D s_61_148: or s_61_142 s_61_147
        let s_61_148: u64 = ((s_61_142) | (s_61_147));
        // D s_61_149: cmpl s_61_147
        let s_61_149: u64 = !s_61_147;
        // D s_61_150: and s_61_142 s_61_149
        let s_61_150: u64 = ((s_61_142) & (s_61_149));
        // D s_61_151: select s_61_146 s_61_148 s_61_150
        let s_61_151: u64 = if s_61_146 { s_61_148 } else { s_61_150 };
        // D s_61_152: cast trunc s_61_151 -> u8
        let s_61_152: bool = ((s_61_151) != 0);
        // D s_61_153: write-var c <= s_61_152
        fn_state.c = s_61_152;
        // C s_61_154: const #2s : i
        let s_61_154: i128 = 2;
        // D s_61_155: read-var descriptor:u32
        let s_61_155: u32 = fn_state.descriptor;
        // D s_61_156: cast zx s_61_155 -> bv
        let s_61_156: Bits = Bits::new(s_61_155 as u128, 32u16);
        // C s_61_157: const #1u : u64
        let s_61_157: u64 = 1;
        // D s_61_158: bit-extract s_61_156 s_61_154 s_61_157
        let s_61_158: Bits = (Bits::new(
            ((s_61_156) >> (s_61_154)).value(),
            u16::try_from(s_61_157).unwrap(),
        ));
        // D s_61_159: cast reint s_61_158 -> u8
        let s_61_159: bool = ((s_61_158.value()) != 0);
        // C s_61_160: const #0s : i
        let s_61_160: i128 = 0;
        // C s_61_161: const #0u : u64
        let s_61_161: u64 = 0;
        // D s_61_162: cast zx s_61_159 -> u64
        let s_61_162: u64 = (s_61_159 as u64);
        // C s_61_163: const #1u : u64
        let s_61_163: u64 = 1;
        // D s_61_164: and s_61_162 s_61_163
        let s_61_164: u64 = ((s_61_162) & (s_61_163));
        // D s_61_165: cmp-eq s_61_164 s_61_163
        let s_61_165: bool = ((s_61_164) == (s_61_163));
        // D s_61_166: lsl s_61_162 s_61_160
        let s_61_166: u64 = s_61_162 << s_61_160;
        // D s_61_167: or s_61_161 s_61_166
        let s_61_167: u64 = ((s_61_161) | (s_61_166));
        // D s_61_168: cmpl s_61_166
        let s_61_168: u64 = !s_61_166;
        // D s_61_169: and s_61_161 s_61_168
        let s_61_169: u64 = ((s_61_161) & (s_61_168));
        // D s_61_170: select s_61_165 s_61_167 s_61_169
        let s_61_170: u64 = if s_61_165 { s_61_167 } else { s_61_169 };
        // D s_61_171: cast trunc s_61_170 -> u8
        let s_61_171: bool = ((s_61_170) != 0);
        // D s_61_172: write-var b <= s_61_171
        fn_state.b = s_61_171;
        // C s_61_173: const #0s : i
        let s_61_173: i128 = 0;
        // D s_61_174: read-var descriptor:u32
        let s_61_174: u32 = fn_state.descriptor;
        // D s_61_175: cast zx s_61_174 -> bv
        let s_61_175: Bits = Bits::new(s_61_174 as u128, 32u16);
        // C s_61_176: const #1u : u64
        let s_61_176: u64 = 1;
        // D s_61_177: bit-extract s_61_175 s_61_173 s_61_176
        let s_61_177: Bits = (Bits::new(
            ((s_61_175) >> (s_61_173)).value(),
            u16::try_from(s_61_176).unwrap(),
        ));
        // D s_61_178: cast reint s_61_177 -> u8
        let s_61_178: bool = ((s_61_177.value()) != 0);
        // C s_61_179: const #0s : i
        let s_61_179: i128 = 0;
        // C s_61_180: const #0u : u64
        let s_61_180: u64 = 0;
        // D s_61_181: cast zx s_61_178 -> u64
        let s_61_181: u64 = (s_61_178 as u64);
        // C s_61_182: const #1u : u64
        let s_61_182: u64 = 1;
        // D s_61_183: and s_61_181 s_61_182
        let s_61_183: u64 = ((s_61_181) & (s_61_182));
        // D s_61_184: cmp-eq s_61_183 s_61_182
        let s_61_184: bool = ((s_61_183) == (s_61_182));
        // D s_61_185: lsl s_61_181 s_61_179
        let s_61_185: u64 = s_61_181 << s_61_179;
        // D s_61_186: or s_61_180 s_61_185
        let s_61_186: u64 = ((s_61_180) | (s_61_185));
        // D s_61_187: cmpl s_61_185
        let s_61_187: u64 = !s_61_185;
        // D s_61_188: and s_61_180 s_61_187
        let s_61_188: u64 = ((s_61_180) & (s_61_187));
        // D s_61_189: select s_61_184 s_61_186 s_61_188
        let s_61_189: u64 = if s_61_184 { s_61_186 } else { s_61_188 };
        // D s_61_190: cast trunc s_61_189 -> u8
        let s_61_190: bool = ((s_61_189) != 0);
        // D s_61_191: write-var pxn <= s_61_190
        fn_state.pxn = s_61_190;
        // C s_61_192: const #56s : i
        let s_61_192: i128 = 56;
        // C s_61_193: const #31s : i
        let s_61_193: i128 = 31;
        // C s_61_194: const #20s : i
        let s_61_194: i128 = 20;
        // C s_61_195: const #20s : i
        let s_61_195: i128 = 20;
        // D s_61_196: read-var descriptor:u32
        let s_61_196: u32 = fn_state.descriptor;
        // D s_61_197: cast zx s_61_196 -> bv
        let s_61_197: Bits = Bits::new(s_61_196 as u128, 32u16);
        // D s_61_198: call place_subrange(s_61_192, s_61_197, s_61_193, s_61_194, s_61_195)
        let s_61_198: Bits = place_subrange(
            state,
            tracer,
            s_61_192,
            s_61_197,
            s_61_193,
            s_61_194,
            s_61_195,
        );
        // D s_61_199: cast reint s_61_198 -> u56
        let s_61_199: u64 = (s_61_198.value() as u64);
        // D s_61_200: write-var walkstate.0.0 <= s_61_199
        fn_state.walkstate._0._0 = s_61_199;
        // C s_61_201: const #0u : u8
        let s_61_201: bool = false;
        // D s_61_202: write-var walkstate.5 <= s_61_201
        fn_state.walkstate._5 = s_61_201;
        // N s_61_203: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_62_0: const #2u : u32
        let s_62_0: u32 = 2;
        // D s_62_1: read-var ga#22286:u32
        let s_62_1: u32 = fn_state.ga_22286;
        // D s_62_2: cmp-eq s_62_0 s_62_1
        let s_62_2: bool = ((s_62_0) == (s_62_1));
        // D s_62_3: not s_62_2
        let s_62_3: bool = !s_62_2;
        // N s_62_4: branch s_62_3 b64 b63
        if s_62_3 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_63_0: const #19s : i
        let s_63_0: i128 = 19;
        // D s_63_1: read-var descriptor:u32
        let s_63_1: u32 = fn_state.descriptor;
        // D s_63_2: cast zx s_63_1 -> bv
        let s_63_2: Bits = Bits::new(s_63_1 as u128, 32u16);
        // C s_63_3: const #1u : u64
        let s_63_3: u64 = 1;
        // D s_63_4: bit-extract s_63_2 s_63_0 s_63_3
        let s_63_4: Bits = (Bits::new(
            ((s_63_2) >> (s_63_0)).value(),
            u16::try_from(s_63_3).unwrap(),
        ));
        // D s_63_5: cast reint s_63_4 -> u8
        let s_63_5: bool = ((s_63_4.value()) != 0);
        // C s_63_6: const #0s : i
        let s_63_6: i128 = 0;
        // C s_63_7: const #0u : u64
        let s_63_7: u64 = 0;
        // D s_63_8: cast zx s_63_5 -> u64
        let s_63_8: u64 = (s_63_5 as u64);
        // C s_63_9: const #1u : u64
        let s_63_9: u64 = 1;
        // D s_63_10: and s_63_8 s_63_9
        let s_63_10: u64 = ((s_63_8) & (s_63_9));
        // D s_63_11: cmp-eq s_63_10 s_63_9
        let s_63_11: bool = ((s_63_10) == (s_63_9));
        // D s_63_12: lsl s_63_8 s_63_6
        let s_63_12: u64 = s_63_8 << s_63_6;
        // D s_63_13: or s_63_7 s_63_12
        let s_63_13: u64 = ((s_63_7) | (s_63_12));
        // D s_63_14: cmpl s_63_12
        let s_63_14: u64 = !s_63_12;
        // D s_63_15: and s_63_7 s_63_14
        let s_63_15: u64 = ((s_63_7) & (s_63_14));
        // D s_63_16: select s_63_11 s_63_13 s_63_15
        let s_63_16: u64 = if s_63_11 { s_63_13 } else { s_63_15 };
        // D s_63_17: cast trunc s_63_16 -> u8
        let s_63_17: bool = ((s_63_16) != 0);
        // D s_63_18: write-var ns <= s_63_17
        fn_state.ns = s_63_17;
        // C s_63_19: const #17s : i
        let s_63_19: i128 = 17;
        // D s_63_20: read-var descriptor:u32
        let s_63_20: u32 = fn_state.descriptor;
        // D s_63_21: cast zx s_63_20 -> bv
        let s_63_21: Bits = Bits::new(s_63_20 as u128, 32u16);
        // C s_63_22: const #1u : u64
        let s_63_22: u64 = 1;
        // D s_63_23: bit-extract s_63_21 s_63_19 s_63_22
        let s_63_23: Bits = (Bits::new(
            ((s_63_21) >> (s_63_19)).value(),
            u16::try_from(s_63_22).unwrap(),
        ));
        // D s_63_24: cast reint s_63_23 -> u8
        let s_63_24: bool = ((s_63_23.value()) != 0);
        // C s_63_25: const #0s : i
        let s_63_25: i128 = 0;
        // C s_63_26: const #0u : u64
        let s_63_26: u64 = 0;
        // D s_63_27: cast zx s_63_24 -> u64
        let s_63_27: u64 = (s_63_24 as u64);
        // C s_63_28: const #1u : u64
        let s_63_28: u64 = 1;
        // D s_63_29: and s_63_27 s_63_28
        let s_63_29: u64 = ((s_63_27) & (s_63_28));
        // D s_63_30: cmp-eq s_63_29 s_63_28
        let s_63_30: bool = ((s_63_29) == (s_63_28));
        // D s_63_31: lsl s_63_27 s_63_25
        let s_63_31: u64 = s_63_27 << s_63_25;
        // D s_63_32: or s_63_26 s_63_31
        let s_63_32: u64 = ((s_63_26) | (s_63_31));
        // D s_63_33: cmpl s_63_31
        let s_63_33: u64 = !s_63_31;
        // D s_63_34: and s_63_26 s_63_33
        let s_63_34: u64 = ((s_63_26) & (s_63_33));
        // D s_63_35: select s_63_30 s_63_32 s_63_34
        let s_63_35: u64 = if s_63_30 { s_63_32 } else { s_63_34 };
        // D s_63_36: cast trunc s_63_35 -> u8
        let s_63_36: bool = ((s_63_35) != 0);
        // D s_63_37: write-var nG <= s_63_36
        fn_state.nG = s_63_36;
        // C s_63_38: const #16s : i
        let s_63_38: i128 = 16;
        // D s_63_39: read-var descriptor:u32
        let s_63_39: u32 = fn_state.descriptor;
        // D s_63_40: cast zx s_63_39 -> bv
        let s_63_40: Bits = Bits::new(s_63_39 as u128, 32u16);
        // C s_63_41: const #1u : u64
        let s_63_41: u64 = 1;
        // D s_63_42: bit-extract s_63_40 s_63_38 s_63_41
        let s_63_42: Bits = (Bits::new(
            ((s_63_40) >> (s_63_38)).value(),
            u16::try_from(s_63_41).unwrap(),
        ));
        // D s_63_43: cast reint s_63_42 -> u8
        let s_63_43: bool = ((s_63_42.value()) != 0);
        // C s_63_44: const #0s : i
        let s_63_44: i128 = 0;
        // C s_63_45: const #0u : u64
        let s_63_45: u64 = 0;
        // D s_63_46: cast zx s_63_43 -> u64
        let s_63_46: u64 = (s_63_43 as u64);
        // C s_63_47: const #1u : u64
        let s_63_47: u64 = 1;
        // D s_63_48: and s_63_46 s_63_47
        let s_63_48: u64 = ((s_63_46) & (s_63_47));
        // D s_63_49: cmp-eq s_63_48 s_63_47
        let s_63_49: bool = ((s_63_48) == (s_63_47));
        // D s_63_50: lsl s_63_46 s_63_44
        let s_63_50: u64 = s_63_46 << s_63_44;
        // D s_63_51: or s_63_45 s_63_50
        let s_63_51: u64 = ((s_63_45) | (s_63_50));
        // D s_63_52: cmpl s_63_50
        let s_63_52: u64 = !s_63_50;
        // D s_63_53: and s_63_45 s_63_52
        let s_63_53: u64 = ((s_63_45) & (s_63_52));
        // D s_63_54: select s_63_49 s_63_51 s_63_53
        let s_63_54: u64 = if s_63_49 { s_63_51 } else { s_63_53 };
        // D s_63_55: cast trunc s_63_54 -> u8
        let s_63_55: bool = ((s_63_54) != 0);
        // D s_63_56: write-var s <= s_63_55
        fn_state.s = s_63_55;
        // C s_63_57: const #15s : i
        let s_63_57: i128 = 15;
        // D s_63_58: read-var descriptor:u32
        let s_63_58: u32 = fn_state.descriptor;
        // D s_63_59: cast zx s_63_58 -> bv
        let s_63_59: Bits = Bits::new(s_63_58 as u128, 32u16);
        // C s_63_60: const #1u : u64
        let s_63_60: u64 = 1;
        // D s_63_61: bit-extract s_63_59 s_63_57 s_63_60
        let s_63_61: Bits = (Bits::new(
            ((s_63_59) >> (s_63_57)).value(),
            u16::try_from(s_63_60).unwrap(),
        ));
        // D s_63_62: cast reint s_63_61 -> u8
        let s_63_62: bool = ((s_63_61.value()) != 0);
        // C s_63_63: const #0s : i
        let s_63_63: i128 = 0;
        // C s_63_64: const #0u : u64
        let s_63_64: u64 = 0;
        // D s_63_65: cast zx s_63_62 -> u64
        let s_63_65: u64 = (s_63_62 as u64);
        // C s_63_66: const #1u : u64
        let s_63_66: u64 = 1;
        // D s_63_67: and s_63_65 s_63_66
        let s_63_67: u64 = ((s_63_65) & (s_63_66));
        // D s_63_68: cmp-eq s_63_67 s_63_66
        let s_63_68: bool = ((s_63_67) == (s_63_66));
        // D s_63_69: lsl s_63_65 s_63_63
        let s_63_69: u64 = s_63_65 << s_63_63;
        // D s_63_70: or s_63_64 s_63_69
        let s_63_70: u64 = ((s_63_64) | (s_63_69));
        // D s_63_71: cmpl s_63_69
        let s_63_71: u64 = !s_63_69;
        // D s_63_72: and s_63_64 s_63_71
        let s_63_72: u64 = ((s_63_64) & (s_63_71));
        // D s_63_73: select s_63_68 s_63_70 s_63_72
        let s_63_73: u64 = if s_63_68 { s_63_70 } else { s_63_72 };
        // D s_63_74: cast trunc s_63_73 -> u8
        let s_63_74: bool = ((s_63_73) != 0);
        // C s_63_75: const #10s : i
        let s_63_75: i128 = 10;
        // D s_63_76: read-var descriptor:u32
        let s_63_76: u32 = fn_state.descriptor;
        // D s_63_77: cast zx s_63_76 -> bv
        let s_63_77: Bits = Bits::new(s_63_76 as u128, 32u16);
        // C s_63_78: const #1s : i64
        let s_63_78: i64 = 1;
        // C s_63_79: cast zx s_63_78 -> i
        let s_63_79: i128 = (i128::try_from(s_63_78).unwrap());
        // C s_63_80: const #1s : i
        let s_63_80: i128 = 1;
        // C s_63_81: add s_63_80 s_63_79
        let s_63_81: i128 = (s_63_80 + s_63_79);
        // D s_63_82: bit-extract s_63_77 s_63_75 s_63_81
        let s_63_82: Bits = (Bits::new(
            ((s_63_77) >> (s_63_75)).value(),
            u16::try_from(s_63_81).unwrap(),
        ));
        // D s_63_83: cast reint s_63_82 -> u8
        let s_63_83: u8 = (s_63_82.value() as u8);
        // D s_63_84: cast zx s_63_74 -> bv
        let s_63_84: Bits = Bits::new(s_63_74 as u128, 1u16);
        // D s_63_85: cast zx s_63_83 -> bv
        let s_63_85: Bits = Bits::new(s_63_83 as u128, 2u16);
        // D s_63_86: cast reint s_63_84 -> u128
        let s_63_86: u128 = (s_63_84.value() as u128);
        // D s_63_87: size-of s_63_84
        let s_63_87: u16 = s_63_84.length();
        // D s_63_88: cast reint s_63_85 -> u128
        let s_63_88: u128 = (s_63_85.value() as u128);
        // D s_63_89: size-of s_63_85
        let s_63_89: u16 = s_63_85.length();
        // D s_63_90: lsl s_63_86 s_63_89
        let s_63_90: u128 = s_63_86 << s_63_89;
        // D s_63_91: or s_63_90 s_63_88
        let s_63_91: u128 = ((s_63_90) | (s_63_88));
        // D s_63_92: add s_63_87 s_63_89
        let s_63_92: u16 = (s_63_87 + s_63_89);
        // D s_63_93: create-bits s_63_91 s_63_92
        let s_63_93: Bits = Bits::new(s_63_91, s_63_92);
        // D s_63_94: cast reint s_63_93 -> u8
        let s_63_94: u8 = (s_63_93.value() as u8);
        // D s_63_95: write-var ap <= s_63_94
        fn_state.ap = s_63_94;
        // C s_63_96: const #12s : i
        let s_63_96: i128 = 12;
        // D s_63_97: read-var descriptor:u32
        let s_63_97: u32 = fn_state.descriptor;
        // D s_63_98: cast zx s_63_97 -> bv
        let s_63_98: Bits = Bits::new(s_63_97 as u128, 32u16);
        // C s_63_99: const #1s : i64
        let s_63_99: i64 = 1;
        // C s_63_100: cast zx s_63_99 -> i
        let s_63_100: i128 = (i128::try_from(s_63_99).unwrap());
        // C s_63_101: const #2s : i
        let s_63_101: i128 = 2;
        // C s_63_102: add s_63_101 s_63_100
        let s_63_102: i128 = (s_63_101 + s_63_100);
        // D s_63_103: bit-extract s_63_98 s_63_96 s_63_102
        let s_63_103: Bits = (Bits::new(
            ((s_63_98) >> (s_63_96)).value(),
            u16::try_from(s_63_102).unwrap(),
        ));
        // D s_63_104: cast reint s_63_103 -> u8
        let s_63_104: u8 = (s_63_103.value() as u8);
        // D s_63_105: write-var tex <= s_63_104
        fn_state.tex = s_63_104;
        // C s_63_106: const #4s : i
        let s_63_106: i128 = 4;
        // D s_63_107: read-var descriptor:u32
        let s_63_107: u32 = fn_state.descriptor;
        // D s_63_108: cast zx s_63_107 -> bv
        let s_63_108: Bits = Bits::new(s_63_107 as u128, 32u16);
        // C s_63_109: const #1u : u64
        let s_63_109: u64 = 1;
        // D s_63_110: bit-extract s_63_108 s_63_106 s_63_109
        let s_63_110: Bits = (Bits::new(
            ((s_63_108) >> (s_63_106)).value(),
            u16::try_from(s_63_109).unwrap(),
        ));
        // D s_63_111: cast reint s_63_110 -> u8
        let s_63_111: bool = ((s_63_110.value()) != 0);
        // C s_63_112: const #0s : i
        let s_63_112: i128 = 0;
        // C s_63_113: const #0u : u64
        let s_63_113: u64 = 0;
        // D s_63_114: cast zx s_63_111 -> u64
        let s_63_114: u64 = (s_63_111 as u64);
        // C s_63_115: const #1u : u64
        let s_63_115: u64 = 1;
        // D s_63_116: and s_63_114 s_63_115
        let s_63_116: u64 = ((s_63_114) & (s_63_115));
        // D s_63_117: cmp-eq s_63_116 s_63_115
        let s_63_117: bool = ((s_63_116) == (s_63_115));
        // D s_63_118: lsl s_63_114 s_63_112
        let s_63_118: u64 = s_63_114 << s_63_112;
        // D s_63_119: or s_63_113 s_63_118
        let s_63_119: u64 = ((s_63_113) | (s_63_118));
        // D s_63_120: cmpl s_63_118
        let s_63_120: u64 = !s_63_118;
        // D s_63_121: and s_63_113 s_63_120
        let s_63_121: u64 = ((s_63_113) & (s_63_120));
        // D s_63_122: select s_63_117 s_63_119 s_63_121
        let s_63_122: u64 = if s_63_117 { s_63_119 } else { s_63_121 };
        // D s_63_123: cast trunc s_63_122 -> u8
        let s_63_123: bool = ((s_63_122) != 0);
        // D s_63_124: write-var xn <= s_63_123
        fn_state.xn = s_63_123;
        // C s_63_125: const #3s : i
        let s_63_125: i128 = 3;
        // D s_63_126: read-var descriptor:u32
        let s_63_126: u32 = fn_state.descriptor;
        // D s_63_127: cast zx s_63_126 -> bv
        let s_63_127: Bits = Bits::new(s_63_126 as u128, 32u16);
        // C s_63_128: const #1u : u64
        let s_63_128: u64 = 1;
        // D s_63_129: bit-extract s_63_127 s_63_125 s_63_128
        let s_63_129: Bits = (Bits::new(
            ((s_63_127) >> (s_63_125)).value(),
            u16::try_from(s_63_128).unwrap(),
        ));
        // D s_63_130: cast reint s_63_129 -> u8
        let s_63_130: bool = ((s_63_129.value()) != 0);
        // C s_63_131: const #0s : i
        let s_63_131: i128 = 0;
        // C s_63_132: const #0u : u64
        let s_63_132: u64 = 0;
        // D s_63_133: cast zx s_63_130 -> u64
        let s_63_133: u64 = (s_63_130 as u64);
        // C s_63_134: const #1u : u64
        let s_63_134: u64 = 1;
        // D s_63_135: and s_63_133 s_63_134
        let s_63_135: u64 = ((s_63_133) & (s_63_134));
        // D s_63_136: cmp-eq s_63_135 s_63_134
        let s_63_136: bool = ((s_63_135) == (s_63_134));
        // D s_63_137: lsl s_63_133 s_63_131
        let s_63_137: u64 = s_63_133 << s_63_131;
        // D s_63_138: or s_63_132 s_63_137
        let s_63_138: u64 = ((s_63_132) | (s_63_137));
        // D s_63_139: cmpl s_63_137
        let s_63_139: u64 = !s_63_137;
        // D s_63_140: and s_63_132 s_63_139
        let s_63_140: u64 = ((s_63_132) & (s_63_139));
        // D s_63_141: select s_63_136 s_63_138 s_63_140
        let s_63_141: u64 = if s_63_136 { s_63_138 } else { s_63_140 };
        // D s_63_142: cast trunc s_63_141 -> u8
        let s_63_142: bool = ((s_63_141) != 0);
        // D s_63_143: write-var c <= s_63_142
        fn_state.c = s_63_142;
        // C s_63_144: const #2s : i
        let s_63_144: i128 = 2;
        // D s_63_145: read-var descriptor:u32
        let s_63_145: u32 = fn_state.descriptor;
        // D s_63_146: cast zx s_63_145 -> bv
        let s_63_146: Bits = Bits::new(s_63_145 as u128, 32u16);
        // C s_63_147: const #1u : u64
        let s_63_147: u64 = 1;
        // D s_63_148: bit-extract s_63_146 s_63_144 s_63_147
        let s_63_148: Bits = (Bits::new(
            ((s_63_146) >> (s_63_144)).value(),
            u16::try_from(s_63_147).unwrap(),
        ));
        // D s_63_149: cast reint s_63_148 -> u8
        let s_63_149: bool = ((s_63_148.value()) != 0);
        // C s_63_150: const #0s : i
        let s_63_150: i128 = 0;
        // C s_63_151: const #0u : u64
        let s_63_151: u64 = 0;
        // D s_63_152: cast zx s_63_149 -> u64
        let s_63_152: u64 = (s_63_149 as u64);
        // C s_63_153: const #1u : u64
        let s_63_153: u64 = 1;
        // D s_63_154: and s_63_152 s_63_153
        let s_63_154: u64 = ((s_63_152) & (s_63_153));
        // D s_63_155: cmp-eq s_63_154 s_63_153
        let s_63_155: bool = ((s_63_154) == (s_63_153));
        // D s_63_156: lsl s_63_152 s_63_150
        let s_63_156: u64 = s_63_152 << s_63_150;
        // D s_63_157: or s_63_151 s_63_156
        let s_63_157: u64 = ((s_63_151) | (s_63_156));
        // D s_63_158: cmpl s_63_156
        let s_63_158: u64 = !s_63_156;
        // D s_63_159: and s_63_151 s_63_158
        let s_63_159: u64 = ((s_63_151) & (s_63_158));
        // D s_63_160: select s_63_155 s_63_157 s_63_159
        let s_63_160: u64 = if s_63_155 { s_63_157 } else { s_63_159 };
        // D s_63_161: cast trunc s_63_160 -> u8
        let s_63_161: bool = ((s_63_160) != 0);
        // D s_63_162: write-var b <= s_63_161
        fn_state.b = s_63_161;
        // C s_63_163: const #0s : i
        let s_63_163: i128 = 0;
        // D s_63_164: read-var descriptor:u32
        let s_63_164: u32 = fn_state.descriptor;
        // D s_63_165: cast zx s_63_164 -> bv
        let s_63_165: Bits = Bits::new(s_63_164 as u128, 32u16);
        // C s_63_166: const #1u : u64
        let s_63_166: u64 = 1;
        // D s_63_167: bit-extract s_63_165 s_63_163 s_63_166
        let s_63_167: Bits = (Bits::new(
            ((s_63_165) >> (s_63_163)).value(),
            u16::try_from(s_63_166).unwrap(),
        ));
        // D s_63_168: cast reint s_63_167 -> u8
        let s_63_168: bool = ((s_63_167.value()) != 0);
        // C s_63_169: const #0s : i
        let s_63_169: i128 = 0;
        // C s_63_170: const #0u : u64
        let s_63_170: u64 = 0;
        // D s_63_171: cast zx s_63_168 -> u64
        let s_63_171: u64 = (s_63_168 as u64);
        // C s_63_172: const #1u : u64
        let s_63_172: u64 = 1;
        // D s_63_173: and s_63_171 s_63_172
        let s_63_173: u64 = ((s_63_171) & (s_63_172));
        // D s_63_174: cmp-eq s_63_173 s_63_172
        let s_63_174: bool = ((s_63_173) == (s_63_172));
        // D s_63_175: lsl s_63_171 s_63_169
        let s_63_175: u64 = s_63_171 << s_63_169;
        // D s_63_176: or s_63_170 s_63_175
        let s_63_176: u64 = ((s_63_170) | (s_63_175));
        // D s_63_177: cmpl s_63_175
        let s_63_177: u64 = !s_63_175;
        // D s_63_178: and s_63_170 s_63_177
        let s_63_178: u64 = ((s_63_170) & (s_63_177));
        // D s_63_179: select s_63_174 s_63_176 s_63_178
        let s_63_179: u64 = if s_63_174 { s_63_176 } else { s_63_178 };
        // D s_63_180: cast trunc s_63_179 -> u8
        let s_63_180: bool = ((s_63_179) != 0);
        // D s_63_181: write-var pxn <= s_63_180
        fn_state.pxn = s_63_180;
        // C s_63_182: const #0u : u8
        let s_63_182: u8 = 0;
        // D s_63_183: write-var domain <= s_63_182
        fn_state.domain = s_63_182;
        // C s_63_184: const #5s : i
        let s_63_184: i128 = 5;
        // D s_63_185: read-var descriptor:u32
        let s_63_185: u32 = fn_state.descriptor;
        // D s_63_186: cast zx s_63_185 -> bv
        let s_63_186: Bits = Bits::new(s_63_185 as u128, 32u16);
        // C s_63_187: const #1s : i64
        let s_63_187: i64 = 1;
        // C s_63_188: cast zx s_63_187 -> i
        let s_63_188: i128 = (i128::try_from(s_63_187).unwrap());
        // C s_63_189: const #3s : i
        let s_63_189: i128 = 3;
        // C s_63_190: add s_63_189 s_63_188
        let s_63_190: i128 = (s_63_189 + s_63_188);
        // D s_63_191: bit-extract s_63_186 s_63_184 s_63_190
        let s_63_191: Bits = (Bits::new(
            ((s_63_186) >> (s_63_184)).value(),
            u16::try_from(s_63_190).unwrap(),
        ));
        // D s_63_192: cast reint s_63_191 -> u8
        let s_63_192: u8 = (s_63_191.value() as u8);
        // C s_63_193: const #20s : i
        let s_63_193: i128 = 20;
        // D s_63_194: read-var descriptor:u32
        let s_63_194: u32 = fn_state.descriptor;
        // D s_63_195: cast zx s_63_194 -> bv
        let s_63_195: Bits = Bits::new(s_63_194 as u128, 32u16);
        // C s_63_196: const #1s : i64
        let s_63_196: i64 = 1;
        // C s_63_197: cast zx s_63_196 -> i
        let s_63_197: i128 = (i128::try_from(s_63_196).unwrap());
        // C s_63_198: const #3s : i
        let s_63_198: i128 = 3;
        // C s_63_199: add s_63_198 s_63_197
        let s_63_199: i128 = (s_63_198 + s_63_197);
        // D s_63_200: bit-extract s_63_195 s_63_193 s_63_199
        let s_63_200: Bits = (Bits::new(
            ((s_63_195) >> (s_63_193)).value(),
            u16::try_from(s_63_199).unwrap(),
        ));
        // D s_63_201: cast reint s_63_200 -> u8
        let s_63_201: u8 = (s_63_200.value() as u8);
        // C s_63_202: const #24s : i
        let s_63_202: i128 = 24;
        // D s_63_203: read-var descriptor:u32
        let s_63_203: u32 = fn_state.descriptor;
        // D s_63_204: cast zx s_63_203 -> bv
        let s_63_204: Bits = Bits::new(s_63_203 as u128, 32u16);
        // C s_63_205: const #1s : i64
        let s_63_205: i64 = 1;
        // C s_63_206: cast zx s_63_205 -> i
        let s_63_206: i128 = (i128::try_from(s_63_205).unwrap());
        // C s_63_207: const #7s : i
        let s_63_207: i128 = 7;
        // C s_63_208: add s_63_207 s_63_206
        let s_63_208: i128 = (s_63_207 + s_63_206);
        // D s_63_209: bit-extract s_63_204 s_63_202 s_63_208
        let s_63_209: Bits = (Bits::new(
            ((s_63_204) >> (s_63_202)).value(),
            u16::try_from(s_63_208).unwrap(),
        ));
        // D s_63_210: cast reint s_63_209 -> u8
        let s_63_210: u8 = (s_63_209.value() as u8);
        // D s_63_211: cast zx s_63_201 -> bv
        let s_63_211: Bits = Bits::new(s_63_201 as u128, 4u16);
        // D s_63_212: cast zx s_63_210 -> bv
        let s_63_212: Bits = Bits::new(s_63_210 as u128, 8u16);
        // D s_63_213: cast reint s_63_211 -> u128
        let s_63_213: u128 = (s_63_211.value() as u128);
        // D s_63_214: size-of s_63_211
        let s_63_214: u16 = s_63_211.length();
        // D s_63_215: cast reint s_63_212 -> u128
        let s_63_215: u128 = (s_63_212.value() as u128);
        // D s_63_216: size-of s_63_212
        let s_63_216: u16 = s_63_212.length();
        // D s_63_217: lsl s_63_213 s_63_216
        let s_63_217: u128 = s_63_213 << s_63_216;
        // D s_63_218: or s_63_217 s_63_215
        let s_63_218: u128 = ((s_63_217) | (s_63_215));
        // D s_63_219: add s_63_214 s_63_216
        let s_63_219: u16 = (s_63_214 + s_63_216);
        // D s_63_220: create-bits s_63_218 s_63_219
        let s_63_220: Bits = Bits::new(s_63_218, s_63_219);
        // D s_63_221: cast reint s_63_220 -> u12
        let s_63_221: u16 = (s_63_220.value() as u16);
        // D s_63_222: cast zx s_63_192 -> bv
        let s_63_222: Bits = Bits::new(s_63_192 as u128, 4u16);
        // D s_63_223: cast zx s_63_221 -> bv
        let s_63_223: Bits = Bits::new(s_63_221 as u128, 12u16);
        // D s_63_224: cast reint s_63_222 -> u128
        let s_63_224: u128 = (s_63_222.value() as u128);
        // D s_63_225: size-of s_63_222
        let s_63_225: u16 = s_63_222.length();
        // D s_63_226: cast reint s_63_223 -> u128
        let s_63_226: u128 = (s_63_223.value() as u128);
        // D s_63_227: size-of s_63_223
        let s_63_227: u16 = s_63_223.length();
        // D s_63_228: lsl s_63_224 s_63_227
        let s_63_228: u128 = s_63_224 << s_63_227;
        // D s_63_229: or s_63_228 s_63_226
        let s_63_229: u128 = ((s_63_228) | (s_63_226));
        // D s_63_230: add s_63_225 s_63_227
        let s_63_230: u16 = (s_63_225 + s_63_227);
        // D s_63_231: create-bits s_63_229 s_63_230
        let s_63_231: Bits = Bits::new(s_63_229, s_63_230);
        // D s_63_232: cast reint s_63_231 -> u16
        let s_63_232: u16 = (s_63_231.value() as u16);
        // C s_63_233: const #56s : i
        let s_63_233: i128 = 56;
        // C s_63_234: const #0s : i
        let s_63_234: i128 = 0;
        // C s_63_235: const #16s : i
        let s_63_235: i128 = 16;
        // C s_63_236: const #24s : i
        let s_63_236: i128 = 24;
        // D s_63_237: cast zx s_63_232 -> bv
        let s_63_237: Bits = Bits::new(s_63_232 as u128, 16u16);
        // D s_63_238: call place_slice(s_63_233, s_63_237, s_63_234, s_63_235, s_63_236)
        let s_63_238: Bits = place_slice(
            state,
            tracer,
            s_63_233,
            s_63_237,
            s_63_234,
            s_63_235,
            s_63_236,
        );
        // D s_63_239: cast reint s_63_238 -> u56
        let s_63_239: u64 = (s_63_238.value() as u64);
        // D s_63_240: write-var walkstate.0.0 <= s_63_239
        fn_state.walkstate._0._0 = s_63_239;
        // C s_63_241: const #0u : u8
        let s_63_241: bool = false;
        // D s_63_242: write-var walkstate.5 <= s_63_241
        fn_state.walkstate._5 = s_63_241;
        // N s_63_243: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_64_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call __UNKNOWN_TTWState(s_65_0)
        let s_65_1: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_65_0,
        );
        // D s_65_2: read-var fault:struct
        let s_65_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_65_3: create-product struct = ["s_65_2", "s_65_1"]
        let s_65_3: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_65_2,
            _1: s_65_1,
        };
        // D s_65_4: write-var return_value <= s_65_3
        fn_state.return_value = s_65_3;
        // N s_65_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: read-var walkstate.6:struct
        let s_66_1: i128 = fn_state.walkstate._6;
        // D s_66_2: create-sum enum = 1:"s_66_1"
        let s_66_2: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_1(s_66_1);
        // D s_66_3: read-var fault:struct
        let s_66_3: ProductType1d757adad216cdef = fn_state.fault;
        // D s_66_4: read-var walkaddress:struct
        let s_66_4: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // D s_66_5: read-var walkaccess:struct
        let s_66_5: ProductType9878976b5bcce9c9 = fn_state.walkaccess;
        // D s_66_6: call AArch32_S2Translate(s_66_3, s_66_4, s_66_2, s_66_0, s_66_5)
        let s_66_6: ProductTypedc31059ca7e2391c = AArch32_S2Translate(
            state,
            tracer,
            s_66_3,
            s_66_4,
            s_66_2,
            s_66_0,
            s_66_5,
        );
        // D s_66_7: write-var ga#22272 <= s_66_6
        fn_state.ga_22272 = s_66_6;
        // D s_66_8: read-var ga#22272.0:struct
        let s_66_8: ProductType1d757adad216cdef = fn_state.ga_22272._0;
        // D s_66_9: read-var ga#22272.1:struct
        let s_66_9: ProductTypece7c66ccb2cab13e = fn_state.ga_22272._1;
        // D s_66_10: write-var s2fault <= s_66_8
        fn_state.s2fault = s_66_8;
        // D s_66_11: write-var s2walkaddress <= s_66_9
        fn_state.s2walkaddress = s_66_9;
        // D s_66_12: read-var s2fault.16:struct
        let s_66_12: u32 = fn_state.s2fault._16;
        // C s_66_13: const #0u : u32
        let s_66_13: u32 = 0;
        // D s_66_14: cmp-eq s_66_12 s_66_13
        let s_66_14: bool = ((s_66_12) == (s_66_13));
        // N s_66_15: branch s_66_14 b68 b67
        if s_66_14 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_67_0: const #32s : i64
        let s_67_0: i64 = 32;
        // D s_67_1: read-var ee:u8
        let s_67_1: bool = fn_state.ee;
        // D s_67_2: read-var s2walkaddress:struct
        let s_67_2: ProductTypece7c66ccb2cab13e = fn_state.s2walkaddress;
        // D s_67_3: read-var walkaccess:struct
        let s_67_3: ProductType9878976b5bcce9c9 = fn_state.walkaccess;
        // D s_67_4: read-var fault:struct
        let s_67_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_67_5: read-var translation_info:struct
        let s_67_5: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_67_6: call FetchDescriptor(s_67_1, s_67_2, s_67_3, s_67_4, s_67_0, s_67_5)
        let s_67_6: ProductTypeb4cea7287e2eb9d6 = FetchDescriptor(
            state,
            tracer,
            s_67_1,
            s_67_2,
            s_67_3,
            s_67_4,
            s_67_0,
            s_67_5,
        );
        // D s_67_7: write-var gs#454060 <= s_67_6
        fn_state.gs_454060 = s_67_6;
        // D s_67_8: read-var gs#454060.0:struct
        let s_67_8: ProductType1d757adad216cdef = fn_state.gs_454060._0;
        // D s_67_9: read-var gs#454060.1:struct
        let s_67_9: Bits = fn_state.gs_454060._1;
        // D s_67_10: cast reint s_67_9 -> u32
        let s_67_10: u32 = (s_67_9.value() as u32);
        // D s_67_11: write-var fault <= s_67_8
        fn_state.fault = s_67_8;
        // D s_67_12: write-var descriptor <= s_67_10
        fn_state.descriptor = s_67_10;
        // N s_67_13: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_68_0: const #() : ()
        let s_68_0: () = ();
        // S s_68_1: call __UNKNOWN_TTWState(s_68_0)
        let s_68_1: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_68_0,
        );
        // D s_68_2: read-var s2fault:struct
        let s_68_2: ProductType1d757adad216cdef = fn_state.s2fault;
        // D s_68_3: create-product struct = ["s_68_2", "s_68_1"]
        let s_68_3: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_68_2,
            _1: s_68_1,
        };
        // D s_68_4: write-var return_value <= s_68_3
        fn_state.return_value = s_68_3;
        // N s_68_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_69_0: read-var accdesc.25:struct
        let s_69_0: u32 = fn_state.accdesc._25;
        // D s_69_1: call AArch32_EL2Enabled(s_69_0)
        let s_69_1: bool = AArch32_EL2Enabled(state, tracer, s_69_0);
        // D s_69_2: write-var gs#28776 <= s_69_1
        fn_state.gs_28776 = s_69_1;
        // N s_69_3: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_70_0: const #31s : i
        let s_70_0: i128 = 31;
        // D s_70_1: read-var nshadow#535:i64
        let s_70_1: i64 = fn_state.nshadow_535;
        // D s_70_2: cast zx s_70_1 -> i
        let s_70_2: i128 = (i128::try_from(s_70_1).unwrap());
        // D s_70_3: sub s_70_0 s_70_2
        let s_70_3: i128 = ((s_70_0) - (s_70_2));
        // D s_70_4: cast reint s_70_3 -> i64
        let s_70_4: i64 = (s_70_3 as i64);
        // C s_70_5: const #32s : i
        let s_70_5: i128 = 32;
        // C s_70_6: const #20s : i
        let s_70_6: i128 = 20;
        // C s_70_7: const #2s : i
        let s_70_7: i128 = 2;
        // D s_70_8: read-var va:u32
        let s_70_8: u32 = fn_state.va;
        // D s_70_9: cast zx s_70_8 -> bv
        let s_70_9: Bits = Bits::new(s_70_8 as u128, 32u16);
        // D s_70_10: cast zx s_70_4 -> i
        let s_70_10: i128 = (i128::try_from(s_70_4).unwrap());
        // D s_70_11: call place_subrange(s_70_5, s_70_9, s_70_10, s_70_6, s_70_7)
        let s_70_11: Bits = place_subrange(
            state,
            tracer,
            s_70_5,
            s_70_9,
            s_70_10,
            s_70_6,
            s_70_7,
        );
        // D s_70_12: cast reint s_70_11 -> u32
        let s_70_12: u32 = (s_70_11.value() as u32);
        // D s_70_13: write-var index <= s_70_12
        fn_state.index = s_70_12;
        // N s_70_14: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_71_0: read-var walkstate.7:struct
        let s_71_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_71_1: write-var ga#22257 <= s_71_0
        fn_state.ga_22257 = s_71_0;
        // D s_71_2: read-var ga#22257.5:struct
        let s_71_2: u32 = fn_state.ga_22257._5;
        // D s_71_3: write-var walkaddress.2.5 <= s_71_2
        fn_state.walkaddress._2._5 = s_71_2;
        // N s_71_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_72_0: const #"Apply effective shareability at stage 1" : str
        let s_72_0: &'static str = "Apply effective shareability at stage 1";
        // S s_72_1: call __IMPDEF_boolean(s_72_0)
        let s_72_1: bool = u__IMPDEF_boolean(state, tracer, s_72_0);
        // S s_72_2: not s_72_1
        let s_72_2: bool = !s_72_1;
        // D s_72_3: write-var gs#28757 <= s_72_2
        fn_state.gs_28757 = s_72_2;
        // N s_72_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_73_0: read-var accdesc.25:struct
        let s_73_0: u32 = fn_state.accdesc._25;
        // C s_73_1: const #3u : u32
        let s_73_1: u32 = 3;
        // D s_73_2: cmp-eq s_73_0 s_73_1
        let s_73_2: bool = ((s_73_0) == (s_73_1));
        // C s_73_3: const #432u : u32
        let s_73_3: u32 = 432;
        // D s_73_4: read-reg s_73_3:u8
        let s_73_4: u8 = {
            let value = state.read_register::<u8>(s_73_3 as isize);
            tracer.read_register(s_73_3 as isize, value);
            value
        };
        // D s_73_5: call ELStateUsingAArch32(s_73_4, s_73_2)
        let s_73_5: bool = ELStateUsingAArch32(state, tracer, s_73_4, s_73_2);
        // N s_73_6: branch s_73_5 b76 b74
        if s_73_5 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_74_0: const #102552u : u32
        let s_74_0: u32 = 102552;
        // D s_74_1: read-reg s_74_0:struct
        let s_74_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: call _get_HCR_EL2_Type_VM(s_74_1)
        let s_74_2: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_74_1);
        // D s_74_3: write-var ga#22252 <= s_74_2
        fn_state.ga_22252 = s_74_2;
        // N s_74_4: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_75_0: read-var ga#22252:u8
        let s_75_0: bool = fn_state.ga_22252;
        // D s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 1u16);
        // C s_75_2: const #1u : u8
        let s_75_2: bool = true;
        // C s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_4: cmp-eq s_75_1 s_75_3
        let s_75_4: bool = ((s_75_1) == (s_75_3));
        // D s_75_5: write-var gs#28756 <= s_75_4
        fn_state.gs_28756 = s_75_4;
        // N s_75_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call HCR_read(s_76_0)
        let s_76_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_76_0);
        // S s_76_2: call _get_HCR_Type_VM(s_76_1)
        let s_76_2: bool = u_get_HCR_Type_VM(state, tracer, s_76_1);
        // D s_76_3: write-var ga#22252 <= s_76_2
        fn_state.ga_22252 = s_76_2;
        // N s_76_4: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_77_0: read-var accdesc.25:struct
        let s_77_0: u32 = fn_state.accdesc._25;
        // D s_77_1: call AArch32_EL2Enabled(s_77_0)
        let s_77_1: bool = AArch32_EL2Enabled(state, tracer, s_77_0);
        // D s_77_2: write-var gs#28755 <= s_77_1
        fn_state.gs_28755 = s_77_1;
        // N s_77_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_78_0: const #() : ()
        let s_78_0: () = ();
        // S s_78_1: call NormalNCMemAttr(s_78_0)
        let s_78_1: ProductTypef170cab34335b70c = NormalNCMemAttr(state, tracer, s_78_0);
        // D s_78_2: write-var walkaddress.2 <= s_78_1
        fn_state.walkaddress._2 = s_78_1;
        // D s_78_3: read-var walkstate.7:struct
        let s_78_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_78_4: write-var ga#22245 <= s_78_3
        fn_state.ga_22245 = s_78_3;
        // D s_78_5: read-var ga#22245.7:struct
        let s_78_5: bool = fn_state.ga_22245._7;
        // D s_78_6: write-var walkaddress.2.7 <= s_78_5
        fn_state.walkaddress._2._7 = s_78_5;
        // N s_78_7: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_79_0: const #1u : u32
        let s_79_0: u32 = 1;
        // D s_79_1: write-var baseaddress.1 <= s_79_0
        fn_state.baseaddress._1 = s_79_0;
        // N s_79_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_80_0: const #1s : i
        let s_80_0: i128 = 1;
        // D s_80_1: write-var fault.9 <= s_80_0
        fn_state.fault._9 = s_80_0;
        // C s_80_2: const #6u : u32
        let s_80_2: u32 = 6;
        // D s_80_3: write-var fault.16 <= s_80_2
        fn_state.fault._16 = s_80_2;
        // C s_80_4: const #() : ()
        let s_80_4: () = ();
        // S s_80_5: call __UNKNOWN_TTWState(s_80_4)
        let s_80_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_80_4,
        );
        // D s_80_6: read-var fault:struct
        let s_80_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_80_7: create-product struct = ["s_80_6", "s_80_5"]
        let s_80_7: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_80_6,
            _1: s_80_5,
        };
        // D s_80_8: write-var return_value <= s_80_7
        fn_state.return_value = s_80_7;
        // N s_80_9: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_81_0: read-var ttbr0:struct
        let s_81_0: ProductType5c790c8ef59cc8b2 = fn_state.ttbr0;
        // D s_81_1: call _get_TTBR0_Type_TTB0(s_81_0)
        let s_81_1: u32 = u_get_TTBR0_Type_TTB0(state, tracer, s_81_0);
        // C s_81_2: const #7s : i
        let s_81_2: i128 = 7;
        // S s_81_3: call Zeros(s_81_2)
        let s_81_3: Bits = Zeros(state, tracer, s_81_2);
        // S s_81_4: cast reint s_81_3 -> u8
        let s_81_4: u8 = (s_81_3.value() as u8);
        // D s_81_5: cast zx s_81_1 -> bv
        let s_81_5: Bits = Bits::new(s_81_1 as u128, 25u16);
        // S s_81_6: cast zx s_81_4 -> bv
        let s_81_6: Bits = Bits::new(s_81_4 as u128, 7u16);
        // D s_81_7: cast reint s_81_5 -> u128
        let s_81_7: u128 = (s_81_5.value() as u128);
        // D s_81_8: size-of s_81_5
        let s_81_8: u16 = s_81_5.length();
        // S s_81_9: cast reint s_81_6 -> u128
        let s_81_9: u128 = (s_81_6.value() as u128);
        // D s_81_10: size-of s_81_6
        let s_81_10: u16 = s_81_6.length();
        // D s_81_11: lsl s_81_7 s_81_10
        let s_81_11: u128 = s_81_7 << s_81_10;
        // D s_81_12: or s_81_11 s_81_9
        let s_81_12: u128 = ((s_81_11) | (s_81_9));
        // D s_81_13: add s_81_8 s_81_10
        let s_81_13: u16 = (s_81_8 + s_81_10);
        // D s_81_14: create-bits s_81_12 s_81_13
        let s_81_14: Bits = Bits::new(s_81_12, s_81_13);
        // D s_81_15: cast reint s_81_14 -> u32
        let s_81_15: u32 = (s_81_14.value() as u32);
        // D s_81_16: write-var ttb <= s_81_15
        fn_state.ttb = s_81_15;
        // D s_81_17: read-var ttbcr:struct
        let s_81_17: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_81_18: call _get_TTBCR_Type_PD0(s_81_17)
        let s_81_18: bool = u_get_TTBCR_Type_PD0(state, tracer, s_81_17);
        // D s_81_19: write-var pd <= s_81_18
        fn_state.pd = s_81_18;
        // D s_81_20: read-var ttbr0:struct
        let s_81_20: ProductType5c790c8ef59cc8b2 = fn_state.ttbr0;
        // D s_81_21: call _get_TTBR0_Type_IRGN(s_81_20)
        let s_81_21: u8 = u_get_TTBR0_Type_IRGN(state, tracer, s_81_20);
        // D s_81_22: write-var irgn <= s_81_21
        fn_state.irgn = s_81_21;
        // D s_81_23: read-var ttbr0:struct
        let s_81_23: ProductType5c790c8ef59cc8b2 = fn_state.ttbr0;
        // D s_81_24: call _get_TTBR0_Type_RGN(s_81_23)
        let s_81_24: u8 = u_get_TTBR0_Type_RGN(state, tracer, s_81_23);
        // D s_81_25: write-var rgn <= s_81_24
        fn_state.rgn = s_81_24;
        // D s_81_26: read-var ttbr0:struct
        let s_81_26: ProductType5c790c8ef59cc8b2 = fn_state.ttbr0;
        // D s_81_27: call _get_TTBR0_Type_S(s_81_26)
        let s_81_27: bool = u_get_TTBR0_Type_S(state, tracer, s_81_26);
        // D s_81_28: write-var s <= s_81_27
        fn_state.s = s_81_27;
        // D s_81_29: read-var ttbr0:struct
        let s_81_29: ProductType5c790c8ef59cc8b2 = fn_state.ttbr0;
        // D s_81_30: call _get_TTBR0_Type_NOS(s_81_29)
        let s_81_30: bool = u_get_TTBR0_Type_NOS(state, tracer, s_81_29);
        // D s_81_31: write-var nos <= s_81_30
        fn_state.nos = s_81_30;
        // C s_81_32: const #0u : u32
        let s_81_32: u32 = 0;
        // D s_81_33: write-var varange <= s_81_32
        fn_state.varange = s_81_32;
        // N s_81_34: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_82_0: const #1u : u8
        let s_82_0: bool = true;
        // D s_82_1: write-var gs#28717 <= s_82_0
        fn_state.gs_28717 = s_82_0;
        // N s_82_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call SCTLR_NS_read(s_83_0)
        let s_83_1: ProductType700c18a878c5601b = SCTLR_NS_read(state, tracer, s_83_0);
        // D s_83_2: write-var sctlr <= s_83_1
        fn_state.sctlr = s_83_1;
        // C s_83_3: const #() : ()
        let s_83_3: () = ();
        // S s_83_4: call TTBCR_NS_read(s_83_3)
        let s_83_4: ProductType700c18a878c5601b = TTBCR_NS_read(state, tracer, s_83_3);
        // D s_83_5: write-var ttbcr <= s_83_4
        fn_state.ttbcr = s_83_4;
        // C s_83_6: const #11616u : u32
        let s_83_6: u32 = 11616;
        // D s_83_7: read-reg s_83_6:struct
        let s_83_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_83_6 as isize);
            tracer.read_register(s_83_6 as isize, value);
            value
        };
        // D s_83_8: write-var ttbr0 <= s_83_7
        fn_state.ttbr0 = s_83_7;
        // C s_83_9: const #20024u : u32
        let s_83_9: u32 = 20024;
        // D s_83_10: read-reg s_83_9:struct
        let s_83_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_83_9 as isize);
            tracer.read_register(s_83_9 as isize, value);
            value
        };
        // D s_83_11: write-var ttbr1 <= s_83_10
        fn_state.ttbr1 = s_83_10;
        // N s_83_12: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_84_0: const #16456u : u32
        let s_84_0: u32 = 16456;
        // D s_84_1: read-reg s_84_0:struct
        let s_84_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: write-var sctlr <= s_84_1
        fn_state.sctlr = s_84_1;
        // C s_84_3: const #15368u : u32
        let s_84_3: u32 = 15368;
        // D s_84_4: read-reg s_84_3:struct
        let s_84_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_84_3 as isize);
            tracer.read_register(s_84_3 as isize, value);
            value
        };
        // D s_84_5: write-var ttbcr <= s_84_4
        fn_state.ttbcr = s_84_4;
        // C s_84_6: const #101800u : u32
        let s_84_6: u32 = 101800;
        // D s_84_7: read-reg s_84_6:struct
        let s_84_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_6 as isize);
            tracer.read_register(s_84_6 as isize, value);
            value
        };
        // D s_84_8: write-var ttbr0 <= s_84_7
        fn_state.ttbr0 = s_84_7;
        // C s_84_9: const #19120u : u32
        let s_84_9: u32 = 19120;
        // D s_84_10: read-reg s_84_9:struct
        let s_84_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_9 as isize);
            tracer.read_register(s_84_9 as isize, value);
            value
        };
        // D s_84_11: write-var ttbr1 <= s_84_10
        fn_state.ttbr1 = s_84_10;
        // N s_84_12: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_85_0: read-var accdesc.25:struct
        let s_85_0: u32 = fn_state.accdesc._25;
        // D s_85_1: read-var regime:u32
        let s_85_1: u32 = fn_state.regime;
        // D s_85_2: read-var va:u32
        let s_85_2: u32 = fn_state.va;
        // D s_85_3: call AArch32_GetS1TLBContext(s_85_1, s_85_0, s_85_2)
        let s_85_3: ProductTypec0d0fb0603850c4c = AArch32_GetS1TLBContext(
            state,
            tracer,
            s_85_1,
            s_85_0,
            s_85_2,
        );
        // D s_85_4: write-var tlbcontext <= s_85_3
        fn_state.tlbcontext = s_85_3;
        // D s_85_5: read-var tlbcontext:struct
        let s_85_5: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_85_6: call S1TLBLookup(s_85_5)
        let s_85_6: ProductTypeeb828c17bbe5e68 = S1TLBLookup(state, tracer, s_85_5);
        // D s_85_7: write-var tlbentry <= s_85_6
        fn_state.tlbentry = s_85_6;
        // D s_85_8: read-var tlbentry.1:struct
        let s_85_8: bool = fn_state.tlbentry._1;
        // N s_85_9: branch s_85_8 b87 b86
        if s_85_8 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_86_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_87_0: read-var tlbentry.0:struct
        let s_87_0: ProductTypee47dd77b186df56e = fn_state.tlbentry._0;
        // D s_87_1: write-var ga#22223 <= s_87_0
        fn_state.ga_22223 = s_87_0;
        // D s_87_2: read-var ga#22223.5:struct
        let s_87_2: ProductType96e7acababe246a1 = fn_state.ga_22223._5;
        // D s_87_3: write-var finalwalkstateshadow#536 <= s_87_2
        fn_state.finalwalkstateshadow_536 = s_87_2;
        // D s_87_4: read-var finalwalkstateshadow#536.6:struct
        let s_87_4: i128 = fn_state.finalwalkstateshadow_536._6;
        // D s_87_5: write-var fault.9 <= s_87_4
        fn_state.fault._9 = s_87_4;
        // D s_87_6: read-var fault:struct
        let s_87_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_87_7: create-product struct = ["s_87_6", "s_87_2"]
        let s_87_7: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_87_6,
            _1: s_87_2,
        };
        // D s_87_8: write-var return_value <= s_87_7
        fn_state.return_value = s_87_7;
        // N s_87_9: jump b32
        return block_32(state, tracer, fn_state);
    }
}
