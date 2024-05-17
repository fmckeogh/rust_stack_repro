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
use AArch32_EL2Enabled::*;
use HaveAArch32EL::*;
use TTBCR_read::*;
use AArch32_GetVARange::*;
use S1TLBCache::*;
use AArch32_GetS1TLBContext::*;
use AArch32_DecodeDescriptorTypeLD::*;
use ELStateUsingAArch32::*;
use IsZero::*;
use NormalNCMemAttr::*;
use S1DecodeMemAttrs::*;
use u_get_HCR_EL2_Type_VM::*;
use TranslationSize::*;
use AArch32_MAIRAttr::*;
use u__UNKNOWN_TTWState::*;
use FetchDescriptor::*;
use u_get_TTBCR_Type_EPD0::*;
use AArch32_S2Translate::*;
use u_get_TTBCR_Type_EPD1::*;
use place_subrange::*;
use HasUnprivileged::*;
use TTBR0_read::*;
use AArch32_S1DCacheEnabled::*;
use AArch32_S1IASize::*;
use EffectiveShareability::*;
use CreateAccDescS1TTW::*;
use TGxGranuleBits::*;
use u__IMPDEF_boolean::*;
use HCR_read::*;
use ContiguousSize::*;
use u_get_HCR_Type_VM::*;
use TTBR1_read::*;
use u__id::*;
use S1TLBLookup::*;
use TTBCR_NS_read::*;
use u_get_TTBCR_Type_EAE::*;
use WalkMemAttrs::*;
use fdiv_int::*;
use common::*;
pub fn AArch32_S1WalkLD<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    regime: u32,
    walkparams: ProductTypeef284266e139aee2,
    accdesc: ProductType9878976b5bcce9c9,
    va: u32,
) -> ProductType201519a0f62623dc {
    #[derive(Default)]
    struct FunctionState {
        tlbcontext: ProductTypec0d0fb0603850c4c,
        levels: i128,
        finalwalkstateshadow_524: ProductType96e7acababe246a1,
        gs_28257: bool,
        gs_28341: bool,
        ga_21964: ProductTypef170cab34335b70c,
        stride: i128,
        indexmsbshadow_522: i128,
        baseaddress: ProductTypeda0231e9dc169f81,
        ttbr1: u64,
        ga_21902: ProductTypebf05c51f33174538,
        varange: u32,
        translation_info: ProductTypeb525737120e184b3,
        gs_28266: bool,
        walkaddress: ProductTypece7c66ccb2cab13e,
        baselsb: i128,
        gs_28227: bool,
        ttbr: u64,
        indexmsb: i128,
        ga_21873: ProductTypedc31059ca7e2391c,
        iasize: i128,
        tlbrecord: ProductTypee47dd77b186df56e,
        gs_453683: ProductTypeb4cea7287e2eb9d6,
        ttbr0: u64,
        gs_28256: bool,
        ga_21949: ProductTypeda0231e9dc169f81,
        gs_28243: bool,
        desctype: u32,
        ga_21906: ProductTypebf05c51f33174538,
        descriptor: u64,
        walkaccess: ProductType9878976b5bcce9c9,
        startlevel: i128,
        fault: ProductType1d757adad216cdef,
        s2fault: ProductType1d757adad216cdef,
        ga_21802: ProductTypee47dd77b186df56e,
        gs_28187: bool,
        gs_28295: bool,
        granulebits: i128,
        epd: bool,
        ga_21808: ProductType5c790c8ef59cc8b2,
        gs_28245: bool,
        return_value: ProductType201519a0f62623dc,
        indexlsbshadow_521: i128,
        gs_28336: bool,
        s2walkaddress: ProductTypece7c66ccb2cab13e,
        ttbcr: ProductType700c18a878c5601b,
        ga_21958: ProductTypeda0231e9dc169f81,
        gs_28346: bool,
        ga_21859: ProductTypef170cab34335b70c,
        ga_21854: bool,
        txsz: u8,
        walkstate: ProductType96e7acababe246a1,
        ga_21864: ProductTypeda0231e9dc169f81,
        ga_21809: ProductType5c790c8ef59cc8b2,
        ga_21867: ProductTypeda0231e9dc169f81,
        indexlsbshadow_523: i128,
        ga_21909: ProductTypebf05c51f33174538,
        indexlsb: i128,
        tlbentry: ProductTypeeb828c17bbe5e68,
        gs_453685: ProductTypeb4cea7287e2eb9d6,
        ga_21894: ProductTypeda0231e9dc169f81,
        gs_28244: bool,
        ga_21847: ProductTypef170cab34335b70c,
        fault_in: ProductType1d757adad216cdef,
        regime: u32,
        walkparams: ProductTypeef284266e139aee2,
        accdesc: ProductType9878976b5bcce9c9,
        va: u32,
    }
    let fn_state = FunctionState {
        fault_in,
        regime,
        walkparams,
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
        // N s_0_4: branch s_0_3 b111 b1
        if s_0_3 {
            return block_111(state, tracer, fn_state);
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
        // C s_2_1: const #2u : u32
        let s_2_1: u32 = 2;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b110 b3
        if s_2_2 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_3_0: read-var walkparams.32:struct
        let s_3_0: u8 = fn_state.walkparams._32;
        // D s_3_1: read-var walkparams.33:struct
        let s_3_1: u8 = fn_state.walkparams._33;
        // D s_3_2: read-var va:u32
        let s_3_2: u32 = fn_state.va;
        // D s_3_3: call AArch32_GetVARange(s_3_2, s_3_0, s_3_1)
        let s_3_3: u32 = AArch32_GetVARange(state, tracer, s_3_2, s_3_0, s_3_1);
        // D s_3_4: write-var varange <= s_3_3
        fn_state.varange = s_3_3;
        // D s_3_5: read-var regime:u32
        let s_3_5: u32 = fn_state.regime;
        // C s_3_6: const #1u : u32
        let s_3_6: u32 = 1;
        // D s_3_7: cmp-eq s_3_5 s_3_6
        let s_3_7: bool = ((s_3_5) == (s_3_6));
        // N s_3_8: branch s_3_7 b109 b4
        if s_3_7 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_4_0: const #424u : u32
        let s_4_0: u32 = 424;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call HaveAArch32EL(s_4_1)
        let s_4_2: bool = HaveAArch32EL(state, tracer, s_4_1);
        // N s_4_3: branch s_4_2 b108 b5
        if s_4_2 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call TTBCR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_5_0);
        // D s_5_2: write-var ttbcr <= s_5_1
        fn_state.ttbcr = s_5_1;
        // C s_5_3: const #() : ()
        let s_5_3: () = ();
        // S s_5_4: call TTBR0_read(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = TTBR0_read(state, tracer, s_5_3);
        // D s_5_5: write-var ga#21808 <= s_5_4
        fn_state.ga_21808 = s_5_4;
        // D s_5_6: read-var ga#21808.0:struct
        let s_5_6: u64 = fn_state.ga_21808._0;
        // D s_5_7: write-var ttbr0 <= s_5_6
        fn_state.ttbr0 = s_5_6;
        // C s_5_8: const #() : ()
        let s_5_8: () = ();
        // S s_5_9: call TTBR1_read(s_5_8)
        let s_5_9: ProductType5c790c8ef59cc8b2 = TTBR1_read(state, tracer, s_5_8);
        // D s_5_10: write-var ga#21809 <= s_5_9
        fn_state.ga_21809 = s_5_9;
        // D s_5_11: read-var ga#21809.0:struct
        let s_5_11: u64 = fn_state.ga_21809._0;
        // D s_5_12: write-var ttbr1 <= s_5_11
        fn_state.ttbr1 = s_5_11;
        // N s_5_13: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_6_0: read-var ttbcr:struct
        let s_6_0: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_6_1: call _get_TTBCR_Type_EAE(s_6_0)
        let s_6_1: bool = u_get_TTBCR_Type_EAE(state, tracer, s_6_0);
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 1u16);
        // C s_6_3: const #1u : u8
        let s_6_3: bool = true;
        // C s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 1u16);
        // D s_6_5: cmp-eq s_6_2 s_6_4
        let s_6_5: bool = ((s_6_2) == (s_6_4));
        // N s_6_6: assert s_6_5
        let s_6_6: () = assert!(s_6_5);
        // D s_6_7: read-var varange:u32
        let s_6_7: u32 = fn_state.varange;
        // C s_6_8: const #0u : u32
        let s_6_8: u32 = 0;
        // D s_6_9: cmp-eq s_6_7 s_6_8
        let s_6_9: bool = ((s_6_7) == (s_6_8));
        // N s_6_10: branch s_6_9 b107 b7
        if s_6_9 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_7_0: read-var walkparams.33:struct
        let s_7_0: u8 = fn_state.walkparams._33;
        // D s_7_1: write-var txsz <= s_7_0
        fn_state.txsz = s_7_0;
        // D s_7_2: read-var ttbr1:u64
        let s_7_2: u64 = fn_state.ttbr1;
        // D s_7_3: write-var ttbr <= s_7_2
        fn_state.ttbr = s_7_2;
        // D s_7_4: read-var ttbcr:struct
        let s_7_4: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_7_5: call _get_TTBCR_Type_EPD1(s_7_4)
        let s_7_5: bool = u_get_TTBCR_Type_EPD1(state, tracer, s_7_4);
        // D s_7_6: write-var epd <= s_7_5
        fn_state.epd = s_7_5;
        // N s_7_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_8_0: read-var regime:u32
        let s_8_0: u32 = fn_state.regime;
        // C s_8_1: const #2u : u32
        let s_8_1: u32 = 2;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b106 b9
        if s_8_2 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#28187 <= s_9_0
        fn_state.gs_28187 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_10_0: read-var gs#28187:u8
        let s_10_0: bool = fn_state.gs_28187;
        // N s_10_1: branch s_10_0 b105 b11
        if s_10_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_11_0: read-var txsz:u8
        let s_11_0: u8 = fn_state.txsz;
        // D s_11_1: call AArch32_S1IASize(s_11_0)
        let s_11_1: i128 = AArch32_S1IASize(state, tracer, s_11_0);
        // D s_11_2: write-var iasize <= s_11_1
        fn_state.iasize = s_11_1;
        // D s_11_3: read-var walkparams.36:struct
        let s_11_3: u32 = fn_state.walkparams._36;
        // D s_11_4: call TGxGranuleBits(s_11_3)
        let s_11_4: i128 = TGxGranuleBits(state, tracer, s_11_3);
        // D s_11_5: write-var granulebits <= s_11_4
        fn_state.granulebits = s_11_4;
        // C s_11_6: const #3s : i
        let s_11_6: i128 = 3;
        // D s_11_7: read-var granulebits:i
        let s_11_7: i128 = fn_state.granulebits;
        // D s_11_8: sub s_11_7 s_11_6
        let s_11_8: i128 = ((s_11_7) - (s_11_6));
        // D s_11_9: write-var stride <= s_11_8
        fn_state.stride = s_11_8;
        // C s_11_10: const #1s : i
        let s_11_10: i128 = 1;
        // D s_11_11: read-var iasize:i
        let s_11_11: i128 = fn_state.iasize;
        // D s_11_12: sub s_11_11 s_11_10
        let s_11_12: i128 = ((s_11_11) - (s_11_10));
        // D s_11_13: read-var granulebits:i
        let s_11_13: i128 = fn_state.granulebits;
        // D s_11_14: sub s_11_12 s_11_13
        let s_11_14: i128 = ((s_11_12) - (s_11_13));
        // D s_11_15: read-var stride:i
        let s_11_15: i128 = fn_state.stride;
        // D s_11_16: call fdiv_int(s_11_14, s_11_15)
        let s_11_16: i128 = fdiv_int(state, tracer, s_11_14, s_11_15);
        // C s_11_17: const #800u : u32
        let s_11_17: u32 = 800;
        // D s_11_18: read-reg s_11_17:i64
        let s_11_18: i64 = {
            let value = state.read_register::<i64>(s_11_17 as isize);
            tracer.read_register(s_11_17 as isize, value);
            value
        };
        // D s_11_19: cast zx s_11_18 -> i
        let s_11_19: i128 = (i128::try_from(s_11_18).unwrap());
        // D s_11_20: sub s_11_19 s_11_16
        let s_11_20: i128 = ((s_11_19) - (s_11_16));
        // D s_11_21: write-var startlevel <= s_11_20
        fn_state.startlevel = s_11_20;
        // C s_11_22: const #800u : u32
        let s_11_22: u32 = 800;
        // D s_11_23: read-reg s_11_22:i64
        let s_11_23: i64 = {
            let value = state.read_register::<i64>(s_11_22 as isize);
            tracer.read_register(s_11_22 as isize, value);
            value
        };
        // D s_11_24: cast zx s_11_23 -> i
        let s_11_24: i128 = (i128::try_from(s_11_23).unwrap());
        // D s_11_25: read-var startlevel:i
        let s_11_25: i128 = fn_state.startlevel;
        // D s_11_26: sub s_11_24 s_11_25
        let s_11_26: i128 = ((s_11_24) - (s_11_25));
        // D s_11_27: write-var levels <= s_11_26
        fn_state.levels = s_11_26;
        // C s_11_28: const #40s : i
        let s_11_28: i128 = 40;
        // D s_11_29: read-var ttbr:u64
        let s_11_29: u64 = fn_state.ttbr;
        // D s_11_30: cast zx s_11_29 -> bv
        let s_11_30: Bits = Bits::new(s_11_29 as u128, 64u16);
        // C s_11_31: const #1s : i64
        let s_11_31: i64 = 1;
        // C s_11_32: cast zx s_11_31 -> i
        let s_11_32: i128 = (i128::try_from(s_11_31).unwrap());
        // C s_11_33: const #7s : i
        let s_11_33: i128 = 7;
        // C s_11_34: add s_11_33 s_11_32
        let s_11_34: i128 = (s_11_33 + s_11_32);
        // D s_11_35: bit-extract s_11_30 s_11_28 s_11_34
        let s_11_35: Bits = (Bits::new(
            ((s_11_30) >> (s_11_28)).value(),
            u16::try_from(s_11_34).unwrap(),
        ));
        // D s_11_36: cast reint s_11_35 -> u8
        let s_11_36: u8 = (s_11_35.value() as u8);
        // D s_11_37: cast zx s_11_36 -> bv
        let s_11_37: Bits = Bits::new(s_11_36 as u128, 8u16);
        // D s_11_38: call IsZero(s_11_37)
        let s_11_38: bool = IsZero(state, tracer, s_11_37);
        // D s_11_39: not s_11_38
        let s_11_39: bool = !s_11_38;
        // N s_11_40: branch s_11_39 b104 b12
        if s_11_39 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_12_0: read-var levels:i
        let s_12_0: i128 = fn_state.levels;
        // D s_12_1: read-var stride:i
        let s_12_1: i128 = fn_state.stride;
        // D s_12_2: mul s_12_0 s_12_1
        let s_12_2: i128 = ((s_12_0) * (s_12_1));
        // D s_12_3: read-var granulebits:i
        let s_12_3: i128 = fn_state.granulebits;
        // D s_12_4: add s_12_2 s_12_3
        let s_12_4: i128 = (s_12_2 + s_12_3);
        // D s_12_5: read-var iasize:i
        let s_12_5: i128 = fn_state.iasize;
        // D s_12_6: sub s_12_5 s_12_4
        let s_12_6: i128 = ((s_12_5) - (s_12_4));
        // C s_12_7: const #3s : i
        let s_12_7: i128 = 3;
        // D s_12_8: add s_12_6 s_12_7
        let s_12_8: i128 = (s_12_6 + s_12_7);
        // D s_12_9: write-var baselsb <= s_12_8
        fn_state.baselsb = s_12_8;
        // D s_12_10: read-var accdesc.25:struct
        let s_12_10: u32 = fn_state.accdesc._25;
        // C s_12_11: const #3u : u32
        let s_12_11: u32 = 3;
        // D s_12_12: cmp-eq s_12_10 s_12_11
        let s_12_12: bool = ((s_12_10) == (s_12_11));
        // N s_12_13: branch s_12_12 b103 b13
        if s_12_12 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_13_0: const #0u : u32
        let s_13_0: u32 = 0;
        // D s_13_1: write-var baseaddress.1 <= s_13_0
        fn_state.baseaddress._1 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_14_0: read-var baselsb:i
        let s_14_0: i128 = fn_state.baselsb;
        // D s_14_1: call __id(s_14_0)
        let s_14_1: i128 = u__id(state, tracer, s_14_0);
        // C s_14_2: const #0s : i
        let s_14_2: i128 = 0;
        // D s_14_3: cmp-le s_14_2 s_14_1
        let s_14_3: bool = ((s_14_2) <= (s_14_1));
        // N s_14_4: branch s_14_3 b102 b15
        if s_14_3 {
            return block_102(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#28227 <= s_15_0
        fn_state.gs_28227 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_16_0: read-var gs#28227:u8
        let s_16_0: bool = fn_state.gs_28227;
        // N s_16_1: assert s_16_0
        let s_16_1: () = assert!(s_16_0);
        // C s_16_2: const #56s : i
        let s_16_2: i128 = 56;
        // C s_16_3: const #39s : i
        let s_16_3: i128 = 39;
        // D s_16_4: read-var ttbr:u64
        let s_16_4: u64 = fn_state.ttbr;
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 64u16);
        // D s_16_6: read-var baselsb:i
        let s_16_6: i128 = fn_state.baselsb;
        // D s_16_7: read-var baselsb:i
        let s_16_7: i128 = fn_state.baselsb;
        // D s_16_8: call place_subrange(s_16_2, s_16_5, s_16_3, s_16_6, s_16_7)
        let s_16_8: Bits = place_subrange(
            state,
            tracer,
            s_16_2,
            s_16_5,
            s_16_3,
            s_16_6,
            s_16_7,
        );
        // D s_16_9: cast reint s_16_8 -> u56
        let s_16_9: u64 = (s_16_8.value() as u64);
        // D s_16_10: write-var baseaddress.0 <= s_16_9
        fn_state.baseaddress._0 = s_16_9;
        // D s_16_11: read-var baseaddress:struct
        let s_16_11: ProductTypeda0231e9dc169f81 = fn_state.baseaddress;
        // D s_16_12: write-var walkstate.0 <= s_16_11
        fn_state.walkstate._0 = s_16_11;
        // D s_16_13: read-var startlevel:i
        let s_16_13: i128 = fn_state.startlevel;
        // D s_16_14: write-var walkstate.6 <= s_16_13
        fn_state.walkstate._6 = s_16_13;
        // C s_16_15: const #1u : u8
        let s_16_15: bool = true;
        // D s_16_16: write-var walkstate.5 <= s_16_15
        fn_state.walkstate._5 = s_16_15;
        // D s_16_17: read-var regime:u32
        let s_16_17: u32 = fn_state.regime;
        // D s_16_18: call HasUnprivileged(s_16_17)
        let s_16_18: bool = HasUnprivileged(state, tracer, s_16_17);
        // N s_16_19: branch s_16_18 b101 b17
        if s_16_18 {
            return block_101(state, tracer, fn_state);
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
        // D s_17_1: write-var walkstate.8 <= s_17_0
        fn_state.walkstate._8 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_18_0: read-var walkparams.29:struct
        let s_18_0: u8 = fn_state.walkparams._29;
        // D s_18_1: read-var walkparams.16:struct
        let s_18_1: u8 = fn_state.walkparams._16;
        // D s_18_2: read-var walkparams.23:struct
        let s_18_2: u8 = fn_state.walkparams._23;
        // D s_18_3: call WalkMemAttrs(s_18_0, s_18_1, s_18_2)
        let s_18_3: ProductTypef170cab34335b70c = WalkMemAttrs(
            state,
            tracer,
            s_18_0,
            s_18_1,
            s_18_2,
        );
        // D s_18_4: write-var walkstate.7 <= s_18_3
        fn_state.walkstate._7 = s_18_3;
        // C s_18_5: const #0u : u8
        let s_18_5: u8 = 0;
        // D s_18_6: write-var walkstate.9.1 <= s_18_5
        fn_state.walkstate._9._1 = s_18_5;
        // C s_18_7: const #0u : u8
        let s_18_7: bool = false;
        // D s_18_8: write-var walkstate.9.18 <= s_18_7
        fn_state.walkstate._9._18 = s_18_7;
        // C s_18_9: const #0u : u8
        let s_18_9: bool = false;
        // D s_18_10: write-var walkstate.9.6 <= s_18_9
        fn_state.walkstate._9._6 = s_18_9;
        // C s_18_11: const #1s : i
        let s_18_11: i128 = 1;
        // D s_18_12: read-var iasize:i
        let s_18_12: i128 = fn_state.iasize;
        // D s_18_13: sub s_18_12 s_18_11
        let s_18_13: i128 = ((s_18_12) - (s_18_11));
        // D s_18_14: write-var indexmsb <= s_18_13
        fn_state.indexmsb = s_18_13;
        // C s_18_15: const #64s : i
        let s_18_15: i128 = 64;
        // D s_18_16: read-var va:u32
        let s_18_16: u32 = fn_state.va;
        // D s_18_17: cast zx s_18_16 -> bv
        let s_18_17: Bits = Bits::new(s_18_16 as u128, 32u16);
        // D s_18_18: bits-cast zx s_18_17 -> bv length s_18_15
        let s_18_18: Bits = s_18_17.zero_extend(s_18_15);
        // D s_18_19: cast reint s_18_18 -> u64
        let s_18_19: u64 = (s_18_18.value() as u64);
        // D s_18_20: write-var walkaddress.7 <= s_18_19
        fn_state.walkaddress._7 = s_18_19;
        // D s_18_21: read-var regime:u32
        let s_18_21: u32 = fn_state.regime;
        // D s_18_22: call AArch32_S1DCacheEnabled(s_18_21)
        let s_18_22: bool = AArch32_S1DCacheEnabled(state, tracer, s_18_21);
        // D s_18_23: not s_18_22
        let s_18_23: bool = !s_18_22;
        // N s_18_24: branch s_18_23 b100 b19
        if s_18_23 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_19_0: read-var walkstate.7:struct
        let s_19_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_19_1: write-var walkaddress.2 <= s_19_0
        fn_state.walkaddress._2 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_20_0: read-var regime:u32
        let s_20_0: u32 = fn_state.regime;
        // C s_20_1: const #4u : u32
        let s_20_1: u32 = 4;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // N s_20_3: branch s_20_2 b99 b21
        if s_20_2 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#28243 <= s_21_0
        fn_state.gs_28243 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_22_0: read-var gs#28243:u8
        let s_22_0: bool = fn_state.gs_28243;
        // N s_22_1: branch s_22_0 b95 b23
        if s_22_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#28244 <= s_23_0
        fn_state.gs_28244 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_24_0: read-var gs#28244:u8
        let s_24_0: bool = fn_state.gs_28244;
        // N s_24_1: branch s_24_0 b94 b25
        if s_24_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#28245 <= s_25_0
        fn_state.gs_28245 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_26_0: read-var gs#28245:u8
        let s_26_0: bool = fn_state.gs_28245;
        // N s_26_1: branch s_26_0 b93 b27
        if s_26_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_27_0: read-var walkaddress.2:struct
        let s_27_0: ProductTypef170cab34335b70c = fn_state.walkaddress._2;
        // D s_27_1: call EffectiveShareability(s_27_0)
        let s_27_1: u32 = EffectiveShareability(state, tracer, s_27_0);
        // D s_27_2: write-var walkaddress.2.5 <= s_27_1
        fn_state.walkaddress._2._5 = s_27_1;
        // N s_27_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_29_0: read-var walkstate.6:struct
        let s_29_0: i128 = fn_state.walkstate._6;
        // D s_29_1: write-var fault.9 <= s_29_0
        fn_state.fault._9 = s_29_0;
        // D s_29_2: read-var walkstate.6:struct
        let s_29_2: i128 = fn_state.walkstate._6;
        // C s_29_3: const #800u : u32
        let s_29_3: u32 = 800;
        // D s_29_4: read-reg s_29_3:i64
        let s_29_4: i64 = {
            let value = state.read_register::<i64>(s_29_3 as isize);
            tracer.read_register(s_29_3 as isize, value);
            value
        };
        // D s_29_5: cast zx s_29_4 -> i
        let s_29_5: i128 = (i128::try_from(s_29_4).unwrap());
        // D s_29_6: sub s_29_5 s_29_2
        let s_29_6: i128 = ((s_29_5) - (s_29_2));
        // D s_29_7: read-var stride:i
        let s_29_7: i128 = fn_state.stride;
        // D s_29_8: mul s_29_6 s_29_7
        let s_29_8: i128 = ((s_29_6) * (s_29_7));
        // D s_29_9: read-var granulebits:i
        let s_29_9: i128 = fn_state.granulebits;
        // D s_29_10: add s_29_8 s_29_9
        let s_29_10: i128 = (s_29_8 + s_29_9);
        // D s_29_11: write-var indexlsb <= s_29_10
        fn_state.indexlsb = s_29_10;
        // D s_29_12: read-var indexmsb:i
        let s_29_12: i128 = fn_state.indexmsb;
        // D s_29_13: write-var indexmsbshadow#522 <= s_29_12
        fn_state.indexmsbshadow_522 = s_29_12;
        // D s_29_14: read-var indexlsb:i
        let s_29_14: i128 = fn_state.indexlsb;
        // D s_29_15: write-var indexlsbshadow#523 <= s_29_14
        fn_state.indexlsbshadow_523 = s_29_14;
        // D s_29_16: read-var indexlsbshadow#523:i
        let s_29_16: i128 = fn_state.indexlsbshadow_523;
        // D s_29_17: call __id(s_29_16)
        let s_29_17: i128 = u__id(state, tracer, s_29_16);
        // C s_29_18: const #0s : i
        let s_29_18: i128 = 0;
        // D s_29_19: cmp-le s_29_18 s_29_17
        let s_29_19: bool = ((s_29_18) <= (s_29_17));
        // N s_29_20: branch s_29_19 b89 b30
        if s_29_19 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#28257 <= s_30_0
        fn_state.gs_28257 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_31_0: read-var gs#28257:u8
        let s_31_0: bool = fn_state.gs_28257;
        // N s_31_1: assert s_31_0
        let s_31_1: () = assert!(s_31_0);
        // C s_31_2: const #40s : i
        let s_31_2: i128 = 40;
        // C s_31_3: const #3s : i
        let s_31_3: i128 = 3;
        // D s_31_4: read-var va:u32
        let s_31_4: u32 = fn_state.va;
        // D s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 32u16);
        // D s_31_6: read-var indexmsbshadow#522:i
        let s_31_6: i128 = fn_state.indexmsbshadow_522;
        // D s_31_7: read-var indexlsbshadow#523:i
        let s_31_7: i128 = fn_state.indexlsbshadow_523;
        // D s_31_8: call place_subrange(s_31_2, s_31_5, s_31_6, s_31_7, s_31_3)
        let s_31_8: Bits = place_subrange(
            state,
            tracer,
            s_31_2,
            s_31_5,
            s_31_6,
            s_31_7,
            s_31_3,
        );
        // D s_31_9: cast reint s_31_8 -> u40
        let s_31_9: u64 = (s_31_8.value() as u64);
        // D s_31_10: read-var walkstate.0:struct
        let s_31_10: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_31_11: write-var ga#21864 <= s_31_10
        fn_state.ga_21864 = s_31_10;
        // D s_31_12: read-var ga#21864.0:struct
        let s_31_12: u64 = fn_state.ga_21864._0;
        // C s_31_13: const #56s : i
        let s_31_13: i128 = 56;
        // D s_31_14: cast zx s_31_9 -> bv
        let s_31_14: Bits = Bits::new(s_31_9 as u128, 40u16);
        // D s_31_15: bits-cast zx s_31_14 -> bv length s_31_13
        let s_31_15: Bits = s_31_14.zero_extend(s_31_13);
        // D s_31_16: cast reint s_31_15 -> u56
        let s_31_16: u64 = (s_31_15.value() as u64);
        // D s_31_17: cast zx s_31_12 -> bv
        let s_31_17: Bits = Bits::new(s_31_12 as u128, 56u16);
        // D s_31_18: cast zx s_31_16 -> bv
        let s_31_18: Bits = Bits::new(s_31_16 as u128, 56u16);
        // D s_31_19: or s_31_17 s_31_18
        let s_31_19: Bits = ((s_31_17) | (s_31_18));
        // D s_31_20: cast reint s_31_19 -> u56
        let s_31_20: u64 = (s_31_19.value() as u64);
        // D s_31_21: write-var walkaddress.3.0 <= s_31_20
        fn_state.walkaddress._3._0 = s_31_20;
        // D s_31_22: read-var walkstate.0:struct
        let s_31_22: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_31_23: write-var ga#21867 <= s_31_22
        fn_state.ga_21867 = s_31_22;
        // D s_31_24: read-var ga#21867.1:struct
        let s_31_24: u32 = fn_state.ga_21867._1;
        // D s_31_25: write-var walkaddress.3.1 <= s_31_24
        fn_state.walkaddress._3._1 = s_31_24;
        // D s_31_26: read-var walkstate.6:struct
        let s_31_26: i128 = fn_state.walkstate._6;
        // D s_31_27: read-var startlevel:i
        let s_31_27: i128 = fn_state.startlevel;
        // D s_31_28: cmp-eq s_31_26 s_31_27
        let s_31_28: bool = ((s_31_26) == (s_31_27));
        // D s_31_29: read-var varange:u32
        let s_31_29: u32 = fn_state.varange;
        // D s_31_30: read-var accdesc:struct
        let s_31_30: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_31_31: call CreateAccDescS1TTW(s_31_28, s_31_29, s_31_30)
        let s_31_31: ProductType9878976b5bcce9c9 = CreateAccDescS1TTW(
            state,
            tracer,
            s_31_28,
            s_31_29,
            s_31_30,
        );
        // D s_31_32: write-var walkaccess <= s_31_31
        fn_state.walkaccess = s_31_31;
        // C s_31_33: const #() : ()
        let s_31_33: () = ();
        // D s_31_34: create-sum enum = 0:"s_31_33"
        let s_31_34: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_31_33);
        // C s_31_35: const #() : ()
        let s_31_35: () = ();
        // D s_31_36: create-sum enum = 0:"s_31_35"
        let s_31_36: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_31_35);
        // D s_31_37: read-var walkaddress.7:struct
        let s_31_37: u64 = fn_state.walkaddress._7;
        // D s_31_38: read-var walkstate.6:struct
        let s_31_38: i128 = fn_state.walkstate._6;
        // D s_31_39: create-sum enum = 1:"s_31_38"
        let s_31_39: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_1(s_31_38);
        // C s_31_40: const #() : ()
        let s_31_40: () = ();
        // D s_31_41: create-sum enum = 0:"s_31_40"
        let s_31_41: SumType3cca557f9e907281 = SumType3cca557f9e907281::_0(s_31_40);
        // D s_31_42: read-var walkparams:struct
        let s_31_42: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_31_43: create-sum enum = 1:"s_31_42"
        let s_31_43: SumTypefc0aa8a49e605a17 = SumTypefc0aa8a49e605a17::_1(s_31_42);
        // C s_31_44: const #() : ()
        let s_31_44: () = ();
        // D s_31_45: create-sum enum = 0:"s_31_44"
        let s_31_45: SumType3436044442b382d9 = SumType3436044442b382d9::_0(s_31_44);
        // D s_31_46: read-var walkaddress.2:struct
        let s_31_46: ProductTypef170cab34335b70c = fn_state.walkaddress._2;
        // D s_31_47: read-var regime:u32
        let s_31_47: u32 = fn_state.regime;
        // D s_31_48: create-product struct = ["s_31_36", "s_31_46", "s_31_47", "s_31_39", "s_31_43", "s_31_41", "s_31_45", "s_31_37", "s_31_34"]
        let s_31_48: ProductTypeb525737120e184b3 = ProductTypeb525737120e184b3 {
            _0: s_31_36,
            _1: s_31_46,
            _2: s_31_47,
            _3: s_31_39,
            _4: s_31_43,
            _5: s_31_41,
            _6: s_31_45,
            _7: s_31_37,
            _8: s_31_34,
        };
        // D s_31_49: write-var translation_info <= s_31_48
        fn_state.translation_info = s_31_48;
        // D s_31_50: read-var regime:u32
        let s_31_50: u32 = fn_state.regime;
        // C s_31_51: const #4u : u32
        let s_31_51: u32 = 4;
        // D s_31_52: cmp-eq s_31_50 s_31_51
        let s_31_52: bool = ((s_31_50) == (s_31_51));
        // N s_31_53: branch s_31_52 b88 b32
        if s_31_52 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#28266 <= s_32_0
        fn_state.gs_28266 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_33_0: read-var gs#28266:u8
        let s_33_0: bool = fn_state.gs_28266;
        // N s_33_1: branch s_33_0 b85 b34
        if s_33_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_34_0: read-var walkparams.9:struct
        let s_34_0: bool = fn_state.walkparams._9;
        // C s_34_1: const #64s : i64
        let s_34_1: i64 = 64;
        // D s_34_2: read-var walkaddress:struct
        let s_34_2: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // D s_34_3: read-var walkaccess:struct
        let s_34_3: ProductType9878976b5bcce9c9 = fn_state.walkaccess;
        // D s_34_4: read-var fault:struct
        let s_34_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_34_5: read-var translation_info:struct
        let s_34_5: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_34_6: call FetchDescriptor(s_34_0, s_34_2, s_34_3, s_34_4, s_34_1, s_34_5)
        let s_34_6: ProductTypeb4cea7287e2eb9d6 = FetchDescriptor(
            state,
            tracer,
            s_34_0,
            s_34_2,
            s_34_3,
            s_34_4,
            s_34_1,
            s_34_5,
        );
        // D s_34_7: write-var gs#453683 <= s_34_6
        fn_state.gs_453683 = s_34_6;
        // D s_34_8: read-var gs#453683.0:struct
        let s_34_8: ProductType1d757adad216cdef = fn_state.gs_453683._0;
        // D s_34_9: read-var gs#453683.1:struct
        let s_34_9: Bits = fn_state.gs_453683._1;
        // D s_34_10: cast reint s_34_9 -> u64
        let s_34_10: u64 = (s_34_9.value() as u64);
        // D s_34_11: write-var fault <= s_34_8
        fn_state.fault = s_34_8;
        // D s_34_12: write-var descriptor <= s_34_10
        fn_state.descriptor = s_34_10;
        // N s_34_13: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_35_0: read-var fault.16:struct
        let s_35_0: u32 = fn_state.fault._16;
        // C s_35_1: const #0u : u32
        let s_35_1: u32 = 0;
        // D s_35_2: cmp-eq s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) == (s_35_1));
        // N s_35_3: branch s_35_2 b84 b36
        if s_35_2 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_36_0: read-var walkstate.6:struct
        let s_36_0: i128 = fn_state.walkstate._6;
        // D s_36_1: read-var descriptor:u64
        let s_36_1: u64 = fn_state.descriptor;
        // D s_36_2: call AArch32_DecodeDescriptorTypeLD(s_36_1, s_36_0)
        let s_36_2: u32 = AArch32_DecodeDescriptorTypeLD(state, tracer, s_36_1, s_36_0);
        // D s_36_3: write-var desctype <= s_36_2
        fn_state.desctype = s_36_2;
        // C s_36_4: const #0u : u32
        let s_36_4: u32 = 0;
        // D s_36_5: read-var desctype:u32
        let s_36_5: u32 = fn_state.desctype;
        // D s_36_6: cmp-eq s_36_4 s_36_5
        let s_36_6: bool = ((s_36_4) == (s_36_5));
        // D s_36_7: not s_36_6
        let s_36_7: bool = !s_36_6;
        // N s_36_8: branch s_36_7 b79 b37
        if s_36_7 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_37_0: const #40s : i
        let s_37_0: i128 = 40;
        // D s_37_1: read-var descriptor:u64
        let s_37_1: u64 = fn_state.descriptor;
        // D s_37_2: cast zx s_37_1 -> bv
        let s_37_2: Bits = Bits::new(s_37_1 as u128, 64u16);
        // C s_37_3: const #1s : i64
        let s_37_3: i64 = 1;
        // C s_37_4: cast zx s_37_3 -> i
        let s_37_4: i128 = (i128::try_from(s_37_3).unwrap());
        // C s_37_5: const #7s : i
        let s_37_5: i128 = 7;
        // C s_37_6: add s_37_5 s_37_4
        let s_37_6: i128 = (s_37_5 + s_37_4);
        // D s_37_7: bit-extract s_37_2 s_37_0 s_37_6
        let s_37_7: Bits = (Bits::new(
            ((s_37_2) >> (s_37_0)).value(),
            u16::try_from(s_37_6).unwrap(),
        ));
        // D s_37_8: cast reint s_37_7 -> u8
        let s_37_8: u8 = (s_37_7.value() as u8);
        // D s_37_9: cast zx s_37_8 -> bv
        let s_37_9: Bits = Bits::new(s_37_8 as u128, 8u16);
        // D s_37_10: call IsZero(s_37_9)
        let s_37_10: bool = IsZero(state, tracer, s_37_9);
        // D s_37_11: not s_37_10
        let s_37_11: bool = !s_37_10;
        // N s_37_12: branch s_37_11 b78 b38
        if s_37_11 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_38_0: const #56s : i
        let s_38_0: i128 = 56;
        // C s_38_1: const #39s : i
        let s_38_1: i128 = 39;
        // C s_38_2: const #12s : i
        let s_38_2: i128 = 12;
        // C s_38_3: const #12s : i
        let s_38_3: i128 = 12;
        // D s_38_4: read-var descriptor:u64
        let s_38_4: u64 = fn_state.descriptor;
        // D s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 64u16);
        // D s_38_6: call place_subrange(s_38_0, s_38_5, s_38_1, s_38_2, s_38_3)
        let s_38_6: Bits = place_subrange(
            state,
            tracer,
            s_38_0,
            s_38_5,
            s_38_1,
            s_38_2,
            s_38_3,
        );
        // D s_38_7: cast reint s_38_6 -> u56
        let s_38_7: u64 = (s_38_6.value() as u64);
        // D s_38_8: write-var walkstate.0.0 <= s_38_7
        fn_state.walkstate._0._0 = s_38_7;
        // D s_38_9: read-var walkstate.0:struct
        let s_38_9: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_38_10: write-var ga#21894 <= s_38_9
        fn_state.ga_21894 = s_38_9;
        // D s_38_11: read-var ga#21894.1:struct
        let s_38_11: u32 = fn_state.ga_21894._1;
        // C s_38_12: const #1u : u32
        let s_38_12: u32 = 1;
        // D s_38_13: cmp-eq s_38_11 s_38_12
        let s_38_13: bool = ((s_38_11) == (s_38_12));
        // N s_38_14: branch s_38_13 b77 b39
        if s_38_13 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#28295 <= s_39_0
        fn_state.gs_28295 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_40_0: read-var gs#28295:u8
        let s_40_0: bool = fn_state.gs_28295;
        // N s_40_1: branch s_40_0 b76 b41
        if s_40_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_41_0: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_42_0: read-var walkparams.15:struct
        let s_42_0: bool = fn_state.walkparams._15;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // C s_42_2: const #0u : u8
        let s_42_2: bool = false;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // N s_42_5: branch s_42_4 b75 b43
        if s_42_4 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_43_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_44_0: read-var walkstate.6:struct
        let s_44_0: i128 = fn_state.walkstate._6;
        // C s_44_1: const #1s : i
        let s_44_1: i128 = 1;
        // D s_44_2: add s_44_0 s_44_1
        let s_44_2: i128 = (s_44_0 + s_44_1);
        // D s_44_3: write-var walkstate.6 <= s_44_2
        fn_state.walkstate._6 = s_44_2;
        // C s_44_4: const #1s : i
        let s_44_4: i128 = 1;
        // D s_44_5: read-var indexlsb:i
        let s_44_5: i128 = fn_state.indexlsb;
        // D s_44_6: sub s_44_5 s_44_4
        let s_44_6: i128 = ((s_44_5) - (s_44_4));
        // D s_44_7: write-var indexmsb <= s_44_6
        fn_state.indexmsb = s_44_6;
        // N s_44_8: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_45_0: read-var desctype:u32
        let s_45_0: u32 = fn_state.desctype;
        // C s_45_1: const #1u : u32
        let s_45_1: u32 = 1;
        // D s_45_2: cmp-eq s_45_0 s_45_1
        let s_45_2: bool = ((s_45_0) == (s_45_1));
        // N s_45_3: branch s_45_2 b46 b29
        if s_45_2 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_46_0: read-var indexlsb:i
        let s_46_0: i128 = fn_state.indexlsb;
        // D s_46_1: write-var indexlsbshadow#521 <= s_46_0
        fn_state.indexlsbshadow_521 = s_46_0;
        // C s_46_2: const #40s : i
        let s_46_2: i128 = 40;
        // D s_46_3: read-var descriptor:u64
        let s_46_3: u64 = fn_state.descriptor;
        // D s_46_4: cast zx s_46_3 -> bv
        let s_46_4: Bits = Bits::new(s_46_3 as u128, 64u16);
        // C s_46_5: const #1s : i64
        let s_46_5: i64 = 1;
        // C s_46_6: cast zx s_46_5 -> i
        let s_46_6: i128 = (i128::try_from(s_46_5).unwrap());
        // C s_46_7: const #7s : i
        let s_46_7: i128 = 7;
        // C s_46_8: add s_46_7 s_46_6
        let s_46_8: i128 = (s_46_7 + s_46_6);
        // D s_46_9: bit-extract s_46_4 s_46_2 s_46_8
        let s_46_9: Bits = (Bits::new(
            ((s_46_4) >> (s_46_2)).value(),
            u16::try_from(s_46_8).unwrap(),
        ));
        // D s_46_10: cast reint s_46_9 -> u8
        let s_46_10: u8 = (s_46_9.value() as u8);
        // D s_46_11: cast zx s_46_10 -> bv
        let s_46_11: Bits = Bits::new(s_46_10 as u128, 8u16);
        // D s_46_12: call IsZero(s_46_11)
        let s_46_12: bool = IsZero(state, tracer, s_46_11);
        // D s_46_13: not s_46_12
        let s_46_13: bool = !s_46_12;
        // N s_46_14: branch s_46_13 b74 b47
        if s_46_13 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_47_0: const #10s : i
        let s_47_0: i128 = 10;
        // D s_47_1: read-var descriptor:u64
        let s_47_1: u64 = fn_state.descriptor;
        // D s_47_2: cast zx s_47_1 -> bv
        let s_47_2: Bits = Bits::new(s_47_1 as u128, 64u16);
        // C s_47_3: const #1u : u64
        let s_47_3: u64 = 1;
        // D s_47_4: bit-extract s_47_2 s_47_0 s_47_3
        let s_47_4: Bits = (Bits::new(
            ((s_47_2) >> (s_47_0)).value(),
            u16::try_from(s_47_3).unwrap(),
        ));
        // D s_47_5: cast reint s_47_4 -> u8
        let s_47_5: bool = ((s_47_4.value()) != 0);
        // C s_47_6: const #0s : i
        let s_47_6: i128 = 0;
        // C s_47_7: const #0u : u64
        let s_47_7: u64 = 0;
        // D s_47_8: cast zx s_47_5 -> u64
        let s_47_8: u64 = (s_47_5 as u64);
        // C s_47_9: const #1u : u64
        let s_47_9: u64 = 1;
        // D s_47_10: and s_47_8 s_47_9
        let s_47_10: u64 = ((s_47_8) & (s_47_9));
        // D s_47_11: cmp-eq s_47_10 s_47_9
        let s_47_11: bool = ((s_47_10) == (s_47_9));
        // D s_47_12: lsl s_47_8 s_47_6
        let s_47_12: u64 = s_47_8 << s_47_6;
        // D s_47_13: or s_47_7 s_47_12
        let s_47_13: u64 = ((s_47_7) | (s_47_12));
        // D s_47_14: cmpl s_47_12
        let s_47_14: u64 = !s_47_12;
        // D s_47_15: and s_47_7 s_47_14
        let s_47_15: u64 = ((s_47_7) & (s_47_14));
        // D s_47_16: select s_47_11 s_47_13 s_47_15
        let s_47_16: u64 = if s_47_11 { s_47_13 } else { s_47_15 };
        // D s_47_17: cast trunc s_47_16 -> u8
        let s_47_17: bool = ((s_47_16) != 0);
        // D s_47_18: cast zx s_47_17 -> bv
        let s_47_18: Bits = Bits::new(s_47_17 as u128, 1u16);
        // C s_47_19: const #0u : u8
        let s_47_19: bool = false;
        // C s_47_20: cast zx s_47_19 -> bv
        let s_47_20: Bits = Bits::new(s_47_19 as u128, 1u16);
        // D s_47_21: cmp-eq s_47_18 s_47_20
        let s_47_21: bool = ((s_47_18) == (s_47_20));
        // N s_47_22: branch s_47_21 b73 b48
        if s_47_21 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_48_0: const #54s : i
        let s_48_0: i128 = 54;
        // D s_48_1: read-var descriptor:u64
        let s_48_1: u64 = fn_state.descriptor;
        // D s_48_2: cast zx s_48_1 -> bv
        let s_48_2: Bits = Bits::new(s_48_1 as u128, 64u16);
        // C s_48_3: const #1u : u64
        let s_48_3: u64 = 1;
        // D s_48_4: bit-extract s_48_2 s_48_0 s_48_3
        let s_48_4: Bits = (Bits::new(
            ((s_48_2) >> (s_48_0)).value(),
            u16::try_from(s_48_3).unwrap(),
        ));
        // D s_48_5: cast reint s_48_4 -> u8
        let s_48_5: bool = ((s_48_4.value()) != 0);
        // C s_48_6: const #0s : i
        let s_48_6: i128 = 0;
        // C s_48_7: const #0u : u64
        let s_48_7: u64 = 0;
        // D s_48_8: cast zx s_48_5 -> u64
        let s_48_8: u64 = (s_48_5 as u64);
        // C s_48_9: const #1u : u64
        let s_48_9: u64 = 1;
        // D s_48_10: and s_48_8 s_48_9
        let s_48_10: u64 = ((s_48_8) & (s_48_9));
        // D s_48_11: cmp-eq s_48_10 s_48_9
        let s_48_11: bool = ((s_48_10) == (s_48_9));
        // D s_48_12: lsl s_48_8 s_48_6
        let s_48_12: u64 = s_48_8 << s_48_6;
        // D s_48_13: or s_48_7 s_48_12
        let s_48_13: u64 = ((s_48_7) | (s_48_12));
        // D s_48_14: cmpl s_48_12
        let s_48_14: u64 = !s_48_12;
        // D s_48_15: and s_48_7 s_48_14
        let s_48_15: u64 = ((s_48_7) & (s_48_14));
        // D s_48_16: select s_48_11 s_48_13 s_48_15
        let s_48_16: u64 = if s_48_11 { s_48_13 } else { s_48_15 };
        // D s_48_17: cast trunc s_48_16 -> u8
        let s_48_17: bool = ((s_48_16) != 0);
        // D s_48_18: write-var walkstate.9.17 <= s_48_17
        fn_state.walkstate._9._17 = s_48_17;
        // C s_48_19: const #53s : i
        let s_48_19: i128 = 53;
        // D s_48_20: read-var descriptor:u64
        let s_48_20: u64 = fn_state.descriptor;
        // D s_48_21: cast zx s_48_20 -> bv
        let s_48_21: Bits = Bits::new(s_48_20 as u128, 64u16);
        // C s_48_22: const #1u : u64
        let s_48_22: u64 = 1;
        // D s_48_23: bit-extract s_48_21 s_48_19 s_48_22
        let s_48_23: Bits = (Bits::new(
            ((s_48_21) >> (s_48_19)).value(),
            u16::try_from(s_48_22).unwrap(),
        ));
        // D s_48_24: cast reint s_48_23 -> u8
        let s_48_24: bool = ((s_48_23.value()) != 0);
        // C s_48_25: const #0s : i
        let s_48_25: i128 = 0;
        // C s_48_26: const #0u : u64
        let s_48_26: u64 = 0;
        // D s_48_27: cast zx s_48_24 -> u64
        let s_48_27: u64 = (s_48_24 as u64);
        // C s_48_28: const #1u : u64
        let s_48_28: u64 = 1;
        // D s_48_29: and s_48_27 s_48_28
        let s_48_29: u64 = ((s_48_27) & (s_48_28));
        // D s_48_30: cmp-eq s_48_29 s_48_28
        let s_48_30: bool = ((s_48_29) == (s_48_28));
        // D s_48_31: lsl s_48_27 s_48_25
        let s_48_31: u64 = s_48_27 << s_48_25;
        // D s_48_32: or s_48_26 s_48_31
        let s_48_32: u64 = ((s_48_26) | (s_48_31));
        // D s_48_33: cmpl s_48_31
        let s_48_33: u64 = !s_48_31;
        // D s_48_34: and s_48_26 s_48_33
        let s_48_34: u64 = ((s_48_26) & (s_48_33));
        // D s_48_35: select s_48_30 s_48_32 s_48_34
        let s_48_35: u64 = if s_48_30 { s_48_32 } else { s_48_34 };
        // D s_48_36: cast trunc s_48_35 -> u8
        let s_48_36: bool = ((s_48_35) != 0);
        // D s_48_37: write-var walkstate.9.5 <= s_48_36
        fn_state.walkstate._9._5 = s_48_36;
        // C s_48_38: const #6s : i
        let s_48_38: i128 = 6;
        // D s_48_39: read-var descriptor:u64
        let s_48_39: u64 = fn_state.descriptor;
        // D s_48_40: cast zx s_48_39 -> bv
        let s_48_40: Bits = Bits::new(s_48_39 as u128, 64u16);
        // C s_48_41: const #1s : i64
        let s_48_41: i64 = 1;
        // C s_48_42: cast zx s_48_41 -> i
        let s_48_42: i128 = (i128::try_from(s_48_41).unwrap());
        // C s_48_43: const #1s : i
        let s_48_43: i128 = 1;
        // C s_48_44: add s_48_43 s_48_42
        let s_48_44: i128 = (s_48_43 + s_48_42);
        // D s_48_45: bit-extract s_48_40 s_48_38 s_48_44
        let s_48_45: Bits = (Bits::new(
            ((s_48_40) >> (s_48_38)).value(),
            u16::try_from(s_48_44).unwrap(),
        ));
        // D s_48_46: cast reint s_48_45 -> u8
        let s_48_46: u8 = (s_48_45.value() as u8);
        // D s_48_47: cast zx s_48_46 -> bv
        let s_48_47: Bits = Bits::new(s_48_46 as u128, 2u16);
        // C s_48_48: const #1u : u8
        let s_48_48: bool = true;
        // C s_48_49: cast zx s_48_48 -> bv
        let s_48_49: Bits = Bits::new(s_48_48 as u128, 1u16);
        // D s_48_50: cast reint s_48_47 -> u128
        let s_48_50: u128 = (s_48_47.value() as u128);
        // D s_48_51: size-of s_48_47
        let s_48_51: u16 = s_48_47.length();
        // C s_48_52: cast reint s_48_49 -> u128
        let s_48_52: u128 = (s_48_49.value() as u128);
        // D s_48_53: size-of s_48_49
        let s_48_53: u16 = s_48_49.length();
        // D s_48_54: lsl s_48_50 s_48_53
        let s_48_54: u128 = s_48_50 << s_48_53;
        // D s_48_55: or s_48_54 s_48_52
        let s_48_55: u128 = ((s_48_54) | (s_48_52));
        // D s_48_56: add s_48_51 s_48_53
        let s_48_56: u16 = (s_48_51 + s_48_53);
        // D s_48_57: create-bits s_48_55 s_48_56
        let s_48_57: Bits = Bits::new(s_48_55, s_48_56);
        // D s_48_58: cast reint s_48_57 -> u8
        let s_48_58: u8 = (s_48_57.value() as u8);
        // D s_48_59: write-var walkstate.9.0 <= s_48_58
        fn_state.walkstate._9._0 = s_48_58;
        // C s_48_60: const #52s : i
        let s_48_60: i128 = 52;
        // D s_48_61: read-var descriptor:u64
        let s_48_61: u64 = fn_state.descriptor;
        // D s_48_62: cast zx s_48_61 -> bv
        let s_48_62: Bits = Bits::new(s_48_61 as u128, 64u16);
        // C s_48_63: const #1u : u64
        let s_48_63: u64 = 1;
        // D s_48_64: bit-extract s_48_62 s_48_60 s_48_63
        let s_48_64: Bits = (Bits::new(
            ((s_48_62) >> (s_48_60)).value(),
            u16::try_from(s_48_63).unwrap(),
        ));
        // D s_48_65: cast reint s_48_64 -> u8
        let s_48_65: bool = ((s_48_64.value()) != 0);
        // C s_48_66: const #0s : i
        let s_48_66: i128 = 0;
        // C s_48_67: const #0u : u64
        let s_48_67: u64 = 0;
        // D s_48_68: cast zx s_48_65 -> u64
        let s_48_68: u64 = (s_48_65 as u64);
        // C s_48_69: const #1u : u64
        let s_48_69: u64 = 1;
        // D s_48_70: and s_48_68 s_48_69
        let s_48_70: u64 = ((s_48_68) & (s_48_69));
        // D s_48_71: cmp-eq s_48_70 s_48_69
        let s_48_71: bool = ((s_48_70) == (s_48_69));
        // D s_48_72: lsl s_48_68 s_48_66
        let s_48_72: u64 = s_48_68 << s_48_66;
        // D s_48_73: or s_48_67 s_48_72
        let s_48_73: u64 = ((s_48_67) | (s_48_72));
        // D s_48_74: cmpl s_48_72
        let s_48_74: u64 = !s_48_72;
        // D s_48_75: and s_48_67 s_48_74
        let s_48_75: u64 = ((s_48_67) & (s_48_74));
        // D s_48_76: select s_48_71 s_48_73 s_48_75
        let s_48_76: u64 = if s_48_71 { s_48_73 } else { s_48_75 };
        // D s_48_77: cast trunc s_48_76 -> u8
        let s_48_77: bool = ((s_48_76) != 0);
        // D s_48_78: write-var walkstate.1 <= s_48_77
        fn_state.walkstate._1 = s_48_77;
        // D s_48_79: read-var regime:u32
        let s_48_79: u32 = fn_state.regime;
        // C s_48_80: const #2u : u32
        let s_48_80: u32 = 2;
        // D s_48_81: cmp-eq s_48_79 s_48_80
        let s_48_81: bool = ((s_48_79) == (s_48_80));
        // N s_48_82: branch s_48_81 b72 b49
        if s_48_81 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_49_0: read-var accdesc.25:struct
        let s_49_0: u32 = fn_state.accdesc._25;
        // C s_49_1: const #3u : u32
        let s_49_1: u32 = 3;
        // D s_49_2: cmp-eq s_49_0 s_49_1
        let s_49_2: bool = ((s_49_0) == (s_49_1));
        // N s_49_3: branch s_49_2 b71 b50
        if s_49_2 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#28346 <= s_50_0
        fn_state.gs_28346 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_51_0: read-var gs#28346:u8
        let s_51_0: bool = fn_state.gs_28346;
        // N s_51_1: branch s_51_0 b70 b52
        if s_51_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_52_0: const #11s : i
        let s_52_0: i128 = 11;
        // D s_52_1: read-var descriptor:u64
        let s_52_1: u64 = fn_state.descriptor;
        // D s_52_2: cast zx s_52_1 -> bv
        let s_52_2: Bits = Bits::new(s_52_1 as u128, 64u16);
        // C s_52_3: const #1u : u64
        let s_52_3: u64 = 1;
        // D s_52_4: bit-extract s_52_2 s_52_0 s_52_3
        let s_52_4: Bits = (Bits::new(
            ((s_52_2) >> (s_52_0)).value(),
            u16::try_from(s_52_3).unwrap(),
        ));
        // D s_52_5: cast reint s_52_4 -> u8
        let s_52_5: bool = ((s_52_4.value()) != 0);
        // C s_52_6: const #0s : i
        let s_52_6: i128 = 0;
        // C s_52_7: const #0u : u64
        let s_52_7: u64 = 0;
        // D s_52_8: cast zx s_52_5 -> u64
        let s_52_8: u64 = (s_52_5 as u64);
        // C s_52_9: const #1u : u64
        let s_52_9: u64 = 1;
        // D s_52_10: and s_52_8 s_52_9
        let s_52_10: u64 = ((s_52_8) & (s_52_9));
        // D s_52_11: cmp-eq s_52_10 s_52_9
        let s_52_11: bool = ((s_52_10) == (s_52_9));
        // D s_52_12: lsl s_52_8 s_52_6
        let s_52_12: u64 = s_52_8 << s_52_6;
        // D s_52_13: or s_52_7 s_52_12
        let s_52_13: u64 = ((s_52_7) | (s_52_12));
        // D s_52_14: cmpl s_52_12
        let s_52_14: u64 = !s_52_12;
        // D s_52_15: and s_52_7 s_52_14
        let s_52_15: u64 = ((s_52_7) & (s_52_14));
        // D s_52_16: select s_52_11 s_52_13 s_52_15
        let s_52_16: u64 = if s_52_11 { s_52_13 } else { s_52_15 };
        // D s_52_17: cast trunc s_52_16 -> u8
        let s_52_17: bool = ((s_52_16) != 0);
        // D s_52_18: write-var walkstate.8 <= s_52_17
        fn_state.walkstate._8 = s_52_17;
        // N s_52_19: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_53_0: read-var indexlsbshadow#521:i
        let s_53_0: i128 = fn_state.indexlsbshadow_521;
        // D s_53_1: call __id(s_53_0)
        let s_53_1: i128 = u__id(state, tracer, s_53_0);
        // C s_53_2: const #0s : i
        let s_53_2: i128 = 0;
        // D s_53_3: cmp-le s_53_2 s_53_1
        let s_53_3: bool = ((s_53_2) <= (s_53_1));
        // N s_53_4: branch s_53_3 b69 b54
        if s_53_3 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#28336 <= s_54_0
        fn_state.gs_28336 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_55_0: read-var gs#28336:u8
        let s_55_0: bool = fn_state.gs_28336;
        // N s_55_1: assert s_55_0
        let s_55_1: () = assert!(s_55_0);
        // C s_55_2: const #56s : i
        let s_55_2: i128 = 56;
        // C s_55_3: const #39s : i
        let s_55_3: i128 = 39;
        // D s_55_4: read-var descriptor:u64
        let s_55_4: u64 = fn_state.descriptor;
        // D s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 64u16);
        // D s_55_6: read-var indexlsbshadow#521:i
        let s_55_6: i128 = fn_state.indexlsbshadow_521;
        // D s_55_7: read-var indexlsbshadow#521:i
        let s_55_7: i128 = fn_state.indexlsbshadow_521;
        // D s_55_8: call place_subrange(s_55_2, s_55_5, s_55_3, s_55_6, s_55_7)
        let s_55_8: Bits = place_subrange(
            state,
            tracer,
            s_55_2,
            s_55_5,
            s_55_3,
            s_55_6,
            s_55_7,
        );
        // D s_55_9: cast reint s_55_8 -> u56
        let s_55_9: u64 = (s_55_8.value() as u64);
        // D s_55_10: write-var walkstate.0.0 <= s_55_9
        fn_state.walkstate._0._0 = s_55_9;
        // D s_55_11: read-var walkstate.0:struct
        let s_55_11: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_55_12: write-var ga#21958 <= s_55_11
        fn_state.ga_21958 = s_55_11;
        // D s_55_13: read-var ga#21958.1:struct
        let s_55_13: u32 = fn_state.ga_21958._1;
        // C s_55_14: const #1u : u32
        let s_55_14: u32 = 1;
        // D s_55_15: cmp-eq s_55_13 s_55_14
        let s_55_15: bool = ((s_55_13) == (s_55_14));
        // N s_55_16: branch s_55_15 b68 b56
        if s_55_15 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#28341 <= s_56_0
        fn_state.gs_28341 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_57_0: read-var gs#28341:u8
        let s_57_0: bool = fn_state.gs_28341;
        // N s_57_1: branch s_57_0 b67 b58
        if s_57_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_58_0: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_59_0: const #2s : i
        let s_59_0: i128 = 2;
        // D s_59_1: read-var descriptor:u64
        let s_59_1: u64 = fn_state.descriptor;
        // D s_59_2: cast zx s_59_1 -> bv
        let s_59_2: Bits = Bits::new(s_59_1 as u128, 64u16);
        // C s_59_3: const #1s : i64
        let s_59_3: i64 = 1;
        // C s_59_4: cast zx s_59_3 -> i
        let s_59_4: i128 = (i128::try_from(s_59_3).unwrap());
        // C s_59_5: const #2s : i
        let s_59_5: i128 = 2;
        // C s_59_6: add s_59_5 s_59_4
        let s_59_6: i128 = (s_59_5 + s_59_4);
        // D s_59_7: bit-extract s_59_2 s_59_0 s_59_6
        let s_59_7: Bits = (Bits::new(
            ((s_59_2) >> (s_59_0)).value(),
            u16::try_from(s_59_6).unwrap(),
        ));
        // D s_59_8: cast reint s_59_7 -> u8
        let s_59_8: u8 = (s_59_7.value() as u8);
        // C s_59_9: const #8s : i
        let s_59_9: i128 = 8;
        // D s_59_10: read-var descriptor:u64
        let s_59_10: u64 = fn_state.descriptor;
        // D s_59_11: cast zx s_59_10 -> bv
        let s_59_11: Bits = Bits::new(s_59_10 as u128, 64u16);
        // C s_59_12: const #1s : i64
        let s_59_12: i64 = 1;
        // C s_59_13: cast zx s_59_12 -> i
        let s_59_13: i128 = (i128::try_from(s_59_12).unwrap());
        // C s_59_14: const #1s : i
        let s_59_14: i128 = 1;
        // C s_59_15: add s_59_14 s_59_13
        let s_59_15: i128 = (s_59_14 + s_59_13);
        // D s_59_16: bit-extract s_59_11 s_59_9 s_59_15
        let s_59_16: Bits = (Bits::new(
            ((s_59_11) >> (s_59_9)).value(),
            u16::try_from(s_59_15).unwrap(),
        ));
        // D s_59_17: cast reint s_59_16 -> u8
        let s_59_17: u8 = (s_59_16.value() as u8);
        // D s_59_18: cast zx s_59_8 -> bv
        let s_59_18: Bits = Bits::new(s_59_8 as u128, 3u16);
        // D s_59_19: cast zx s_59_18 -> i
        let s_59_19: i128 = (s_59_18.value() as i128);
        // D s_59_20: cast reint s_59_19 -> i64
        let s_59_20: i64 = (s_59_19 as i64);
        // D s_59_21: read-var walkparams.17:struct
        let s_59_21: ProductType5c790c8ef59cc8b2 = fn_state.walkparams._17;
        // D s_59_22: cast zx s_59_20 -> i
        let s_59_22: i128 = (i128::try_from(s_59_20).unwrap());
        // D s_59_23: call AArch32_MAIRAttr(s_59_22, s_59_21)
        let s_59_23: u8 = AArch32_MAIRAttr(state, tracer, s_59_22, s_59_21);
        // C s_59_24: const #0u : u8
        let s_59_24: bool = false;
        // D s_59_25: read-var walkparams:struct
        let s_59_25: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_59_26: call S1DecodeMemAttrs(s_59_23, s_59_17, s_59_24, s_59_25)
        let s_59_26: ProductTypef170cab34335b70c = S1DecodeMemAttrs(
            state,
            tracer,
            s_59_23,
            s_59_17,
            s_59_24,
            s_59_25,
        );
        // D s_59_27: write-var walkstate.7 <= s_59_26
        fn_state.walkstate._7 = s_59_26;
        // C s_59_28: const #19088u : u32
        let s_59_28: u32 = 19088;
        // D s_59_29: read-reg s_59_28:u8
        let s_59_29: bool = {
            let value = state.read_register::<bool>(s_59_28 as isize);
            tracer.read_register(s_59_28 as isize, value);
            value
        };
        // N s_59_30: branch s_59_29 b63 b60
        if s_59_29 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_60_0: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_61_0: read-var fault:struct
        let s_61_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_61_1: read-var walkstate:struct
        let s_61_1: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_61_2: create-product struct = ["s_61_0", "s_61_1"]
        let s_61_2: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_61_0,
            _1: s_61_1,
        };
        // D s_61_3: write-var return_value <= s_61_2
        fn_state.return_value = s_61_2;
        // N s_61_4: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_62_0: read-var return_value:struct
        let s_62_0: ProductType201519a0f62623dc = fn_state.return_value;
        // N s_62_1: return s_62_0
        return s_62_0;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_63_0: read-var walkstate.7:struct
        let s_63_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_63_1: write-var ga#21964 <= s_63_0
        fn_state.ga_21964 = s_63_0;
        // D s_63_2: read-var ga#21964.7:struct
        let s_63_2: bool = fn_state.ga_21964._7;
        // D s_63_3: write-var tlbcontext.14 <= s_63_2
        fn_state.tlbcontext._14 = s_63_2;
        // D s_63_4: read-var walkstate.6:struct
        let s_63_4: i128 = fn_state.walkstate._6;
        // D s_63_5: write-var tlbcontext.8 <= s_63_4
        fn_state.tlbcontext._8 = s_63_4;
        // D s_63_6: read-var walkstate.8:struct
        let s_63_6: bool = fn_state.walkstate._8;
        // D s_63_7: write-var tlbcontext.9 <= s_63_6
        fn_state.tlbcontext._9 = s_63_6;
        // D s_63_8: read-var tlbcontext:struct
        let s_63_8: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_63_9: write-var tlbrecord.1 <= s_63_8
        fn_state.tlbrecord._1 = s_63_8;
        // D s_63_10: read-var walkstate:struct
        let s_63_10: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_63_11: write-var tlbrecord.5 <= s_63_10
        fn_state.tlbrecord._5 = s_63_10;
        // D s_63_12: read-var walkparams.3:struct
        let s_63_12: bool = fn_state.walkparams._3;
        // D s_63_13: read-var walkparams.36:struct
        let s_63_13: u32 = fn_state.walkparams._36;
        // D s_63_14: read-var walkstate.6:struct
        let s_63_14: i128 = fn_state.walkstate._6;
        // D s_63_15: call TranslationSize(s_63_12, s_63_13, s_63_14)
        let s_63_15: i128 = TranslationSize(state, tracer, s_63_12, s_63_13, s_63_14);
        // D s_63_16: write-var tlbrecord.0 <= s_63_15
        fn_state.tlbrecord._0 = s_63_15;
        // D s_63_17: read-var walkstate.1:struct
        let s_63_17: bool = fn_state.walkstate._1;
        // D s_63_18: cast zx s_63_17 -> bv
        let s_63_18: Bits = Bits::new(s_63_17 as u128, 1u16);
        // C s_63_19: const #1u : u8
        let s_63_19: bool = true;
        // C s_63_20: cast zx s_63_19 -> bv
        let s_63_20: Bits = Bits::new(s_63_19 as u128, 1u16);
        // D s_63_21: cmp-eq s_63_18 s_63_20
        let s_63_21: bool = ((s_63_18) == (s_63_20));
        // N s_63_22: branch s_63_21 b66 b64
        if s_63_21 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_64_0: const #0s : i
        let s_64_0: i128 = 0;
        // D s_64_1: write-var tlbrecord.2 <= s_64_0
        fn_state.tlbrecord._2 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_65_0: read-var tlbrecord:struct
        let s_65_0: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_65_1: call S1TLBCache(s_65_0)
        let s_65_1: () = S1TLBCache(state, tracer, s_65_0);
        // N s_65_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_66_0: read-var walkparams.3:struct
        let s_66_0: bool = fn_state.walkparams._3;
        // D s_66_1: read-var walkparams.36:struct
        let s_66_1: u32 = fn_state.walkparams._36;
        // D s_66_2: read-var walkstate.6:struct
        let s_66_2: i128 = fn_state.walkstate._6;
        // D s_66_3: call ContiguousSize(s_66_0, s_66_1, s_66_2)
        let s_66_3: i128 = ContiguousSize(state, tracer, s_66_0, s_66_1, s_66_2);
        // D s_66_4: write-var tlbrecord.2 <= s_66_3
        fn_state.tlbrecord._2 = s_66_3;
        // N s_66_5: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_67_0: const #0u : u32
        let s_67_0: u32 = 0;
        // D s_67_1: write-var walkstate.0.1 <= s_67_0
        fn_state.walkstate._0._1 = s_67_0;
        // N s_67_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_68_0: const #5s : i
        let s_68_0: i128 = 5;
        // D s_68_1: read-var descriptor:u64
        let s_68_1: u64 = fn_state.descriptor;
        // D s_68_2: cast zx s_68_1 -> bv
        let s_68_2: Bits = Bits::new(s_68_1 as u128, 64u16);
        // C s_68_3: const #1u : u64
        let s_68_3: u64 = 1;
        // D s_68_4: bit-extract s_68_2 s_68_0 s_68_3
        let s_68_4: Bits = (Bits::new(
            ((s_68_2) >> (s_68_0)).value(),
            u16::try_from(s_68_3).unwrap(),
        ));
        // D s_68_5: cast reint s_68_4 -> u8
        let s_68_5: bool = ((s_68_4.value()) != 0);
        // C s_68_6: const #0s : i
        let s_68_6: i128 = 0;
        // C s_68_7: const #0u : u64
        let s_68_7: u64 = 0;
        // D s_68_8: cast zx s_68_5 -> u64
        let s_68_8: u64 = (s_68_5 as u64);
        // C s_68_9: const #1u : u64
        let s_68_9: u64 = 1;
        // D s_68_10: and s_68_8 s_68_9
        let s_68_10: u64 = ((s_68_8) & (s_68_9));
        // D s_68_11: cmp-eq s_68_10 s_68_9
        let s_68_11: bool = ((s_68_10) == (s_68_9));
        // D s_68_12: lsl s_68_8 s_68_6
        let s_68_12: u64 = s_68_8 << s_68_6;
        // D s_68_13: or s_68_7 s_68_12
        let s_68_13: u64 = ((s_68_7) | (s_68_12));
        // D s_68_14: cmpl s_68_12
        let s_68_14: u64 = !s_68_12;
        // D s_68_15: and s_68_7 s_68_14
        let s_68_15: u64 = ((s_68_7) & (s_68_14));
        // D s_68_16: select s_68_11 s_68_13 s_68_15
        let s_68_16: u64 = if s_68_11 { s_68_13 } else { s_68_15 };
        // D s_68_17: cast trunc s_68_16 -> u8
        let s_68_17: bool = ((s_68_16) != 0);
        // D s_68_18: cast zx s_68_17 -> bv
        let s_68_18: Bits = Bits::new(s_68_17 as u128, 1u16);
        // C s_68_19: const #1u : u8
        let s_68_19: bool = true;
        // C s_68_20: cast zx s_68_19 -> bv
        let s_68_20: Bits = Bits::new(s_68_19 as u128, 1u16);
        // D s_68_21: cmp-eq s_68_18 s_68_20
        let s_68_21: bool = ((s_68_18) == (s_68_20));
        // D s_68_22: write-var gs#28341 <= s_68_21
        fn_state.gs_28341 = s_68_21;
        // N s_68_23: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_69_0: read-var indexlsbshadow#521:i
        let s_69_0: i128 = fn_state.indexlsbshadow_521;
        // D s_69_1: call __id(s_69_0)
        let s_69_1: i128 = u__id(state, tracer, s_69_0);
        // C s_69_2: const #39s : i
        let s_69_2: i128 = 39;
        // D s_69_3: cmp-le s_69_1 s_69_2
        let s_69_3: bool = ((s_69_1) <= (s_69_2));
        // D s_69_4: write-var gs#28336 <= s_69_3
        fn_state.gs_28336 = s_69_3;
        // N s_69_5: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var walkstate.8 <= s_70_0
        fn_state.walkstate._8 = s_70_0;
        // N s_70_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_71_0: read-var walkstate.0:struct
        let s_71_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_71_1: write-var ga#21949 <= s_71_0
        fn_state.ga_21949 = s_71_0;
        // D s_71_2: read-var ga#21949.1:struct
        let s_71_2: u32 = fn_state.ga_21949._1;
        // C s_71_3: const #0u : u32
        let s_71_3: u32 = 0;
        // D s_71_4: cmp-eq s_71_2 s_71_3
        let s_71_4: bool = ((s_71_2) == (s_71_3));
        // D s_71_5: write-var gs#28346 <= s_71_4
        fn_state.gs_28346 = s_71_4;
        // N s_71_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var walkstate.8 <= s_72_0
        fn_state.walkstate._8 = s_72_0;
        // N s_72_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_73_0: const #1u : u32
        let s_73_0: u32 = 1;
        // D s_73_1: write-var fault.16 <= s_73_0
        fn_state.fault._16 = s_73_0;
        // C s_73_2: const #() : ()
        let s_73_2: () = ();
        // S s_73_3: call __UNKNOWN_TTWState(s_73_2)
        let s_73_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_73_2,
        );
        // D s_73_4: read-var fault:struct
        let s_73_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_73_5: create-product struct = ["s_73_4", "s_73_3"]
        let s_73_5: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_73_4,
            _1: s_73_3,
        };
        // D s_73_6: write-var return_value <= s_73_5
        fn_state.return_value = s_73_5;
        // N s_73_7: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_74_0: const #7u : u32
        let s_74_0: u32 = 7;
        // D s_74_1: write-var fault.16 <= s_74_0
        fn_state.fault._16 = s_74_0;
        // C s_74_2: const #() : ()
        let s_74_2: () = ();
        // S s_74_3: call __UNKNOWN_TTWState(s_74_2)
        let s_74_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_74_2,
        );
        // D s_74_4: read-var fault:struct
        let s_74_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_74_5: create-product struct = ["s_74_4", "s_74_3"]
        let s_74_5: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_74_4,
            _1: s_74_3,
        };
        // D s_74_6: write-var return_value <= s_74_5
        fn_state.return_value = s_74_5;
        // N s_74_7: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_75_0: read-var walkstate.9:struct
        let s_75_0: ProductTypebf05c51f33174538 = fn_state.walkstate._9;
        // D s_75_1: write-var ga#21902 <= s_75_0
        fn_state.ga_21902 = s_75_0;
        // D s_75_2: read-var ga#21902.18:struct
        let s_75_2: bool = fn_state.ga_21902._18;
        // C s_75_3: const #60s : i
        let s_75_3: i128 = 60;
        // D s_75_4: read-var descriptor:u64
        let s_75_4: u64 = fn_state.descriptor;
        // D s_75_5: cast zx s_75_4 -> bv
        let s_75_5: Bits = Bits::new(s_75_4 as u128, 64u16);
        // C s_75_6: const #1u : u64
        let s_75_6: u64 = 1;
        // D s_75_7: bit-extract s_75_5 s_75_3 s_75_6
        let s_75_7: Bits = (Bits::new(
            ((s_75_5) >> (s_75_3)).value(),
            u16::try_from(s_75_6).unwrap(),
        ));
        // D s_75_8: cast reint s_75_7 -> u8
        let s_75_8: bool = ((s_75_7.value()) != 0);
        // C s_75_9: const #0s : i
        let s_75_9: i128 = 0;
        // C s_75_10: const #0u : u64
        let s_75_10: u64 = 0;
        // D s_75_11: cast zx s_75_8 -> u64
        let s_75_11: u64 = (s_75_8 as u64);
        // C s_75_12: const #1u : u64
        let s_75_12: u64 = 1;
        // D s_75_13: and s_75_11 s_75_12
        let s_75_13: u64 = ((s_75_11) & (s_75_12));
        // D s_75_14: cmp-eq s_75_13 s_75_12
        let s_75_14: bool = ((s_75_13) == (s_75_12));
        // D s_75_15: lsl s_75_11 s_75_9
        let s_75_15: u64 = s_75_11 << s_75_9;
        // D s_75_16: or s_75_10 s_75_15
        let s_75_16: u64 = ((s_75_10) | (s_75_15));
        // D s_75_17: cmpl s_75_15
        let s_75_17: u64 = !s_75_15;
        // D s_75_18: and s_75_10 s_75_17
        let s_75_18: u64 = ((s_75_10) & (s_75_17));
        // D s_75_19: select s_75_14 s_75_16 s_75_18
        let s_75_19: u64 = if s_75_14 { s_75_16 } else { s_75_18 };
        // D s_75_20: cast trunc s_75_19 -> u8
        let s_75_20: bool = ((s_75_19) != 0);
        // D s_75_21: cast zx s_75_2 -> bv
        let s_75_21: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_22: cast zx s_75_20 -> bv
        let s_75_22: Bits = Bits::new(s_75_20 as u128, 1u16);
        // D s_75_23: or s_75_21 s_75_22
        let s_75_23: Bits = ((s_75_21) | (s_75_22));
        // D s_75_24: cast reint s_75_23 -> u8
        let s_75_24: bool = ((s_75_23.value()) != 0);
        // D s_75_25: write-var walkstate.9.18 <= s_75_24
        fn_state.walkstate._9._18 = s_75_24;
        // D s_75_26: read-var walkstate.9:struct
        let s_75_26: ProductTypebf05c51f33174538 = fn_state.walkstate._9;
        // D s_75_27: write-var ga#21906 <= s_75_26
        fn_state.ga_21906 = s_75_26;
        // D s_75_28: read-var ga#21906.1:struct
        let s_75_28: u8 = fn_state.ga_21906._1;
        // C s_75_29: const #61s : i
        let s_75_29: i128 = 61;
        // D s_75_30: read-var descriptor:u64
        let s_75_30: u64 = fn_state.descriptor;
        // D s_75_31: cast zx s_75_30 -> bv
        let s_75_31: Bits = Bits::new(s_75_30 as u128, 64u16);
        // C s_75_32: const #1s : i64
        let s_75_32: i64 = 1;
        // C s_75_33: cast zx s_75_32 -> i
        let s_75_33: i128 = (i128::try_from(s_75_32).unwrap());
        // C s_75_34: const #1s : i
        let s_75_34: i128 = 1;
        // C s_75_35: add s_75_34 s_75_33
        let s_75_35: i128 = (s_75_34 + s_75_33);
        // D s_75_36: bit-extract s_75_31 s_75_29 s_75_35
        let s_75_36: Bits = (Bits::new(
            ((s_75_31) >> (s_75_29)).value(),
            u16::try_from(s_75_35).unwrap(),
        ));
        // D s_75_37: cast reint s_75_36 -> u8
        let s_75_37: u8 = (s_75_36.value() as u8);
        // D s_75_38: cast zx s_75_28 -> bv
        let s_75_38: Bits = Bits::new(s_75_28 as u128, 2u16);
        // D s_75_39: cast zx s_75_37 -> bv
        let s_75_39: Bits = Bits::new(s_75_37 as u128, 2u16);
        // D s_75_40: or s_75_38 s_75_39
        let s_75_40: Bits = ((s_75_38) | (s_75_39));
        // D s_75_41: cast reint s_75_40 -> u8
        let s_75_41: u8 = (s_75_40.value() as u8);
        // D s_75_42: write-var walkstate.9.1 <= s_75_41
        fn_state.walkstate._9._1 = s_75_41;
        // D s_75_43: read-var walkstate.9:struct
        let s_75_43: ProductTypebf05c51f33174538 = fn_state.walkstate._9;
        // D s_75_44: write-var ga#21909 <= s_75_43
        fn_state.ga_21909 = s_75_43;
        // D s_75_45: read-var ga#21909.6:struct
        let s_75_45: bool = fn_state.ga_21909._6;
        // C s_75_46: const #59s : i
        let s_75_46: i128 = 59;
        // D s_75_47: read-var descriptor:u64
        let s_75_47: u64 = fn_state.descriptor;
        // D s_75_48: cast zx s_75_47 -> bv
        let s_75_48: Bits = Bits::new(s_75_47 as u128, 64u16);
        // C s_75_49: const #1u : u64
        let s_75_49: u64 = 1;
        // D s_75_50: bit-extract s_75_48 s_75_46 s_75_49
        let s_75_50: Bits = (Bits::new(
            ((s_75_48) >> (s_75_46)).value(),
            u16::try_from(s_75_49).unwrap(),
        ));
        // D s_75_51: cast reint s_75_50 -> u8
        let s_75_51: bool = ((s_75_50.value()) != 0);
        // C s_75_52: const #0s : i
        let s_75_52: i128 = 0;
        // C s_75_53: const #0u : u64
        let s_75_53: u64 = 0;
        // D s_75_54: cast zx s_75_51 -> u64
        let s_75_54: u64 = (s_75_51 as u64);
        // C s_75_55: const #1u : u64
        let s_75_55: u64 = 1;
        // D s_75_56: and s_75_54 s_75_55
        let s_75_56: u64 = ((s_75_54) & (s_75_55));
        // D s_75_57: cmp-eq s_75_56 s_75_55
        let s_75_57: bool = ((s_75_56) == (s_75_55));
        // D s_75_58: lsl s_75_54 s_75_52
        let s_75_58: u64 = s_75_54 << s_75_52;
        // D s_75_59: or s_75_53 s_75_58
        let s_75_59: u64 = ((s_75_53) | (s_75_58));
        // D s_75_60: cmpl s_75_58
        let s_75_60: u64 = !s_75_58;
        // D s_75_61: and s_75_53 s_75_60
        let s_75_61: u64 = ((s_75_53) & (s_75_60));
        // D s_75_62: select s_75_57 s_75_59 s_75_61
        let s_75_62: u64 = if s_75_57 { s_75_59 } else { s_75_61 };
        // D s_75_63: cast trunc s_75_62 -> u8
        let s_75_63: bool = ((s_75_62) != 0);
        // D s_75_64: cast zx s_75_45 -> bv
        let s_75_64: Bits = Bits::new(s_75_45 as u128, 1u16);
        // D s_75_65: cast zx s_75_63 -> bv
        let s_75_65: Bits = Bits::new(s_75_63 as u128, 1u16);
        // D s_75_66: or s_75_64 s_75_65
        let s_75_66: Bits = ((s_75_64) | (s_75_65));
        // D s_75_67: cast reint s_75_66 -> u8
        let s_75_67: bool = ((s_75_66.value()) != 0);
        // D s_75_68: write-var walkstate.9.6 <= s_75_67
        fn_state.walkstate._9._6 = s_75_67;
        // N s_75_69: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_76_0: const #0u : u32
        let s_76_0: u32 = 0;
        // D s_76_1: write-var walkstate.0.1 <= s_76_0
        fn_state.walkstate._0._1 = s_76_0;
        // N s_76_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_77_0: const #63s : i
        let s_77_0: i128 = 63;
        // D s_77_1: read-var descriptor:u64
        let s_77_1: u64 = fn_state.descriptor;
        // D s_77_2: cast zx s_77_1 -> bv
        let s_77_2: Bits = Bits::new(s_77_1 as u128, 64u16);
        // C s_77_3: const #1u : u64
        let s_77_3: u64 = 1;
        // D s_77_4: bit-extract s_77_2 s_77_0 s_77_3
        let s_77_4: Bits = (Bits::new(
            ((s_77_2) >> (s_77_0)).value(),
            u16::try_from(s_77_3).unwrap(),
        ));
        // D s_77_5: cast reint s_77_4 -> u8
        let s_77_5: bool = ((s_77_4.value()) != 0);
        // C s_77_6: const #0s : i
        let s_77_6: i128 = 0;
        // C s_77_7: const #0u : u64
        let s_77_7: u64 = 0;
        // D s_77_8: cast zx s_77_5 -> u64
        let s_77_8: u64 = (s_77_5 as u64);
        // C s_77_9: const #1u : u64
        let s_77_9: u64 = 1;
        // D s_77_10: and s_77_8 s_77_9
        let s_77_10: u64 = ((s_77_8) & (s_77_9));
        // D s_77_11: cmp-eq s_77_10 s_77_9
        let s_77_11: bool = ((s_77_10) == (s_77_9));
        // D s_77_12: lsl s_77_8 s_77_6
        let s_77_12: u64 = s_77_8 << s_77_6;
        // D s_77_13: or s_77_7 s_77_12
        let s_77_13: u64 = ((s_77_7) | (s_77_12));
        // D s_77_14: cmpl s_77_12
        let s_77_14: u64 = !s_77_12;
        // D s_77_15: and s_77_7 s_77_14
        let s_77_15: u64 = ((s_77_7) & (s_77_14));
        // D s_77_16: select s_77_11 s_77_13 s_77_15
        let s_77_16: u64 = if s_77_11 { s_77_13 } else { s_77_15 };
        // D s_77_17: cast trunc s_77_16 -> u8
        let s_77_17: bool = ((s_77_16) != 0);
        // D s_77_18: cast zx s_77_17 -> bv
        let s_77_18: Bits = Bits::new(s_77_17 as u128, 1u16);
        // C s_77_19: const #1u : u8
        let s_77_19: bool = true;
        // C s_77_20: cast zx s_77_19 -> bv
        let s_77_20: Bits = Bits::new(s_77_19 as u128, 1u16);
        // D s_77_21: cmp-eq s_77_18 s_77_20
        let s_77_21: bool = ((s_77_18) == (s_77_20));
        // D s_77_22: write-var gs#28295 <= s_77_21
        fn_state.gs_28295 = s_77_21;
        // N s_77_23: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_78_0: const #7u : u32
        let s_78_0: u32 = 7;
        // D s_78_1: write-var fault.16 <= s_78_0
        fn_state.fault._16 = s_78_0;
        // C s_78_2: const #() : ()
        let s_78_2: () = ();
        // S s_78_3: call __UNKNOWN_TTWState(s_78_2)
        let s_78_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_78_2,
        );
        // D s_78_4: read-var fault:struct
        let s_78_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_78_5: create-product struct = ["s_78_4", "s_78_3"]
        let s_78_5: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_78_4,
            _1: s_78_3,
        };
        // D s_78_6: write-var return_value <= s_78_5
        fn_state.return_value = s_78_5;
        // N s_78_7: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_79_0: const #2u : u32
        let s_79_0: u32 = 2;
        // D s_79_1: read-var desctype:u32
        let s_79_1: u32 = fn_state.desctype;
        // D s_79_2: cmp-eq s_79_0 s_79_1
        let s_79_2: bool = ((s_79_0) == (s_79_1));
        // D s_79_3: not s_79_2
        let s_79_3: bool = !s_79_2;
        // N s_79_4: branch s_79_3 b81 b80
        if s_79_3 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_80_0: const #6u : u32
        let s_80_0: u32 = 6;
        // D s_80_1: write-var fault.16 <= s_80_0
        fn_state.fault._16 = s_80_0;
        // C s_80_2: const #() : ()
        let s_80_2: () = ();
        // S s_80_3: call __UNKNOWN_TTWState(s_80_2)
        let s_80_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_80_2,
        );
        // D s_80_4: read-var fault:struct
        let s_80_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_80_5: create-product struct = ["s_80_4", "s_80_3"]
        let s_80_5: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_80_4,
            _1: s_80_3,
        };
        // D s_80_6: write-var return_value <= s_80_5
        fn_state.return_value = s_80_5;
        // N s_80_7: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_81_0: const #1u : u32
        let s_81_0: u32 = 1;
        // D s_81_1: read-var desctype:u32
        let s_81_1: u32 = fn_state.desctype;
        // D s_81_2: cmp-eq s_81_0 s_81_1
        let s_81_2: bool = ((s_81_0) == (s_81_1));
        // D s_81_3: not s_81_2
        let s_81_3: bool = !s_81_2;
        // N s_81_4: branch s_81_3 b83 b82
        if s_81_3 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var walkstate.5 <= s_82_0
        fn_state.walkstate._5 = s_82_0;
        // N s_82_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_83_0: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call __UNKNOWN_TTWState(s_84_0)
        let s_84_1: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_84_0,
        );
        // D s_84_2: read-var fault:struct
        let s_84_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_84_3: create-product struct = ["s_84_2", "s_84_1"]
        let s_84_3: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_84_2,
            _1: s_84_1,
        };
        // D s_84_4: write-var return_value <= s_84_3
        fn_state.return_value = s_84_3;
        // N s_84_5: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_85_0: const #1u : u8
        let s_85_0: bool = true;
        // D s_85_1: read-var walkstate.6:struct
        let s_85_1: i128 = fn_state.walkstate._6;
        // D s_85_2: create-sum enum = 1:"s_85_1"
        let s_85_2: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_1(s_85_1);
        // D s_85_3: read-var fault:struct
        let s_85_3: ProductType1d757adad216cdef = fn_state.fault;
        // D s_85_4: read-var walkaddress:struct
        let s_85_4: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // D s_85_5: read-var walkaccess:struct
        let s_85_5: ProductType9878976b5bcce9c9 = fn_state.walkaccess;
        // D s_85_6: call AArch32_S2Translate(s_85_3, s_85_4, s_85_2, s_85_0, s_85_5)
        let s_85_6: ProductTypedc31059ca7e2391c = AArch32_S2Translate(
            state,
            tracer,
            s_85_3,
            s_85_4,
            s_85_2,
            s_85_0,
            s_85_5,
        );
        // D s_85_7: write-var ga#21873 <= s_85_6
        fn_state.ga_21873 = s_85_6;
        // D s_85_8: read-var ga#21873.0:struct
        let s_85_8: ProductType1d757adad216cdef = fn_state.ga_21873._0;
        // D s_85_9: read-var ga#21873.1:struct
        let s_85_9: ProductTypece7c66ccb2cab13e = fn_state.ga_21873._1;
        // D s_85_10: write-var s2fault <= s_85_8
        fn_state.s2fault = s_85_8;
        // D s_85_11: write-var s2walkaddress <= s_85_9
        fn_state.s2walkaddress = s_85_9;
        // D s_85_12: read-var s2fault.16:struct
        let s_85_12: u32 = fn_state.s2fault._16;
        // C s_85_13: const #0u : u32
        let s_85_13: u32 = 0;
        // D s_85_14: cmp-eq s_85_12 s_85_13
        let s_85_14: bool = ((s_85_12) == (s_85_13));
        // N s_85_15: branch s_85_14 b87 b86
        if s_85_14 {
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
        // D s_86_0: read-var walkparams.9:struct
        let s_86_0: bool = fn_state.walkparams._9;
        // C s_86_1: const #64s : i64
        let s_86_1: i64 = 64;
        // D s_86_2: read-var s2walkaddress:struct
        let s_86_2: ProductTypece7c66ccb2cab13e = fn_state.s2walkaddress;
        // D s_86_3: read-var walkaccess:struct
        let s_86_3: ProductType9878976b5bcce9c9 = fn_state.walkaccess;
        // D s_86_4: read-var fault:struct
        let s_86_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_86_5: read-var translation_info:struct
        let s_86_5: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_86_6: call FetchDescriptor(s_86_0, s_86_2, s_86_3, s_86_4, s_86_1, s_86_5)
        let s_86_6: ProductTypeb4cea7287e2eb9d6 = FetchDescriptor(
            state,
            tracer,
            s_86_0,
            s_86_2,
            s_86_3,
            s_86_4,
            s_86_1,
            s_86_5,
        );
        // D s_86_7: write-var gs#453685 <= s_86_6
        fn_state.gs_453685 = s_86_6;
        // D s_86_8: read-var gs#453685.0:struct
        let s_86_8: ProductType1d757adad216cdef = fn_state.gs_453685._0;
        // D s_86_9: read-var gs#453685.1:struct
        let s_86_9: Bits = fn_state.gs_453685._1;
        // D s_86_10: cast reint s_86_9 -> u64
        let s_86_10: u64 = (s_86_9.value() as u64);
        // D s_86_11: write-var fault <= s_86_8
        fn_state.fault = s_86_8;
        // D s_86_12: write-var descriptor <= s_86_10
        fn_state.descriptor = s_86_10;
        // N s_86_13: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_87_0: const #() : ()
        let s_87_0: () = ();
        // S s_87_1: call __UNKNOWN_TTWState(s_87_0)
        let s_87_1: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_87_0,
        );
        // D s_87_2: read-var s2fault:struct
        let s_87_2: ProductType1d757adad216cdef = fn_state.s2fault;
        // D s_87_3: create-product struct = ["s_87_2", "s_87_1"]
        let s_87_3: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_87_2,
            _1: s_87_1,
        };
        // D s_87_4: write-var return_value <= s_87_3
        fn_state.return_value = s_87_3;
        // N s_87_5: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_88_0: read-var accdesc.25:struct
        let s_88_0: u32 = fn_state.accdesc._25;
        // D s_88_1: call AArch32_EL2Enabled(s_88_0)
        let s_88_1: bool = AArch32_EL2Enabled(state, tracer, s_88_0);
        // D s_88_2: write-var gs#28266 <= s_88_1
        fn_state.gs_28266 = s_88_1;
        // N s_88_3: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_89_0: read-var indexlsbshadow#523:i
        let s_89_0: i128 = fn_state.indexlsbshadow_523;
        // D s_89_1: call __id(s_89_0)
        let s_89_1: i128 = u__id(state, tracer, s_89_0);
        // D s_89_2: read-var indexmsbshadow#522:i
        let s_89_2: i128 = fn_state.indexmsbshadow_522;
        // D s_89_3: call __id(s_89_2)
        let s_89_3: i128 = u__id(state, tracer, s_89_2);
        // D s_89_4: cmp-le s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) <= (s_89_3));
        // N s_89_5: branch s_89_4 b92 b90
        if s_89_4 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#28256 <= s_90_0
        fn_state.gs_28256 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_91_0: read-var gs#28256:u8
        let s_91_0: bool = fn_state.gs_28256;
        // D s_91_1: write-var gs#28257 <= s_91_0
        fn_state.gs_28257 = s_91_0;
        // N s_91_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_92_0: read-var indexmsbshadow#522:i
        let s_92_0: i128 = fn_state.indexmsbshadow_522;
        // D s_92_1: call __id(s_92_0)
        let s_92_1: i128 = u__id(state, tracer, s_92_0);
        // C s_92_2: const #32s : i
        let s_92_2: i128 = 32;
        // D s_92_3: cmp-lt s_92_1 s_92_2
        let s_92_3: bool = ((s_92_1) < (s_92_2));
        // D s_92_4: write-var gs#28256 <= s_92_3
        fn_state.gs_28256 = s_92_3;
        // N s_92_5: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_93_0: read-var walkstate.7:struct
        let s_93_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_93_1: write-var ga#21859 <= s_93_0
        fn_state.ga_21859 = s_93_0;
        // D s_93_2: read-var ga#21859.5:struct
        let s_93_2: u32 = fn_state.ga_21859._5;
        // D s_93_3: write-var walkaddress.2.5 <= s_93_2
        fn_state.walkaddress._2._5 = s_93_2;
        // N s_93_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_94_0: const #"Apply effective shareability at stage 1" : str
        let s_94_0: &'static str = "Apply effective shareability at stage 1";
        // S s_94_1: call __IMPDEF_boolean(s_94_0)
        let s_94_1: bool = u__IMPDEF_boolean(state, tracer, s_94_0);
        // S s_94_2: not s_94_1
        let s_94_2: bool = !s_94_1;
        // D s_94_3: write-var gs#28245 <= s_94_2
        fn_state.gs_28245 = s_94_2;
        // N s_94_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_95_0: read-var accdesc.25:struct
        let s_95_0: u32 = fn_state.accdesc._25;
        // C s_95_1: const #3u : u32
        let s_95_1: u32 = 3;
        // D s_95_2: cmp-eq s_95_0 s_95_1
        let s_95_2: bool = ((s_95_0) == (s_95_1));
        // C s_95_3: const #432u : u32
        let s_95_3: u32 = 432;
        // D s_95_4: read-reg s_95_3:u8
        let s_95_4: u8 = {
            let value = state.read_register::<u8>(s_95_3 as isize);
            tracer.read_register(s_95_3 as isize, value);
            value
        };
        // D s_95_5: call ELStateUsingAArch32(s_95_4, s_95_2)
        let s_95_5: bool = ELStateUsingAArch32(state, tracer, s_95_4, s_95_2);
        // N s_95_6: branch s_95_5 b98 b96
        if s_95_5 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_96_0: const #102552u : u32
        let s_96_0: u32 = 102552;
        // D s_96_1: read-reg s_96_0:struct
        let s_96_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: call _get_HCR_EL2_Type_VM(s_96_1)
        let s_96_2: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_96_1);
        // D s_96_3: write-var ga#21854 <= s_96_2
        fn_state.ga_21854 = s_96_2;
        // N s_96_4: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_97_0: read-var ga#21854:u8
        let s_97_0: bool = fn_state.ga_21854;
        // D s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 1u16);
        // C s_97_2: const #1u : u8
        let s_97_2: bool = true;
        // C s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // D s_97_4: cmp-eq s_97_1 s_97_3
        let s_97_4: bool = ((s_97_1) == (s_97_3));
        // D s_97_5: write-var gs#28244 <= s_97_4
        fn_state.gs_28244 = s_97_4;
        // N s_97_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call HCR_read(s_98_0)
        let s_98_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_98_0);
        // S s_98_2: call _get_HCR_Type_VM(s_98_1)
        let s_98_2: bool = u_get_HCR_Type_VM(state, tracer, s_98_1);
        // D s_98_3: write-var ga#21854 <= s_98_2
        fn_state.ga_21854 = s_98_2;
        // N s_98_4: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_99_0: read-var accdesc.25:struct
        let s_99_0: u32 = fn_state.accdesc._25;
        // D s_99_1: call AArch32_EL2Enabled(s_99_0)
        let s_99_1: bool = AArch32_EL2Enabled(state, tracer, s_99_0);
        // D s_99_2: write-var gs#28243 <= s_99_1
        fn_state.gs_28243 = s_99_1;
        // N s_99_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_100_0: const #() : ()
        let s_100_0: () = ();
        // S s_100_1: call NormalNCMemAttr(s_100_0)
        let s_100_1: ProductTypef170cab34335b70c = NormalNCMemAttr(
            state,
            tracer,
            s_100_0,
        );
        // D s_100_2: write-var walkaddress.2 <= s_100_1
        fn_state.walkaddress._2 = s_100_1;
        // D s_100_3: read-var walkstate.7:struct
        let s_100_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_100_4: write-var ga#21847 <= s_100_3
        fn_state.ga_21847 = s_100_3;
        // D s_100_5: read-var ga#21847.7:struct
        let s_100_5: bool = fn_state.ga_21847._7;
        // D s_100_6: write-var walkaddress.2.7 <= s_100_5
        fn_state.walkaddress._2._7 = s_100_5;
        // N s_100_7: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_101_0: const #1u : u8
        let s_101_0: bool = true;
        // D s_101_1: write-var walkstate.8 <= s_101_0
        fn_state.walkstate._8 = s_101_0;
        // N s_101_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_102_0: read-var baselsb:i
        let s_102_0: i128 = fn_state.baselsb;
        // D s_102_1: call __id(s_102_0)
        let s_102_1: i128 = u__id(state, tracer, s_102_0);
        // C s_102_2: const #39s : i
        let s_102_2: i128 = 39;
        // D s_102_3: cmp-le s_102_1 s_102_2
        let s_102_3: bool = ((s_102_1) <= (s_102_2));
        // D s_102_4: write-var gs#28227 <= s_102_3
        fn_state.gs_28227 = s_102_3;
        // N s_102_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_103_0: const #1u : u32
        let s_103_0: u32 = 1;
        // D s_103_1: write-var baseaddress.1 <= s_103_0
        fn_state.baseaddress._1 = s_103_0;
        // N s_103_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_104_0: const #7u : u32
        let s_104_0: u32 = 7;
        // D s_104_1: write-var fault.16 <= s_104_0
        fn_state.fault._16 = s_104_0;
        // C s_104_2: const #0s : i
        let s_104_2: i128 = 0;
        // D s_104_3: write-var fault.9 <= s_104_2
        fn_state.fault._9 = s_104_2;
        // C s_104_4: const #() : ()
        let s_104_4: () = ();
        // S s_104_5: call __UNKNOWN_TTWState(s_104_4)
        let s_104_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_104_4,
        );
        // D s_104_6: read-var fault:struct
        let s_104_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_104_7: create-product struct = ["s_104_6", "s_104_5"]
        let s_104_7: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_104_6,
            _1: s_104_5,
        };
        // D s_104_8: write-var return_value <= s_104_7
        fn_state.return_value = s_104_7;
        // N s_104_9: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_105_0: const #1s : i
        let s_105_0: i128 = 1;
        // D s_105_1: write-var fault.9 <= s_105_0
        fn_state.fault._9 = s_105_0;
        // C s_105_2: const #6u : u32
        let s_105_2: u32 = 6;
        // D s_105_3: write-var fault.16 <= s_105_2
        fn_state.fault._16 = s_105_2;
        // C s_105_4: const #() : ()
        let s_105_4: () = ();
        // S s_105_5: call __UNKNOWN_TTWState(s_105_4)
        let s_105_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_105_4,
        );
        // D s_105_6: read-var fault:struct
        let s_105_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_105_7: create-product struct = ["s_105_6", "s_105_5"]
        let s_105_7: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_105_6,
            _1: s_105_5,
        };
        // D s_105_8: write-var return_value <= s_105_7
        fn_state.return_value = s_105_7;
        // N s_105_9: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_106_0: read-var epd:u8
        let s_106_0: bool = fn_state.epd;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 1u16);
        // C s_106_2: const #1u : u8
        let s_106_2: bool = true;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#28187 <= s_106_4
        fn_state.gs_28187 = s_106_4;
        // N s_106_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_107_0: read-var walkparams.32:struct
        let s_107_0: u8 = fn_state.walkparams._32;
        // D s_107_1: write-var txsz <= s_107_0
        fn_state.txsz = s_107_0;
        // D s_107_2: read-var ttbr0:u64
        let s_107_2: u64 = fn_state.ttbr0;
        // D s_107_3: write-var ttbr <= s_107_2
        fn_state.ttbr = s_107_2;
        // D s_107_4: read-var ttbcr:struct
        let s_107_4: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_107_5: call _get_TTBCR_Type_EPD0(s_107_4)
        let s_107_5: bool = u_get_TTBCR_Type_EPD0(state, tracer, s_107_4);
        // D s_107_6: write-var epd <= s_107_5
        fn_state.epd = s_107_5;
        // N s_107_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_108_0: const #() : ()
        let s_108_0: () = ();
        // S s_108_1: call TTBCR_NS_read(s_108_0)
        let s_108_1: ProductType700c18a878c5601b = TTBCR_NS_read(state, tracer, s_108_0);
        // D s_108_2: write-var ttbcr <= s_108_1
        fn_state.ttbcr = s_108_1;
        // C s_108_3: const #11616u : u32
        let s_108_3: u32 = 11616;
        // D s_108_4: read-reg s_108_3:u64
        let s_108_4: u64 = {
            let value = state.read_register::<u64>(s_108_3 as isize);
            tracer.read_register(s_108_3 as isize, value);
            value
        };
        // D s_108_5: write-var ttbr0 <= s_108_4
        fn_state.ttbr0 = s_108_4;
        // C s_108_6: const #20024u : u32
        let s_108_6: u32 = 20024;
        // D s_108_7: read-reg s_108_6:u64
        let s_108_7: u64 = {
            let value = state.read_register::<u64>(s_108_6 as isize);
            tracer.read_register(s_108_6 as isize, value);
            value
        };
        // D s_108_8: write-var ttbr1 <= s_108_7
        fn_state.ttbr1 = s_108_7;
        // N s_108_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_109_0: const #15368u : u32
        let s_109_0: u32 = 15368;
        // D s_109_1: read-reg s_109_0:struct
        let s_109_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_109_0 as isize);
            tracer.read_register(s_109_0 as isize, value);
            value
        };
        // D s_109_2: write-var ttbcr <= s_109_1
        fn_state.ttbcr = s_109_1;
        // C s_109_3: const #101800u : u32
        let s_109_3: u32 = 101800;
        // D s_109_4: read-reg s_109_3:u64
        let s_109_4: u64 = {
            let value = state.read_register::<u64>(s_109_3 as isize);
            tracer.read_register(s_109_3 as isize, value);
            value
        };
        // D s_109_5: write-var ttbr0 <= s_109_4
        fn_state.ttbr0 = s_109_4;
        // C s_109_6: const #19120u : u32
        let s_109_6: u32 = 19120;
        // D s_109_7: read-reg s_109_6:u64
        let s_109_7: u64 = {
            let value = state.read_register::<u64>(s_109_6 as isize);
            tracer.read_register(s_109_6 as isize, value);
            value
        };
        // D s_109_8: write-var ttbr1 <= s_109_7
        fn_state.ttbr1 = s_109_7;
        // N s_109_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_110_0: const #14824u : u32
        let s_110_0: u32 = 14824;
        // D s_110_1: read-reg s_110_0:u64
        let s_110_1: u64 = {
            let value = state.read_register::<u64>(s_110_0 as isize);
            tracer.read_register(s_110_0 as isize, value);
            value
        };
        // D s_110_2: write-var ttbr <= s_110_1
        fn_state.ttbr = s_110_1;
        // D s_110_3: read-var walkparams.32:struct
        let s_110_3: u8 = fn_state.walkparams._32;
        // D s_110_4: write-var txsz <= s_110_3
        fn_state.txsz = s_110_3;
        // C s_110_5: const #0u : u32
        let s_110_5: u32 = 0;
        // D s_110_6: write-var varange <= s_110_5
        fn_state.varange = s_110_5;
        // N s_110_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_111_0: read-var accdesc.25:struct
        let s_111_0: u32 = fn_state.accdesc._25;
        // D s_111_1: read-var regime:u32
        let s_111_1: u32 = fn_state.regime;
        // D s_111_2: read-var va:u32
        let s_111_2: u32 = fn_state.va;
        // D s_111_3: call AArch32_GetS1TLBContext(s_111_1, s_111_0, s_111_2)
        let s_111_3: ProductTypec0d0fb0603850c4c = AArch32_GetS1TLBContext(
            state,
            tracer,
            s_111_1,
            s_111_0,
            s_111_2,
        );
        // D s_111_4: write-var tlbcontext <= s_111_3
        fn_state.tlbcontext = s_111_3;
        // D s_111_5: read-var tlbcontext:struct
        let s_111_5: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_111_6: call S1TLBLookup(s_111_5)
        let s_111_6: ProductTypeeb828c17bbe5e68 = S1TLBLookup(state, tracer, s_111_5);
        // D s_111_7: write-var tlbentry <= s_111_6
        fn_state.tlbentry = s_111_6;
        // D s_111_8: read-var tlbentry.1:struct
        let s_111_8: bool = fn_state.tlbentry._1;
        // N s_111_9: branch s_111_8 b113 b112
        if s_111_8 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_112_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_113_0: read-var tlbentry.0:struct
        let s_113_0: ProductTypee47dd77b186df56e = fn_state.tlbentry._0;
        // D s_113_1: write-var ga#21802 <= s_113_0
        fn_state.ga_21802 = s_113_0;
        // D s_113_2: read-var ga#21802.5:struct
        let s_113_2: ProductType96e7acababe246a1 = fn_state.ga_21802._5;
        // D s_113_3: write-var finalwalkstateshadow#524 <= s_113_2
        fn_state.finalwalkstateshadow_524 = s_113_2;
        // D s_113_4: read-var finalwalkstateshadow#524.6:struct
        let s_113_4: i128 = fn_state.finalwalkstateshadow_524._6;
        // D s_113_5: write-var fault.9 <= s_113_4
        fn_state.fault._9 = s_113_4;
        // D s_113_6: read-var fault:struct
        let s_113_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_113_7: create-product struct = ["s_113_6", "s_113_2"]
        let s_113_7: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_113_6,
            _1: s_113_2,
        };
        // D s_113_8: write-var return_value <= s_113_7
        fn_state.return_value = s_113_7;
        // N s_113_9: jump b62
        return block_62(state, tracer, fn_state);
    }
}
