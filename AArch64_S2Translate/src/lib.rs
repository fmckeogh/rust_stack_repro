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
use AArch64_S2TxSZFaults::*;
use AArch64_S2MinTxSZ::*;
use AArch64_S2HasAlignmentFault::*;
use AArch64_MaxTxSZ::*;
use AArch64_S2Walk::*;
use Zeros::*;
use VMID_read::*;
use set_subrange_zeros::*;
use AArch64_S2CheckPermissions::*;
use NormalNCMemAttr::*;
use S2TLBLookup::*;
use AArch64_SettingAccessFlagPermitted::*;
use CreateAccDescTTEUpdate::*;
use TranslationSize::*;
use AArch64_MemSwapTableDesc::*;
use StageOA::*;
use u_get_HCR_EL2_Type_CD::*;
use AArch64_SettingDirtyStatePermitted::*;
use AArch64_S2InconsistentSL::*;
use AArch64_GetS2TTWParams::*;
use AArch64_S2InvalidSL::*;
use CreateAddressDescriptor::*;
use ContiguousSize::*;
use AArch64_IPAIsOutOfRange::*;
use S2CombineS1MemAttrs::*;
use u__UNKNOWN_AddressDescriptor::*;
use integer_subrange::*;
use Bit::*;
use S2TLBCache::*;
use u_get_HCR_EL2_Type_ID::*;
use AArch64_S2OutputMECID::*;
use common::*;
pub fn AArch64_S2Translate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    ipa: ProductTypece7c66ccb2cab13e,
    s1aarch64: bool,
    s1level: SumTypebf36e919d71ba1d6,
    aligned: bool,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductTypedc31059ca7e2391c {
    #[derive(Default)]
    struct FunctionState {
        gs_19603: bool,
        ga_14891: ProductTypef170cab34335b70c,
        ga_14913: ProductTypeda0231e9dc169f81,
        gs_19601: bool,
        fault: ProductType1d757adad216cdef,
        ga_14911: ProductTypeda0231e9dc169f81,
        gs_19507: bool,
        gs_19611: bool,
        gs_19602: bool,
        ga_14834: ProductType963c597a88a9ddbc,
        gs_19506: bool,
        oa: ProductTypeda0231e9dc169f81,
        memattrs: ProductTypef170cab34335b70c,
        gs_446538: ProductType1f0c48777d4d25a0,
        ga_14830: ProductTypeda0231e9dc169f81,
        gs_19512: bool,
        gs_19511: bool,
        descaccess: ProductType9878976b5bcce9c9,
        return_value: ProductTypedc31059ca7e2391c,
        gs_446519: ProductType4b99944cd5e0b59d,
        translation_info: ProductTypeb525737120e184b3,
        gs_19604: bool,
        s2fs1mro: bool,
        gs_19572: bool,
        gs_19605: bool,
        gs_19516: bool,
        gs_19515: bool,
        new_desc: u128,
        s2mintxsz: i128,
        ga_14848: ProductTypef170cab34335b70c,
        ga_14877: ProductTypef170cab34335b70c,
        tlbrecord: ProductTypee47dd77b186df56e,
        ga_14871: ProductTypef170cab34335b70c,
        gs_19514: bool,
        gs_446512: ProductType4b99944cd5e0b59d,
        ga_14773: ProductTypeda0231e9dc169f81,
        s2_memattrs: ProductTypef170cab34335b70c,
        gs_446547: ProductTypeb4cea7287e2eb9d6,
        gs_19485: bool,
        s2maxtxsz: i128,
        descpaddr: ProductTypece7c66ccb2cab13e,
        gs_19484: bool,
        gs_19569: bool,
        pa: ProductTypece7c66ccb2cab13e,
        walkstate: ProductType96e7acababe246a1,
        ga_14897: ProductType183e6678e5239c85,
        ga_14884: ProductTypef170cab34335b70c,
        gs_19608: bool,
        mem_desc: u128,
        gs_19607: bool,
        gs_19609: bool,
        gs_446555: ProductTypeb4cea7287e2eb9d6,
        ga_14793: ProductType3b8bd97143a1dd5c,
        walkparams: ProductTypeb05ce25a107f0c5e,
        ga_14906: ProductTypeda0231e9dc169f81,
        tlbentry: ProductTypeeb828c17bbe5e68,
        gs_19513: bool,
        ga_14895: ProductType183e6678e5239c85,
        gs_19508: bool,
        descriptor: u128,
        gs_19606: bool,
        gs_19505: bool,
        fault_in: ProductType1d757adad216cdef,
        ipa: ProductTypece7c66ccb2cab13e,
        s1aarch64: bool,
        s1level: SumTypebf36e919d71ba1d6,
        aligned: bool,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        fault_in,
        ipa,
        s1aarch64,
        s1level,
        aligned,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_0_0: read-var accdesc.25:struct
        let s_0_0: u32 = fn_state.accdesc._25;
        // D s_0_1: read-var ipa.3:struct
        let s_0_1: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_0_2: write-var ga#14913 <= s_0_1
        fn_state.ga_14913 = s_0_1;
        // D s_0_3: read-var ga#14913.1:struct
        let s_0_3: u32 = fn_state.ga_14913._1;
        // D s_0_4: read-var s1aarch64:u8
        let s_0_4: bool = fn_state.s1aarch64;
        // D s_0_5: call AArch64_GetS2TTWParams(s_0_0, s_0_3, s_0_4)
        let s_0_5: ProductTypeb05ce25a107f0c5e = AArch64_GetS2TTWParams(
            state,
            tracer,
            s_0_0,
            s_0_3,
            s_0_4,
        );
        // D s_0_6: write-var walkparams <= s_0_5
        fn_state.walkparams = s_0_5;
        // D s_0_7: read-var fault_in:struct
        let s_0_7: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_8: write-var fault <= s_0_7
        fn_state.fault = s_0_7;
        // C s_0_9: const #0u : u32
        let s_0_9: u32 = 0;
        // D s_0_10: write-var fault.16 <= s_0_9
        fn_state.fault._16 = s_0_9;
        // C s_0_11: const #1u : u8
        let s_0_11: bool = true;
        // D s_0_12: write-var fault.15 <= s_0_11
        fn_state.fault._15 = s_0_11;
        // D s_0_13: read-var accdesc.1:struct
        let s_0_13: u32 = fn_state.accdesc._1;
        // C s_0_14: const #13u : u32
        let s_0_14: u32 = 13;
        // D s_0_15: cmp-eq s_0_13 s_0_14
        let s_0_15: bool = ((s_0_13) == (s_0_14));
        // D s_0_16: write-var fault.14 <= s_0_15
        fn_state.fault._14 = s_0_15;
        // D s_0_17: read-var ipa.3:struct
        let s_0_17: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_0_18: write-var fault.8 <= s_0_17
        fn_state.fault._8 = s_0_17;
        // D s_0_19: read-var walkparams.30:struct
        let s_0_19: bool = fn_state.walkparams._30;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 1u16);
        // C s_0_21: const #1u : u8
        let s_0_21: bool = true;
        // C s_0_22: cast zx s_0_21 -> bv
        let s_0_22: Bits = Bits::new(s_0_21 as u128, 1u16);
        // D s_0_23: cmp-ne s_0_20 s_0_22
        let s_0_23: bool = ((s_0_20) != (s_0_22));
        // N s_0_24: branch s_0_23 b134 b1
        if s_0_23 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_1_0: read-var walkparams.2:struct
        let s_1_0: bool = fn_state.walkparams._2;
        // D s_1_1: read-var walkparams.3:struct
        let s_1_1: bool = fn_state.walkparams._3;
        // D s_1_2: read-var walkparams.26:struct
        let s_1_2: u32 = fn_state.walkparams._26;
        // D s_1_3: read-var s1aarch64:u8
        let s_1_3: bool = fn_state.s1aarch64;
        // D s_1_4: call AArch64_S2MinTxSZ(s_1_0, s_1_1, s_1_2, s_1_3)
        let s_1_4: i128 = AArch64_S2MinTxSZ(state, tracer, s_1_0, s_1_1, s_1_2, s_1_3);
        // D s_1_5: write-var s2mintxsz <= s_1_4
        fn_state.s2mintxsz = s_1_4;
        // D s_1_6: read-var walkparams.26:struct
        let s_1_6: u32 = fn_state.walkparams._26;
        // D s_1_7: call AArch64_MaxTxSZ(s_1_6)
        let s_1_7: i128 = AArch64_MaxTxSZ(state, tracer, s_1_6);
        // D s_1_8: write-var s2maxtxsz <= s_1_7
        fn_state.s2maxtxsz = s_1_7;
        // D s_1_9: read-var walkparams:struct
        let s_1_9: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_1_10: read-var s1aarch64:u8
        let s_1_10: bool = fn_state.s1aarch64;
        // D s_1_11: call AArch64_S2TxSZFaults(s_1_9, s_1_10)
        let s_1_11: bool = AArch64_S2TxSZFaults(state, tracer, s_1_9, s_1_10);
        // N s_1_12: branch s_1_11 b133 b2
        if s_1_11 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_2_0: read-var walkparams.29:struct
        let s_2_0: u8 = fn_state.walkparams._29;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 6u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: read-var s2mintxsz:i
        let s_2_5: i128 = fn_state.s2mintxsz;
        // D s_2_6: cmp-lt s_2_4 s_2_5
        let s_2_6: bool = ((s_2_4) < (s_2_5));
        // N s_2_7: branch s_2_6 b132 b3
        if s_2_6 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_3_0: read-var walkparams.29:struct
        let s_3_0: u8 = fn_state.walkparams._29;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 6u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var s2maxtxsz:i
        let s_3_5: i128 = fn_state.s2maxtxsz;
        // D s_3_6: cmp-gt s_3_4 s_3_5
        let s_3_6: bool = ((s_3_4) > (s_3_5));
        // N s_3_7: branch s_3_6 b131 b4
        if s_3_6 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_5_0: read-var walkparams.2:struct
        let s_5_0: bool = fn_state.walkparams._2;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b127 b6
        if s_5_4 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#19485 <= s_6_0
        fn_state.gs_19485 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_7_0: read-var gs#19485:u8
        let s_7_0: bool = fn_state.gs_19485;
        // N s_7_1: branch s_7_0 b126 b8
        if s_7_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_8_0: read-var ipa.3:struct
        let s_8_0: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_8_1: write-var ga#14773 <= s_8_0
        fn_state.ga_14773 = s_8_0;
        // D s_8_2: read-var ga#14773.0:struct
        let s_8_2: u64 = fn_state.ga_14773._0;
        // D s_8_3: read-var walkparams:struct
        let s_8_3: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_8_4: call AArch64_IPAIsOutOfRange(s_8_2, s_8_3)
        let s_8_4: bool = AArch64_IPAIsOutOfRange(state, tracer, s_8_2, s_8_3);
        // N s_8_5: branch s_8_4 b125 b9
        if s_8_4 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_10_0: read-var walkparams.2:struct
        let s_10_0: bool = fn_state.walkparams._2;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b124 b11
        if s_10_4 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // D s_11_1: read-var fault:struct
        let s_11_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_11_2: read-var ipa:struct
        let s_11_2: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_11_3: read-var walkparams:struct
        let s_11_3: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_11_4: read-var accdesc:struct
        let s_11_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_11_5: read-var s1level:enum
        let s_11_5: SumTypebf36e919d71ba1d6 = fn_state.s1level;
        // D s_11_6: call AArch64_S2Walk(s_11_1, s_11_2, s_11_3, s_11_4, s_11_5, s_11_0)
        let s_11_6: ProductType4b99944cd5e0b59d = AArch64_S2Walk(
            state,
            tracer,
            s_11_1,
            s_11_2,
            s_11_3,
            s_11_4,
            s_11_5,
            s_11_0,
        );
        // D s_11_7: write-var gs#446512 <= s_11_6
        fn_state.gs_446512 = s_11_6;
        // D s_11_8: read-var gs#446512.0:struct
        let s_11_8: ProductType1d757adad216cdef = fn_state.gs_446512._0;
        // D s_11_9: read-var gs#446512.1:struct
        let s_11_9: ProductTypece7c66ccb2cab13e = fn_state.gs_446512._1;
        // D s_11_10: read-var gs#446512.2:struct
        let s_11_10: ProductType96e7acababe246a1 = fn_state.gs_446512._2;
        // D s_11_11: read-var gs#446512.3:struct
        let s_11_11: Bits = fn_state.gs_446512._3;
        // D s_11_12: cast reint s_11_11 -> u64
        let s_11_12: u64 = (s_11_11.value() as u64);
        // D s_11_13: write-var fault <= s_11_8
        fn_state.fault = s_11_8;
        // D s_11_14: write-var descpaddr <= s_11_9
        fn_state.descpaddr = s_11_9;
        // D s_11_15: write-var walkstate <= s_11_10
        fn_state.walkstate = s_11_10;
        // C s_11_16: const #0s : i
        let s_11_16: i128 = 0;
        // D s_11_17: read-var descriptor:u128
        let s_11_17: u128 = fn_state.descriptor;
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 128u16);
        // D s_11_19: cast zx s_11_12 -> bv
        let s_11_19: Bits = Bits::new(s_11_12 as u128, 64u16);
        // C s_11_20: const #63s : i
        let s_11_20: i128 = 63;
        // C s_11_21: const #1u : u64
        let s_11_21: u64 = 1;
        // C s_11_22: cast zx s_11_21 -> bv
        let s_11_22: Bits = Bits::new(s_11_21 as u128, 64u16);
        // C s_11_23: lsl s_11_22 s_11_20
        let s_11_23: Bits = s_11_22 << s_11_20;
        // C s_11_24: sub s_11_23 s_11_22
        let s_11_24: Bits = ((s_11_23) - (s_11_22));
        // D s_11_25: and s_11_19 s_11_24
        let s_11_25: Bits = ((s_11_19) & (s_11_24));
        // D s_11_26: lsl s_11_25 s_11_16
        let s_11_26: Bits = s_11_25 << s_11_16;
        // C s_11_27: lsl s_11_24 s_11_16
        let s_11_27: Bits = s_11_24 << s_11_16;
        // C s_11_28: cmpl s_11_27
        let s_11_28: Bits = !s_11_27;
        // D s_11_29: and s_11_18 s_11_28
        let s_11_29: Bits = ((s_11_18) & (s_11_28));
        // D s_11_30: or s_11_29 s_11_26
        let s_11_30: Bits = ((s_11_29) | (s_11_26));
        // D s_11_31: cast reint s_11_30 -> u128
        let s_11_31: u128 = (s_11_30.value() as u128);
        // D s_11_32: write-var descriptor <= s_11_31
        fn_state.descriptor = s_11_31;
        // C s_11_33: const #128s : i
        let s_11_33: i128 = 128;
        // C s_11_34: const #127s : i
        let s_11_34: i128 = 127;
        // C s_11_35: const #64s : i
        let s_11_35: i128 = 64;
        // D s_11_36: read-var descriptor:u128
        let s_11_36: u128 = fn_state.descriptor;
        // D s_11_37: cast zx s_11_36 -> bv
        let s_11_37: Bits = Bits::new(s_11_36 as u128, 128u16);
        // D s_11_38: call set_subrange_zeros(s_11_33, s_11_37, s_11_34, s_11_35)
        let s_11_38: Bits = set_subrange_zeros(
            state,
            tracer,
            s_11_33,
            s_11_37,
            s_11_34,
            s_11_35,
        );
        // D s_11_39: cast reint s_11_38 -> u128
        let s_11_39: u128 = (s_11_38.value() as u128);
        // D s_11_40: write-var descriptor <= s_11_39
        fn_state.descriptor = s_11_39;
        // N s_11_41: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_12_0: read-var fault.16:struct
        let s_12_0: u32 = fn_state.fault._16;
        // C s_12_1: const #0u : u32
        let s_12_1: u32 = 0;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // N s_12_3: branch s_12_2 b123 b13
        if s_12_2 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_13_0: read-var walkstate.7:struct
        let s_13_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_13_1: read-var accdesc:struct
        let s_13_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_13_2: read-var aligned:u8
        let s_13_2: bool = fn_state.aligned;
        // D s_13_3: call AArch64_S2HasAlignmentFault(s_13_1, s_13_2, s_13_0)
        let s_13_3: bool = AArch64_S2HasAlignmentFault(
            state,
            tracer,
            s_13_1,
            s_13_2,
            s_13_0,
        );
        // N s_13_4: branch s_13_3 b122 b14
        if s_13_3 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_15_0: read-var fault.16:struct
        let s_15_0: u32 = fn_state.fault._16;
        // C s_15_1: const #0u : u32
        let s_15_1: u32 = 0;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // N s_15_3: branch s_15_2 b121 b16
        if s_15_2 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_17_0: read-var descriptor:u128
        let s_17_0: u128 = fn_state.descriptor;
        // D s_17_1: write-var new_desc <= s_17_0
        fn_state.new_desc = s_17_0;
        // D s_17_2: read-var walkparams.7:struct
        let s_17_2: bool = fn_state.walkparams._7;
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // C s_17_4: const #1u : u8
        let s_17_4: bool = true;
        // C s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 1u16);
        // D s_17_6: cmp-eq s_17_3 s_17_5
        let s_17_6: bool = ((s_17_3) == (s_17_5));
        // N s_17_7: branch s_17_6 b120 b18
        if s_17_6 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#19506 <= s_18_0
        fn_state.gs_19506 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_19_0: read-var gs#19506:u8
        let s_19_0: bool = fn_state.gs_19506;
        // N s_19_1: branch s_19_0 b119 b20
        if s_19_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_21_0: read-var fault:struct
        let s_21_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_21_1: call AArch64_SettingDirtyStatePermitted(s_21_0)
        let s_21_1: bool = AArch64_SettingDirtyStatePermitted(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b118 b22
        if s_21_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#19507 <= s_22_0
        fn_state.gs_19507 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_23_0: read-var gs#19507:u8
        let s_23_0: bool = fn_state.gs_19507;
        // N s_23_1: branch s_23_0 b117 b24
        if s_23_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#19508 <= s_24_0
        fn_state.gs_19508 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_25_0: read-var gs#19508:u8
        let s_25_0: bool = fn_state.gs_19508;
        // N s_25_1: branch s_25_0 b113 b26
        if s_25_0 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#19512 <= s_26_0
        fn_state.gs_19512 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_27_0: read-var gs#19512:u8
        let s_27_0: bool = fn_state.gs_19512;
        // N s_27_1: branch s_27_0 b112 b28
        if s_27_0 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#19513 <= s_28_0
        fn_state.gs_19513 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_29_0: read-var gs#19513:u8
        let s_29_0: bool = fn_state.gs_19513;
        // N s_29_1: branch s_29_0 b105 b30
        if s_29_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#19516 <= s_30_0
        fn_state.gs_19516 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_31_0: read-var gs#19516:u8
        let s_31_0: bool = fn_state.gs_19516;
        // N s_31_1: branch s_31_0 b104 b32
        if s_31_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_32_0: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_33_0: read-var new_desc:u128
        let s_33_0: u128 = fn_state.new_desc;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 128u16);
        // D s_33_2: read-var descriptor:u128
        let s_33_2: u128 = fn_state.descriptor;
        // D s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 128u16);
        // D s_33_4: cmp-ne s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) != (s_33_3));
        // N s_33_5: branch s_33_4 b101 b34
        if s_33_4 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_34_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_35_0: read-var new_desc:u128
        let s_35_0: u128 = fn_state.new_desc;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 128u16);
        // D s_35_2: read-var descriptor:u128
        let s_35_2: u128 = fn_state.descriptor;
        // D s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 128u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // N s_35_5: branch s_35_4 b100 b36
        if s_35_4 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_36_0: read-var mem_desc:u128
        let s_36_0: u128 = fn_state.mem_desc;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 128u16);
        // D s_36_2: read-var new_desc:u128
        let s_36_2: u128 = fn_state.new_desc;
        // D s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 128u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: write-var gs#19505 <= s_36_4
        fn_state.gs_19505 = s_36_4;
        // N s_36_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_37_0: read-var gs#19505:u8
        let s_37_0: bool = fn_state.gs_19505;
        // N s_37_1: branch s_37_0 b38 b10
        if s_37_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_38_0: const #19088u : u32
        let s_38_0: u32 = 19088;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: bool = {
            let value = state.read_register::<bool>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // N s_38_2: branch s_38_1 b99 b39
        if s_38_1 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#19569 <= s_39_0
        fn_state.gs_19569 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_40_0: read-var gs#19569:u8
        let s_40_0: bool = fn_state.gs_19569;
        // N s_40_1: branch s_40_0 b98 b41
        if s_40_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#19572 <= s_41_0
        fn_state.gs_19572 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_42_0: read-var gs#19572:u8
        let s_42_0: bool = fn_state.gs_19572;
        // N s_42_1: branch s_42_0 b89 b43
        if s_42_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_43_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_44_0: read-var fault.16:struct
        let s_44_0: u32 = fn_state.fault._16;
        // C s_44_1: const #0u : u32
        let s_44_1: u32 = 0;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // N s_44_3: branch s_44_2 b88 b45
        if s_44_2 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_45_0: read-var ipa.3:struct
        let s_45_0: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_45_1: write-var ga#14911 <= s_45_0
        fn_state.ga_14911 = s_45_0;
        // D s_45_2: read-var ga#14911.0:struct
        let s_45_2: u64 = fn_state.ga_14911._0;
        // C s_45_3: const #64s : i
        let s_45_3: i128 = 64;
        // D s_45_4: cast zx s_45_2 -> bv
        let s_45_4: Bits = Bits::new(s_45_2 as u128, 56u16);
        // D s_45_5: bits-cast zx s_45_4 -> bv length s_45_3
        let s_45_5: Bits = s_45_4.zero_extend(s_45_3);
        // D s_45_6: cast reint s_45_5 -> u64
        let s_45_6: u64 = (s_45_5.value() as u64);
        // D s_45_7: read-var walkparams.2:struct
        let s_45_7: bool = fn_state.walkparams._2;
        // D s_45_8: read-var walkparams.26:struct
        let s_45_8: u32 = fn_state.walkparams._26;
        // D s_45_9: read-var walkstate:struct
        let s_45_9: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_45_10: call StageOA(s_45_6, s_45_7, s_45_8, s_45_9)
        let s_45_10: ProductTypeda0231e9dc169f81 = StageOA(
            state,
            tracer,
            s_45_6,
            s_45_7,
            s_45_8,
            s_45_9,
        );
        // D s_45_11: write-var oa <= s_45_10
        fn_state.oa = s_45_10;
        // D s_45_12: read-var accdesc.1:struct
        let s_45_12: u32 = fn_state.accdesc._1;
        // C s_45_13: const #13u : u32
        let s_45_13: u32 = 13;
        // D s_45_14: cmp-eq s_45_12 s_45_13
        let s_45_14: bool = ((s_45_12) == (s_45_13));
        // N s_45_15: branch s_45_14 b87 b46
        if s_45_14 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#19601 <= s_46_0
        fn_state.gs_19601 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_47_0: read-var gs#19601:u8
        let s_47_0: bool = fn_state.gs_19601;
        // N s_47_1: branch s_47_0 b86 b48
        if s_47_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#19602 <= s_48_0
        fn_state.gs_19602 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_49_0: read-var gs#19602:u8
        let s_49_0: bool = fn_state.gs_19602;
        // N s_49_1: branch s_49_0 b85 b50
        if s_49_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_50_0: read-var accdesc.1:struct
        let s_50_0: u32 = fn_state.accdesc._1;
        // C s_50_1: const #0u : u32
        let s_50_1: u32 = 0;
        // D s_50_2: cmp-eq s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) == (s_50_1));
        // N s_50_3: branch s_50_2 b81 b51
        if s_50_2 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#19604 <= s_51_0
        fn_state.gs_19604 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_52_0: read-var gs#19604:u8
        let s_52_0: bool = fn_state.gs_19604;
        // D s_52_1: write-var gs#19605 <= s_52_0
        fn_state.gs_19605 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_53_0: read-var gs#19605:u8
        let s_53_0: bool = fn_state.gs_19605;
        // N s_53_1: branch s_53_0 b80 b54
        if s_53_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_54_0: read-var accdesc.1:struct
        let s_54_0: u32 = fn_state.accdesc._1;
        // C s_54_1: const #0u : u32
        let s_54_1: u32 = 0;
        // D s_54_2: cmp-eq s_54_0 s_54_1
        let s_54_2: bool = ((s_54_0) == (s_54_1));
        // N s_54_3: branch s_54_2 b79 b55
        if s_54_2 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#19606 <= s_55_0
        fn_state.gs_19606 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_56_0: read-var gs#19606:u8
        let s_56_0: bool = fn_state.gs_19606;
        // N s_56_1: branch s_56_0 b78 b57
        if s_56_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#19607 <= s_57_0
        fn_state.gs_19607 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_58_0: read-var gs#19607:u8
        let s_58_0: bool = fn_state.gs_19607;
        // D s_58_1: write-var gs#19608 <= s_58_0
        fn_state.gs_19608 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_59_0: read-var gs#19608:u8
        let s_59_0: bool = fn_state.gs_19608;
        // N s_59_1: branch s_59_0 b77 b60
        if s_59_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_60_0: read-var walkstate.7:struct
        let s_60_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_60_1: write-var s2_memattrs <= s_60_0
        fn_state.s2_memattrs = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_61_0: read-var accdesc.13:struct
        let s_61_0: bool = fn_state.accdesc._13;
        // N s_61_1: branch s_61_0 b76 b62
        if s_61_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#19609 <= s_62_0
        fn_state.gs_19609 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_63_0: read-var gs#19609:u8
        let s_63_0: bool = fn_state.gs_19609;
        // N s_63_1: branch s_63_0 b70 b64
        if s_63_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_64_0: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_65_0: read-var walkparams.6:struct
        let s_65_0: bool = fn_state.walkparams._6;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #0u : u8
        let s_65_2: bool = false;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // N s_65_5: branch s_65_4 b69 b66
        if s_65_4 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_66_0: read-var s2_memattrs:struct
        let s_66_0: ProductTypef170cab34335b70c = fn_state.s2_memattrs;
        // D s_66_1: write-var memattrs <= s_66_0
        fn_state.memattrs = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_67_0: read-var ipa.7:struct
        let s_67_0: u64 = fn_state.ipa._7;
        // D s_67_1: read-var oa:struct
        let s_67_1: ProductTypeda0231e9dc169f81 = fn_state.oa;
        // D s_67_2: read-var memattrs:struct
        let s_67_2: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_67_3: call CreateAddressDescriptor(s_67_0, s_67_1, s_67_2)
        let s_67_3: ProductTypece7c66ccb2cab13e = CreateAddressDescriptor(
            state,
            tracer,
            s_67_0,
            s_67_1,
            s_67_2,
        );
        // D s_67_4: write-var pa <= s_67_3
        fn_state.pa = s_67_3;
        // D s_67_5: read-var descpaddr.6:struct
        let s_67_5: ProductTypec0d0fb0603850c4c = fn_state.descpaddr._6;
        // D s_67_6: write-var pa.6 <= s_67_5
        fn_state.pa._6 = s_67_5;
        // D s_67_7: read-var s2fs1mro:u8
        let s_67_7: bool = fn_state.s2fs1mro;
        // D s_67_8: write-var pa.5 <= s_67_7
        fn_state.pa._5 = s_67_7;
        // D s_67_9: read-var pa.3:struct
        let s_67_9: ProductTypeda0231e9dc169f81 = fn_state.pa._3;
        // D s_67_10: write-var ga#14906 <= s_67_9
        fn_state.ga_14906 = s_67_9;
        // D s_67_11: read-var ga#14906.1:struct
        let s_67_11: u32 = fn_state.ga_14906._1;
        // D s_67_12: read-var descriptor:u128
        let s_67_12: u128 = fn_state.descriptor;
        // D s_67_13: cast zx s_67_12 -> bv
        let s_67_13: Bits = Bits::new(s_67_12 as u128, 128u16);
        // D s_67_14: read-var walkparams:struct
        let s_67_14: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_67_15: call AArch64_S2OutputMECID(s_67_14, s_67_11, s_67_13)
        let s_67_15: u16 = AArch64_S2OutputMECID(
            state,
            tracer,
            s_67_14,
            s_67_11,
            s_67_13,
        );
        // D s_67_16: write-var pa.1 <= s_67_15
        fn_state.pa._1 = s_67_15;
        // D s_67_17: read-var fault:struct
        let s_67_17: ProductType1d757adad216cdef = fn_state.fault;
        // D s_67_18: create-product struct = ["s_67_17", "s_67_3"]
        let s_67_18: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_67_17,
            _1: s_67_3,
        };
        // D s_67_19: write-var return_value <= s_67_18
        fn_state.return_value = s_67_18;
        // N s_67_20: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_68_0: read-var return_value:struct
        let s_68_0: ProductTypedc31059ca7e2391c = fn_state.return_value;
        // N s_68_1: return s_68_0
        return s_68_0;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_69_0: read-var ipa.2:struct
        let s_69_0: ProductTypef170cab34335b70c = fn_state.ipa._2;
        // D s_69_1: read-var s2_memattrs:struct
        let s_69_1: ProductTypef170cab34335b70c = fn_state.s2_memattrs;
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // D s_69_3: call S2CombineS1MemAttrs(s_69_0, s_69_1, s_69_2)
        let s_69_3: ProductTypef170cab34335b70c = S2CombineS1MemAttrs(
            state,
            tracer,
            s_69_0,
            s_69_1,
            s_69_2,
        );
        // D s_69_4: write-var memattrs <= s_69_3
        fn_state.memattrs = s_69_3;
        // N s_69_5: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_70_0: read-var s2_memattrs.1:struct
        let s_70_0: ProductType183e6678e5239c85 = fn_state.s2_memattrs._1;
        // D s_70_1: write-var ga#14895 <= s_70_0
        fn_state.ga_14895 = s_70_0;
        // D s_70_2: read-var ga#14895.0:struct
        let s_70_2: u8 = fn_state.ga_14895._0;
        // D s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 2u16);
        // C s_70_4: const #464u : u32
        let s_70_4: u32 = 464;
        // D s_70_5: read-reg s_70_4:u8
        let s_70_5: u8 = {
            let value = state.read_register::<u8>(s_70_4 as isize);
            tracer.read_register(s_70_4 as isize, value);
            value
        };
        // D s_70_6: cast zx s_70_5 -> bv
        let s_70_6: Bits = Bits::new(s_70_5 as u128, 2u16);
        // D s_70_7: cmp-ne s_70_3 s_70_6
        let s_70_7: bool = ((s_70_3) != (s_70_6));
        // N s_70_8: branch s_70_7 b75 b71
        if s_70_7 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_71_0: read-var s2_memattrs.4:struct
        let s_71_0: ProductType183e6678e5239c85 = fn_state.s2_memattrs._4;
        // D s_71_1: write-var ga#14897 <= s_71_0
        fn_state.ga_14897 = s_71_0;
        // D s_71_2: read-var ga#14897.0:struct
        let s_71_2: u8 = fn_state.ga_14897._0;
        // D s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 2u16);
        // C s_71_4: const #464u : u32
        let s_71_4: u32 = 464;
        // D s_71_5: read-reg s_71_4:u8
        let s_71_5: u8 = {
            let value = state.read_register::<u8>(s_71_4 as isize);
            tracer.read_register(s_71_4 as isize, value);
            value
        };
        // D s_71_6: cast zx s_71_5 -> bv
        let s_71_6: Bits = Bits::new(s_71_5 as u128, 2u16);
        // D s_71_7: cmp-ne s_71_3 s_71_6
        let s_71_7: bool = ((s_71_3) != (s_71_6));
        // D s_71_8: write-var gs#19611 <= s_71_7
        fn_state.gs_19611 = s_71_7;
        // N s_71_9: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_72_0: read-var gs#19611:u8
        let s_72_0: bool = fn_state.gs_19611;
        // N s_72_1: branch s_72_0 b74 b73
        if s_72_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_73_0: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_74_0: const #22u : u32
        let s_74_0: u32 = 22;
        // D s_74_1: write-var fault.16 <= s_74_0
        fn_state.fault._16 = s_74_0;
        // C s_74_2: const #() : ()
        let s_74_2: () = ();
        // S s_74_3: call __UNKNOWN_AddressDescriptor(s_74_2)
        let s_74_3: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_74_2,
        );
        // D s_74_4: read-var fault:struct
        let s_74_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_74_5: create-product struct = ["s_74_4", "s_74_3"]
        let s_74_5: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_74_4,
            _1: s_74_3,
        };
        // D s_74_6: write-var return_value <= s_74_5
        fn_state.return_value = s_74_5;
        // N s_74_7: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#19611 <= s_75_0
        fn_state.gs_19611 = s_75_0;
        // N s_75_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_76_0: read-var s2_memattrs.2:struct
        let s_76_0: u32 = fn_state.s2_memattrs._2;
        // C s_76_1: const #0u : u32
        let s_76_1: u32 = 0;
        // D s_76_2: cmp-eq s_76_0 s_76_1
        let s_76_2: bool = ((s_76_0) == (s_76_1));
        // D s_76_3: write-var gs#19609 <= s_76_2
        fn_state.gs_19609 = s_76_2;
        // N s_76_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_77_0: const #() : ()
        let s_77_0: () = ();
        // S s_77_1: call NormalNCMemAttr(s_77_0)
        let s_77_1: ProductTypef170cab34335b70c = NormalNCMemAttr(state, tracer, s_77_0);
        // D s_77_2: write-var s2_memattrs <= s_77_1
        fn_state.s2_memattrs = s_77_1;
        // D s_77_3: read-var walkstate.7:struct
        let s_77_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_77_4: write-var ga#14891 <= s_77_3
        fn_state.ga_14891 = s_77_3;
        // D s_77_5: read-var ga#14891.7:struct
        let s_77_5: bool = fn_state.ga_14891._7;
        // D s_77_6: write-var s2_memattrs.7 <= s_77_5
        fn_state.s2_memattrs._7 = s_77_5;
        // N s_77_7: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_78_0: const #102552u : u32
        let s_78_0: u32 = 102552;
        // D s_78_1: read-reg s_78_0:struct
        let s_78_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call _get_HCR_EL2_Type_CD(s_78_1)
        let s_78_2: bool = u_get_HCR_EL2_Type_CD(state, tracer, s_78_1);
        // D s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // C s_78_4: const #1u : u8
        let s_78_4: bool = true;
        // C s_78_5: cast zx s_78_4 -> bv
        let s_78_5: Bits = Bits::new(s_78_4 as u128, 1u16);
        // D s_78_6: cmp-eq s_78_3 s_78_5
        let s_78_6: bool = ((s_78_3) == (s_78_5));
        // D s_78_7: write-var gs#19607 <= s_78_6
        fn_state.gs_19607 = s_78_6;
        // N s_78_8: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_79_0: read-var walkstate.7:struct
        let s_79_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_79_1: write-var ga#14884 <= s_79_0
        fn_state.ga_14884 = s_79_0;
        // D s_79_2: read-var ga#14884.2:struct
        let s_79_2: u32 = fn_state.ga_14884._2;
        // C s_79_3: const #0u : u32
        let s_79_3: u32 = 0;
        // D s_79_4: cmp-eq s_79_2 s_79_3
        let s_79_4: bool = ((s_79_2) == (s_79_3));
        // D s_79_5: write-var gs#19606 <= s_79_4
        fn_state.gs_19606 = s_79_4;
        // N s_79_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_80_0: const #1u : u8
        let s_80_0: bool = true;
        // D s_80_1: write-var gs#19608 <= s_80_0
        fn_state.gs_19608 = s_80_0;
        // N s_80_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_81_0: read-var walkstate.7:struct
        let s_81_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_81_1: write-var ga#14877 <= s_81_0
        fn_state.ga_14877 = s_81_0;
        // D s_81_2: read-var ga#14877.2:struct
        let s_81_2: u32 = fn_state.ga_14877._2;
        // C s_81_3: const #1u : u32
        let s_81_3: u32 = 1;
        // D s_81_4: cmp-eq s_81_2 s_81_3
        let s_81_4: bool = ((s_81_2) == (s_81_3));
        // N s_81_5: branch s_81_4 b84 b82
        if s_81_4 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_82_0: const #102552u : u32
        let s_82_0: u32 = 102552;
        // D s_82_1: read-reg s_82_0:struct
        let s_82_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call _get_HCR_EL2_Type_ID(s_82_1)
        let s_82_2: bool = u_get_HCR_EL2_Type_ID(state, tracer, s_82_1);
        // D s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // C s_82_4: const #1u : u8
        let s_82_4: bool = true;
        // C s_82_5: cast zx s_82_4 -> bv
        let s_82_5: Bits = Bits::new(s_82_4 as u128, 1u16);
        // D s_82_6: cmp-eq s_82_3 s_82_5
        let s_82_6: bool = ((s_82_3) == (s_82_5));
        // D s_82_7: write-var gs#19603 <= s_82_6
        fn_state.gs_19603 = s_82_6;
        // N s_82_8: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_83_0: read-var gs#19603:u8
        let s_83_0: bool = fn_state.gs_19603;
        // D s_83_1: write-var gs#19604 <= s_83_0
        fn_state.gs_19604 = s_83_0;
        // N s_83_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_84_0: const #1u : u8
        let s_84_0: bool = true;
        // D s_84_1: write-var gs#19603 <= s_84_0
        fn_state.gs_19603 = s_84_0;
        // N s_84_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_85_0: const #1u : u8
        let s_85_0: bool = true;
        // D s_85_1: write-var gs#19605 <= s_85_0
        fn_state.gs_19605 = s_85_0;
        // N s_85_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_86_0: read-var walkparams.15:struct
        let s_86_0: bool = fn_state.walkparams._15;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 1u16);
        // C s_86_2: const #0u : u8
        let s_86_2: bool = false;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: write-var gs#19602 <= s_86_4
        fn_state.gs_19602 = s_86_4;
        // N s_86_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_87_0: read-var walkstate.7:struct
        let s_87_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_87_1: write-var ga#14871 <= s_87_0
        fn_state.ga_14871 = s_87_0;
        // D s_87_2: read-var ga#14871.2:struct
        let s_87_2: u32 = fn_state.ga_14871._2;
        // C s_87_3: const #1u : u32
        let s_87_3: u32 = 1;
        // D s_87_4: cmp-eq s_87_2 s_87_3
        let s_87_4: bool = ((s_87_2) == (s_87_3));
        // D s_87_5: write-var gs#19601 <= s_87_4
        fn_state.gs_19601 = s_87_4;
        // N s_87_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_88_0: const #() : ()
        let s_88_0: () = ();
        // S s_88_1: call __UNKNOWN_AddressDescriptor(s_88_0)
        let s_88_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_88_0,
        );
        // D s_88_2: read-var fault:struct
        let s_88_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_88_3: create-product struct = ["s_88_2", "s_88_1"]
        let s_88_3: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_88_2,
            _1: s_88_1,
        };
        // D s_88_4: write-var return_value <= s_88_3
        fn_state.return_value = s_88_3;
        // N s_88_5: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_89_0: const #10s : i
        let s_89_0: i128 = 10;
        // D s_89_1: read-var descriptor:u128
        let s_89_1: u128 = fn_state.descriptor;
        // D s_89_2: cast zx s_89_1 -> bv
        let s_89_2: Bits = Bits::new(s_89_1 as u128, 128u16);
        // C s_89_3: const #1u : u64
        let s_89_3: u64 = 1;
        // D s_89_4: bit-extract s_89_2 s_89_0 s_89_3
        let s_89_4: Bits = (Bits::new(
            ((s_89_2) >> (s_89_0)).value(),
            u16::try_from(s_89_3).unwrap(),
        ));
        // D s_89_5: cast reint s_89_4 -> u8
        let s_89_5: bool = ((s_89_4.value()) != 0);
        // C s_89_6: const #0s : i
        let s_89_6: i128 = 0;
        // C s_89_7: const #0u : u64
        let s_89_7: u64 = 0;
        // D s_89_8: cast zx s_89_5 -> u64
        let s_89_8: u64 = (s_89_5 as u64);
        // C s_89_9: const #1u : u64
        let s_89_9: u64 = 1;
        // D s_89_10: and s_89_8 s_89_9
        let s_89_10: u64 = ((s_89_8) & (s_89_9));
        // D s_89_11: cmp-eq s_89_10 s_89_9
        let s_89_11: bool = ((s_89_10) == (s_89_9));
        // D s_89_12: lsl s_89_8 s_89_6
        let s_89_12: u64 = s_89_8 << s_89_6;
        // D s_89_13: or s_89_7 s_89_12
        let s_89_13: u64 = ((s_89_7) | (s_89_12));
        // D s_89_14: cmpl s_89_12
        let s_89_14: u64 = !s_89_12;
        // D s_89_15: and s_89_7 s_89_14
        let s_89_15: u64 = ((s_89_7) & (s_89_14));
        // D s_89_16: select s_89_11 s_89_13 s_89_15
        let s_89_16: u64 = if s_89_11 { s_89_13 } else { s_89_15 };
        // D s_89_17: cast trunc s_89_16 -> u8
        let s_89_17: bool = ((s_89_16) != 0);
        // D s_89_18: cast zx s_89_17 -> bv
        let s_89_18: Bits = Bits::new(s_89_17 as u128, 1u16);
        // C s_89_19: const #1u : u8
        let s_89_19: bool = true;
        // C s_89_20: cast zx s_89_19 -> bv
        let s_89_20: Bits = Bits::new(s_89_19 as u128, 1u16);
        // D s_89_21: cmp-eq s_89_18 s_89_20
        let s_89_21: bool = ((s_89_18) == (s_89_20));
        // N s_89_22: branch s_89_21 b97 b90
        if s_89_21 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_90_0: read-var descpaddr.6:struct
        let s_90_0: ProductTypec0d0fb0603850c4c = fn_state.descpaddr._6;
        // D s_90_1: write-var tlbrecord.1 <= s_90_0
        fn_state.tlbrecord._1 = s_90_0;
        // D s_90_2: read-var walkstate.7:struct
        let s_90_2: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_90_3: write-var ga#14848 <= s_90_2
        fn_state.ga_14848 = s_90_2;
        // D s_90_4: read-var ga#14848.7:struct
        let s_90_4: bool = fn_state.ga_14848._7;
        // D s_90_5: write-var tlbrecord.1.14 <= s_90_4
        fn_state.tlbrecord._1._14 = s_90_4;
        // D s_90_6: read-var walkstate.6:struct
        let s_90_6: i128 = fn_state.walkstate._6;
        // D s_90_7: write-var tlbrecord.1.8 <= s_90_6
        fn_state.tlbrecord._1._8 = s_90_6;
        // C s_90_8: const #1u : u8
        let s_90_8: bool = true;
        // D s_90_9: write-var tlbrecord.1.5 <= s_90_8
        fn_state.tlbrecord._1._5 = s_90_8;
        // D s_90_10: read-var walkparams.2:struct
        let s_90_10: bool = fn_state.walkparams._2;
        // D s_90_11: cast zx s_90_10 -> bv
        let s_90_11: Bits = Bits::new(s_90_10 as u128, 1u16);
        // C s_90_12: const #1u : u8
        let s_90_12: bool = true;
        // C s_90_13: cast zx s_90_12 -> bv
        let s_90_13: Bits = Bits::new(s_90_12 as u128, 1u16);
        // D s_90_14: cmp-eq s_90_11 s_90_13
        let s_90_14: bool = ((s_90_11) == (s_90_13));
        // D s_90_15: write-var tlbrecord.1.7 <= s_90_14
        fn_state.tlbrecord._1._7 = s_90_14;
        // D s_90_16: read-var walkstate:struct
        let s_90_16: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_90_17: write-var tlbrecord.5 <= s_90_16
        fn_state.tlbrecord._5 = s_90_16;
        // D s_90_18: read-var walkparams.2:struct
        let s_90_18: bool = fn_state.walkparams._2;
        // D s_90_19: read-var walkparams.26:struct
        let s_90_19: u32 = fn_state.walkparams._26;
        // D s_90_20: read-var walkstate.6:struct
        let s_90_20: i128 = fn_state.walkstate._6;
        // D s_90_21: call TranslationSize(s_90_18, s_90_19, s_90_20)
        let s_90_21: i128 = TranslationSize(state, tracer, s_90_18, s_90_19, s_90_20);
        // D s_90_22: write-var tlbrecord.0 <= s_90_21
        fn_state.tlbrecord._0 = s_90_21;
        // D s_90_23: read-var walkstate.1:struct
        let s_90_23: bool = fn_state.walkstate._1;
        // D s_90_24: cast zx s_90_23 -> bv
        let s_90_24: Bits = Bits::new(s_90_23 as u128, 1u16);
        // C s_90_25: const #1u : u8
        let s_90_25: bool = true;
        // C s_90_26: cast zx s_90_25 -> bv
        let s_90_26: Bits = Bits::new(s_90_25 as u128, 1u16);
        // D s_90_27: cmp-eq s_90_24 s_90_26
        let s_90_27: bool = ((s_90_24) == (s_90_26));
        // N s_90_28: branch s_90_27 b96 b91
        if s_90_27 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_91_0: const #0s : i
        let s_91_0: i128 = 0;
        // D s_91_1: write-var tlbrecord.2 <= s_91_0
        fn_state.tlbrecord._2 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_92_0: read-var walkparams.2:struct
        let s_92_0: bool = fn_state.walkparams._2;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // N s_92_5: branch s_92_4 b95 b93
        if s_92_4 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_93_0: read-var tlbrecord:struct
        let s_93_0: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_93_1: write-var tlbrecord <= s_93_0
        fn_state.tlbrecord = s_93_0;
        // C s_93_2: const #64s : i
        let s_93_2: i128 = 64;
        // S s_93_3: call Zeros(s_93_2)
        let s_93_3: Bits = Zeros(state, tracer, s_93_2);
        // D s_93_4: read-var tlbrecord:struct
        let s_93_4: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_93_5: write-var tlbrecord <= s_93_4
        fn_state.tlbrecord = s_93_4;
        // N s_93_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_94_0: read-var tlbrecord:struct
        let s_94_0: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_94_1: call S2TLBCache(s_94_0)
        let s_94_1: () = S2TLBCache(state, tracer, s_94_0);
        // N s_94_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_95_0: read-var mem_desc:u128
        let s_95_0: u128 = fn_state.mem_desc;
        // D s_95_1: write-var tlbrecord.4 <= s_95_0
        fn_state.tlbrecord._4 = s_95_0;
        // N s_95_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_96_0: read-var walkparams.2:struct
        let s_96_0: bool = fn_state.walkparams._2;
        // D s_96_1: read-var walkparams.26:struct
        let s_96_1: u32 = fn_state.walkparams._26;
        // D s_96_2: read-var walkstate.6:struct
        let s_96_2: i128 = fn_state.walkstate._6;
        // D s_96_3: call ContiguousSize(s_96_0, s_96_1, s_96_2)
        let s_96_3: i128 = ContiguousSize(state, tracer, s_96_0, s_96_1, s_96_2);
        // D s_96_4: write-var tlbrecord.2 <= s_96_3
        fn_state.tlbrecord._2 = s_96_3;
        // N s_96_5: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_97_0: read-var descpaddr.6:struct
        let s_97_0: ProductTypec0d0fb0603850c4c = fn_state.descpaddr._6;
        // D s_97_1: call S2TLBLookup(s_97_0)
        let s_97_1: ProductTypeeb828c17bbe5e68 = S2TLBLookup(state, tracer, s_97_0);
        // D s_97_2: write-var tlbentry <= s_97_1
        fn_state.tlbentry = s_97_1;
        // D s_97_3: read-var tlbentry.1:struct
        let s_97_3: bool = fn_state.tlbentry._1;
        // N s_97_4: assert s_97_3
        let s_97_4: () = assert!(s_97_3);
        // D s_97_5: read-var tlbentry.0:struct
        let s_97_5: ProductTypee47dd77b186df56e = fn_state.tlbentry._0;
        // D s_97_6: write-var tlbrecord <= s_97_5
        fn_state.tlbrecord = s_97_5;
        // N s_97_7: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_98_0: const #10s : i
        let s_98_0: i128 = 10;
        // D s_98_1: read-var mem_desc:u128
        let s_98_1: u128 = fn_state.mem_desc;
        // D s_98_2: cast zx s_98_1 -> bv
        let s_98_2: Bits = Bits::new(s_98_1 as u128, 128u16);
        // C s_98_3: const #1u : u64
        let s_98_3: u64 = 1;
        // D s_98_4: bit-extract s_98_2 s_98_0 s_98_3
        let s_98_4: Bits = (Bits::new(
            ((s_98_2) >> (s_98_0)).value(),
            u16::try_from(s_98_3).unwrap(),
        ));
        // D s_98_5: cast reint s_98_4 -> u8
        let s_98_5: bool = ((s_98_4.value()) != 0);
        // C s_98_6: const #0s : i
        let s_98_6: i128 = 0;
        // C s_98_7: const #0u : u64
        let s_98_7: u64 = 0;
        // D s_98_8: cast zx s_98_5 -> u64
        let s_98_8: u64 = (s_98_5 as u64);
        // C s_98_9: const #1u : u64
        let s_98_9: u64 = 1;
        // D s_98_10: and s_98_8 s_98_9
        let s_98_10: u64 = ((s_98_8) & (s_98_9));
        // D s_98_11: cmp-eq s_98_10 s_98_9
        let s_98_11: bool = ((s_98_10) == (s_98_9));
        // D s_98_12: lsl s_98_8 s_98_6
        let s_98_12: u64 = s_98_8 << s_98_6;
        // D s_98_13: or s_98_7 s_98_12
        let s_98_13: u64 = ((s_98_7) | (s_98_12));
        // D s_98_14: cmpl s_98_12
        let s_98_14: u64 = !s_98_12;
        // D s_98_15: and s_98_7 s_98_14
        let s_98_15: u64 = ((s_98_7) & (s_98_14));
        // D s_98_16: select s_98_11 s_98_13 s_98_15
        let s_98_16: u64 = if s_98_11 { s_98_13 } else { s_98_15 };
        // D s_98_17: cast trunc s_98_16 -> u8
        let s_98_17: bool = ((s_98_16) != 0);
        // D s_98_18: cast zx s_98_17 -> bv
        let s_98_18: Bits = Bits::new(s_98_17 as u128, 1u16);
        // C s_98_19: const #1u : u8
        let s_98_19: bool = true;
        // C s_98_20: cast zx s_98_19 -> bv
        let s_98_20: Bits = Bits::new(s_98_19 as u128, 1u16);
        // D s_98_21: cmp-eq s_98_18 s_98_20
        let s_98_21: bool = ((s_98_18) == (s_98_20));
        // D s_98_22: write-var gs#19572 <= s_98_21
        fn_state.gs_19572 = s_98_21;
        // N s_98_23: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_99_0: read-var new_desc:u128
        let s_99_0: u128 = fn_state.new_desc;
        // D s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 128u16);
        // D s_99_2: read-var descriptor:u128
        let s_99_2: u128 = fn_state.descriptor;
        // D s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 128u16);
        // D s_99_4: cmp-ne s_99_1 s_99_3
        let s_99_4: bool = ((s_99_1) != (s_99_3));
        // D s_99_5: write-var gs#19569 <= s_99_4
        fn_state.gs_19569 = s_99_4;
        // N s_99_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_100_0: const #1u : u8
        let s_100_0: bool = true;
        // D s_100_1: write-var gs#19505 <= s_100_0
        fn_state.gs_19505 = s_100_0;
        // N s_100_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_101_0: read-var accdesc:struct
        let s_101_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_101_1: call CreateAccDescTTEUpdate(s_101_0)
        let s_101_1: ProductType9878976b5bcce9c9 = CreateAccDescTTEUpdate(
            state,
            tracer,
            s_101_0,
        );
        // D s_101_2: write-var descaccess <= s_101_1
        fn_state.descaccess = s_101_1;
        // C s_101_3: const #() : ()
        let s_101_3: () = ();
        // S s_101_4: call VMID_read(s_101_3)
        let s_101_4: u16 = VMID_read(state, tracer, s_101_3);
        // S s_101_5: cast zx s_101_4 -> bv
        let s_101_5: Bits = Bits::new(s_101_4 as u128, 16u16);
        // D s_101_6: create-sum enum = 1:"s_101_5"
        let s_101_6: SumType755586eec3e2b646 = SumType755586eec3e2b646::_1(s_101_5);
        // C s_101_7: const #() : ()
        let s_101_7: () = ();
        // D s_101_8: create-sum enum = 0:"s_101_7"
        let s_101_8: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_101_7);
        // D s_101_9: read-var ipa.7:struct
        let s_101_9: u64 = fn_state.ipa._7;
        // D s_101_10: read-var ipa.3:struct
        let s_101_10: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_101_11: write-var ga#14830 <= s_101_10
        fn_state.ga_14830 = s_101_10;
        // D s_101_12: read-var ga#14830.0:struct
        let s_101_12: u64 = fn_state.ga_14830._0;
        // C s_101_13: const #64s : i
        let s_101_13: i128 = 64;
        // D s_101_14: cast zx s_101_12 -> bv
        let s_101_14: Bits = Bits::new(s_101_12 as u128, 56u16);
        // D s_101_15: bits-cast zx s_101_14 -> bv length s_101_13
        let s_101_15: Bits = s_101_14.zero_extend(s_101_13);
        // D s_101_16: cast reint s_101_15 -> u64
        let s_101_16: u64 = (s_101_15.value() as u64);
        // D s_101_17: read-var walkstate.6:struct
        let s_101_17: i128 = fn_state.walkstate._6;
        // D s_101_18: create-product struct = ["s_101_16", "s_101_17"]
        let s_101_18: ProductType963c597a88a9ddbc = ProductType963c597a88a9ddbc {
            _0: s_101_16,
            _1: s_101_17,
        };
        // D s_101_19: write-var ga#14834 <= s_101_18
        fn_state.ga_14834 = s_101_18;
        // D s_101_20: read-var ga#14834.0:struct
        let s_101_20: u64 = fn_state.ga_14834._0;
        // D s_101_21: cast zx s_101_20 -> bv
        let s_101_21: Bits = Bits::new(s_101_20 as u128, 64u16);
        // D s_101_22: write-var gs#446538.0 <= s_101_21
        fn_state.gs_446538._0 = s_101_21;
        // D s_101_23: read-var ga#14834.1:struct
        let s_101_23: i128 = fn_state.ga_14834._1;
        // D s_101_24: write-var gs#446538.1 <= s_101_23
        fn_state.gs_446538._1 = s_101_23;
        // D s_101_25: read-var gs#446538:struct
        let s_101_25: ProductType1f0c48777d4d25a0 = fn_state.gs_446538;
        // D s_101_26: create-sum enum = 1:"s_101_25"
        let s_101_26: SumType3cca557f9e907281 = SumType3cca557f9e907281::_1(s_101_25);
        // C s_101_27: const #() : ()
        let s_101_27: () = ();
        // D s_101_28: create-sum enum = 0:"s_101_27"
        let s_101_28: SumTypefc0aa8a49e605a17 = SumTypefc0aa8a49e605a17::_0(s_101_27);
        // D s_101_29: read-var walkparams:struct
        let s_101_29: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_101_30: create-sum enum = 1:"s_101_29"
        let s_101_30: SumType3436044442b382d9 = SumType3436044442b382d9::_1(s_101_29);
        // D s_101_31: read-var walkstate.7:struct
        let s_101_31: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // C s_101_32: const #4u : u32
        let s_101_32: u32 = 4;
        // D s_101_33: read-var s1level:enum
        let s_101_33: SumTypebf36e919d71ba1d6 = fn_state.s1level;
        // D s_101_34: create-product struct = ["s_101_8", "s_101_31", "s_101_32", "s_101_33", "s_101_28", "s_101_26", "s_101_30", "s_101_9", "s_101_6"]
        let s_101_34: ProductTypeb525737120e184b3 = ProductTypeb525737120e184b3 {
            _0: s_101_8,
            _1: s_101_31,
            _2: s_101_32,
            _3: s_101_33,
            _4: s_101_28,
            _5: s_101_26,
            _6: s_101_30,
            _7: s_101_9,
            _8: s_101_6,
        };
        // D s_101_35: write-var translation_info <= s_101_34
        fn_state.translation_info = s_101_34;
        // D s_101_36: read-var walkparams.2:struct
        let s_101_36: bool = fn_state.walkparams._2;
        // D s_101_37: cast zx s_101_36 -> bv
        let s_101_37: Bits = Bits::new(s_101_36 as u128, 1u16);
        // C s_101_38: const #1u : u8
        let s_101_38: bool = true;
        // C s_101_39: cast zx s_101_38 -> bv
        let s_101_39: Bits = Bits::new(s_101_38 as u128, 1u16);
        // D s_101_40: cmp-eq s_101_37 s_101_39
        let s_101_40: bool = ((s_101_37) == (s_101_39));
        // N s_101_41: branch s_101_40 b103 b102
        if s_101_40 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_102_0: const #0s : i
        let s_102_0: i128 = 0;
        // D s_102_1: read-var descriptor:u128
        let s_102_1: u128 = fn_state.descriptor;
        // D s_102_2: cast zx s_102_1 -> bv
        let s_102_2: Bits = Bits::new(s_102_1 as u128, 128u16);
        // C s_102_3: const #1s : i64
        let s_102_3: i64 = 1;
        // C s_102_4: cast zx s_102_3 -> i
        let s_102_4: i128 = (i128::try_from(s_102_3).unwrap());
        // C s_102_5: const #63s : i
        let s_102_5: i128 = 63;
        // C s_102_6: add s_102_5 s_102_4
        let s_102_6: i128 = (s_102_5 + s_102_4);
        // D s_102_7: bit-extract s_102_2 s_102_0 s_102_6
        let s_102_7: Bits = (Bits::new(
            ((s_102_2) >> (s_102_0)).value(),
            u16::try_from(s_102_6).unwrap(),
        ));
        // D s_102_8: cast reint s_102_7 -> u64
        let s_102_8: u64 = (s_102_7.value() as u64);
        // C s_102_9: const #0s : i
        let s_102_9: i128 = 0;
        // D s_102_10: read-var new_desc:u128
        let s_102_10: u128 = fn_state.new_desc;
        // D s_102_11: cast zx s_102_10 -> bv
        let s_102_11: Bits = Bits::new(s_102_10 as u128, 128u16);
        // C s_102_12: const #1s : i64
        let s_102_12: i64 = 1;
        // C s_102_13: cast zx s_102_12 -> i
        let s_102_13: i128 = (i128::try_from(s_102_12).unwrap());
        // C s_102_14: const #63s : i
        let s_102_14: i128 = 63;
        // C s_102_15: add s_102_14 s_102_13
        let s_102_15: i128 = (s_102_14 + s_102_13);
        // D s_102_16: bit-extract s_102_11 s_102_9 s_102_15
        let s_102_16: Bits = (Bits::new(
            ((s_102_11) >> (s_102_9)).value(),
            u16::try_from(s_102_15).unwrap(),
        ));
        // D s_102_17: cast reint s_102_16 -> u64
        let s_102_17: u64 = (s_102_16.value() as u64);
        // D s_102_18: read-var walkparams.4:struct
        let s_102_18: bool = fn_state.walkparams._4;
        // D s_102_19: cast zx s_102_8 -> bv
        let s_102_19: Bits = Bits::new(s_102_8 as u128, 64u16);
        // D s_102_20: cast zx s_102_17 -> bv
        let s_102_20: Bits = Bits::new(s_102_17 as u128, 64u16);
        // D s_102_21: read-var fault:struct
        let s_102_21: ProductType1d757adad216cdef = fn_state.fault;
        // D s_102_22: read-var descaccess:struct
        let s_102_22: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_102_23: read-var descpaddr:struct
        let s_102_23: ProductTypece7c66ccb2cab13e = fn_state.descpaddr;
        // D s_102_24: read-var translation_info:struct
        let s_102_24: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_102_25: call AArch64_MemSwapTableDesc(s_102_21, s_102_19, s_102_20, s_102_18, s_102_22, s_102_23, s_102_24)
        let s_102_25: ProductTypeb4cea7287e2eb9d6 = AArch64_MemSwapTableDesc(
            state,
            tracer,
            s_102_21,
            s_102_19,
            s_102_20,
            s_102_18,
            s_102_22,
            s_102_23,
            s_102_24,
        );
        // D s_102_26: write-var gs#446547 <= s_102_25
        fn_state.gs_446547 = s_102_25;
        // D s_102_27: read-var gs#446547.0:struct
        let s_102_27: ProductType1d757adad216cdef = fn_state.gs_446547._0;
        // D s_102_28: read-var gs#446547.1:struct
        let s_102_28: Bits = fn_state.gs_446547._1;
        // D s_102_29: cast reint s_102_28 -> u64
        let s_102_29: u64 = (s_102_28.value() as u64);
        // D s_102_30: write-var fault <= s_102_27
        fn_state.fault = s_102_27;
        // C s_102_31: const #0s : i
        let s_102_31: i128 = 0;
        // D s_102_32: read-var mem_desc:u128
        let s_102_32: u128 = fn_state.mem_desc;
        // D s_102_33: cast zx s_102_32 -> bv
        let s_102_33: Bits = Bits::new(s_102_32 as u128, 128u16);
        // D s_102_34: cast zx s_102_29 -> bv
        let s_102_34: Bits = Bits::new(s_102_29 as u128, 64u16);
        // C s_102_35: const #63s : i
        let s_102_35: i128 = 63;
        // C s_102_36: const #1u : u64
        let s_102_36: u64 = 1;
        // C s_102_37: cast zx s_102_36 -> bv
        let s_102_37: Bits = Bits::new(s_102_36 as u128, 64u16);
        // C s_102_38: lsl s_102_37 s_102_35
        let s_102_38: Bits = s_102_37 << s_102_35;
        // C s_102_39: sub s_102_38 s_102_37
        let s_102_39: Bits = ((s_102_38) - (s_102_37));
        // D s_102_40: and s_102_34 s_102_39
        let s_102_40: Bits = ((s_102_34) & (s_102_39));
        // D s_102_41: lsl s_102_40 s_102_31
        let s_102_41: Bits = s_102_40 << s_102_31;
        // C s_102_42: lsl s_102_39 s_102_31
        let s_102_42: Bits = s_102_39 << s_102_31;
        // C s_102_43: cmpl s_102_42
        let s_102_43: Bits = !s_102_42;
        // D s_102_44: and s_102_33 s_102_43
        let s_102_44: Bits = ((s_102_33) & (s_102_43));
        // D s_102_45: or s_102_44 s_102_41
        let s_102_45: Bits = ((s_102_44) | (s_102_41));
        // D s_102_46: cast reint s_102_45 -> u128
        let s_102_46: u128 = (s_102_45.value() as u128);
        // D s_102_47: write-var mem_desc <= s_102_46
        fn_state.mem_desc = s_102_46;
        // C s_102_48: const #128s : i
        let s_102_48: i128 = 128;
        // C s_102_49: const #127s : i
        let s_102_49: i128 = 127;
        // C s_102_50: const #64s : i
        let s_102_50: i128 = 64;
        // D s_102_51: read-var mem_desc:u128
        let s_102_51: u128 = fn_state.mem_desc;
        // D s_102_52: cast zx s_102_51 -> bv
        let s_102_52: Bits = Bits::new(s_102_51 as u128, 128u16);
        // D s_102_53: call set_subrange_zeros(s_102_48, s_102_52, s_102_49, s_102_50)
        let s_102_53: Bits = set_subrange_zeros(
            state,
            tracer,
            s_102_48,
            s_102_52,
            s_102_49,
            s_102_50,
        );
        // D s_102_54: cast reint s_102_53 -> u128
        let s_102_54: u128 = (s_102_53.value() as u128);
        // D s_102_55: write-var mem_desc <= s_102_54
        fn_state.mem_desc = s_102_54;
        // N s_102_56: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_103_0: read-var walkparams.4:struct
        let s_103_0: bool = fn_state.walkparams._4;
        // D s_103_1: read-var descriptor:u128
        let s_103_1: u128 = fn_state.descriptor;
        // D s_103_2: cast zx s_103_1 -> bv
        let s_103_2: Bits = Bits::new(s_103_1 as u128, 128u16);
        // D s_103_3: read-var new_desc:u128
        let s_103_3: u128 = fn_state.new_desc;
        // D s_103_4: cast zx s_103_3 -> bv
        let s_103_4: Bits = Bits::new(s_103_3 as u128, 128u16);
        // D s_103_5: read-var fault:struct
        let s_103_5: ProductType1d757adad216cdef = fn_state.fault;
        // D s_103_6: read-var descaccess:struct
        let s_103_6: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_103_7: read-var descpaddr:struct
        let s_103_7: ProductTypece7c66ccb2cab13e = fn_state.descpaddr;
        // D s_103_8: read-var translation_info:struct
        let s_103_8: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_103_9: call AArch64_MemSwapTableDesc(s_103_5, s_103_2, s_103_4, s_103_0, s_103_6, s_103_7, s_103_8)
        let s_103_9: ProductTypeb4cea7287e2eb9d6 = AArch64_MemSwapTableDesc(
            state,
            tracer,
            s_103_5,
            s_103_2,
            s_103_4,
            s_103_0,
            s_103_6,
            s_103_7,
            s_103_8,
        );
        // D s_103_10: write-var gs#446555 <= s_103_9
        fn_state.gs_446555 = s_103_9;
        // D s_103_11: read-var gs#446555.0:struct
        let s_103_11: ProductType1d757adad216cdef = fn_state.gs_446555._0;
        // D s_103_12: read-var gs#446555.1:struct
        let s_103_12: Bits = fn_state.gs_446555._1;
        // D s_103_13: cast reint s_103_12 -> u128
        let s_103_13: u128 = (s_103_12.value() as u128);
        // D s_103_14: write-var fault <= s_103_11
        fn_state.fault = s_103_11;
        // D s_103_15: write-var mem_desc <= s_103_13
        fn_state.mem_desc = s_103_13;
        // N s_103_16: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_104_0: const #1u : u8
        let s_104_0: bool = true;
        // S s_104_1: call Bit(s_104_0)
        let s_104_1: bool = Bit(state, tracer, s_104_0);
        // C s_104_2: const #7s : i
        let s_104_2: i128 = 7;
        // D s_104_3: read-var new_desc:u128
        let s_104_3: u128 = fn_state.new_desc;
        // D s_104_4: cast zx s_104_3 -> bv
        let s_104_4: Bits = Bits::new(s_104_3 as u128, 128u16);
        // C s_104_5: const #1u : u64
        let s_104_5: u64 = 1;
        // D s_104_6: bit-insert s_104_4 s_104_4 s_104_2 s_104_5
        let s_104_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_104_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_104_4.length(),
            );
            (s_104_4 & mask) | (s_104_4 << s_104_2)
        };
        // D s_104_7: cast reint s_104_6 -> u128
        let s_104_7: u128 = (s_104_6.value() as u128);
        // D s_104_8: write-var new_desc <= s_104_7
        fn_state.new_desc = s_104_7;
        // N s_104_9: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_105_0: read-var accdesc.1:struct
        let s_105_0: u32 = fn_state.accdesc._1;
        // C s_105_1: const #8u : u32
        let s_105_1: u32 = 8;
        // D s_105_2: cmp-eq s_105_0 s_105_1
        let s_105_2: bool = ((s_105_0) == (s_105_1));
        // N s_105_3: branch s_105_2 b111 b106
        if s_105_2 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_106_0: read-var accdesc.1:struct
        let s_106_0: u32 = fn_state.accdesc._1;
        // C s_106_1: const #5u : u32
        let s_106_1: u32 = 5;
        // D s_106_2: cmp-eq s_106_0 s_106_1
        let s_106_2: bool = ((s_106_0) == (s_106_1));
        // N s_106_3: branch s_106_2 b110 b107
        if s_106_2 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_107_0: read-var accdesc.1:struct
        let s_107_0: u32 = fn_state.accdesc._1;
        // C s_107_1: const #6u : u32
        let s_107_1: u32 = 6;
        // D s_107_2: cmp-eq s_107_0 s_107_1
        let s_107_2: bool = ((s_107_0) == (s_107_1));
        // D s_107_3: write-var gs#19514 <= s_107_2
        fn_state.gs_19514 = s_107_2;
        // N s_107_4: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_108_0: read-var gs#19514:u8
        let s_108_0: bool = fn_state.gs_19514;
        // D s_108_1: write-var gs#19515 <= s_108_0
        fn_state.gs_19515 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_109_0: read-var gs#19515:u8
        let s_109_0: bool = fn_state.gs_19515;
        // D s_109_1: not s_109_0
        let s_109_1: bool = !s_109_0;
        // D s_109_2: write-var gs#19516 <= s_109_1
        fn_state.gs_19516 = s_109_1;
        // N s_109_3: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_110_0: const #1u : u8
        let s_110_0: bool = true;
        // D s_110_1: write-var gs#19514 <= s_110_0
        fn_state.gs_19514 = s_110_0;
        // N s_110_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_111_0: const #1u : u8
        let s_111_0: bool = true;
        // D s_111_1: write-var gs#19515 <= s_111_0
        fn_state.gs_19515 = s_111_0;
        // N s_111_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_112_0: read-var accdesc.32:struct
        let s_112_0: bool = fn_state.accdesc._32;
        // D s_112_1: write-var gs#19513 <= s_112_0
        fn_state.gs_19513 = s_112_0;
        // N s_112_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_113_0: read-var walkparams.17:struct
        let s_113_0: bool = fn_state.walkparams._17;
        // D s_113_1: cast zx s_113_0 -> bv
        let s_113_1: Bits = Bits::new(s_113_0 as u128, 1u16);
        // C s_113_2: const #1u : u8
        let s_113_2: bool = true;
        // C s_113_3: cast zx s_113_2 -> bv
        let s_113_3: Bits = Bits::new(s_113_2 as u128, 1u16);
        // D s_113_4: cmp-eq s_113_1 s_113_3
        let s_113_4: bool = ((s_113_1) == (s_113_3));
        // N s_113_5: branch s_113_4 b116 b114
        if s_113_4 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_114_0: const #51s : i
        let s_114_0: i128 = 51;
        // D s_114_1: read-var descriptor:u128
        let s_114_1: u128 = fn_state.descriptor;
        // D s_114_2: cast zx s_114_1 -> bv
        let s_114_2: Bits = Bits::new(s_114_1 as u128, 128u16);
        // C s_114_3: const #1u : u64
        let s_114_3: u64 = 1;
        // D s_114_4: bit-extract s_114_2 s_114_0 s_114_3
        let s_114_4: Bits = (Bits::new(
            ((s_114_2) >> (s_114_0)).value(),
            u16::try_from(s_114_3).unwrap(),
        ));
        // D s_114_5: cast reint s_114_4 -> u8
        let s_114_5: bool = ((s_114_4.value()) != 0);
        // C s_114_6: const #0s : i
        let s_114_6: i128 = 0;
        // C s_114_7: const #0u : u64
        let s_114_7: u64 = 0;
        // D s_114_8: cast zx s_114_5 -> u64
        let s_114_8: u64 = (s_114_5 as u64);
        // C s_114_9: const #1u : u64
        let s_114_9: u64 = 1;
        // D s_114_10: and s_114_8 s_114_9
        let s_114_10: u64 = ((s_114_8) & (s_114_9));
        // D s_114_11: cmp-eq s_114_10 s_114_9
        let s_114_11: bool = ((s_114_10) == (s_114_9));
        // D s_114_12: lsl s_114_8 s_114_6
        let s_114_12: u64 = s_114_8 << s_114_6;
        // D s_114_13: or s_114_7 s_114_12
        let s_114_13: u64 = ((s_114_7) | (s_114_12));
        // D s_114_14: cmpl s_114_12
        let s_114_14: u64 = !s_114_12;
        // D s_114_15: and s_114_7 s_114_14
        let s_114_15: u64 = ((s_114_7) & (s_114_14));
        // D s_114_16: select s_114_11 s_114_13 s_114_15
        let s_114_16: u64 = if s_114_11 { s_114_13 } else { s_114_15 };
        // D s_114_17: cast trunc s_114_16 -> u8
        let s_114_17: bool = ((s_114_16) != 0);
        // D s_114_18: cast zx s_114_17 -> bv
        let s_114_18: Bits = Bits::new(s_114_17 as u128, 1u16);
        // C s_114_19: const #1u : u8
        let s_114_19: bool = true;
        // C s_114_20: cast zx s_114_19 -> bv
        let s_114_20: Bits = Bits::new(s_114_19 as u128, 1u16);
        // D s_114_21: cmp-eq s_114_18 s_114_20
        let s_114_21: bool = ((s_114_18) == (s_114_20));
        // D s_114_22: write-var gs#19511 <= s_114_21
        fn_state.gs_19511 = s_114_21;
        // N s_114_23: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_115_0: read-var gs#19511:u8
        let s_115_0: bool = fn_state.gs_19511;
        // D s_115_1: write-var gs#19512 <= s_115_0
        fn_state.gs_19512 = s_115_0;
        // N s_115_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_116_0: const #1u : u8
        let s_116_0: bool = true;
        // D s_116_1: write-var gs#19511 <= s_116_0
        fn_state.gs_19511 = s_116_0;
        // N s_116_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_117_0: read-var walkparams.9:struct
        let s_117_0: bool = fn_state.walkparams._9;
        // D s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 1u16);
        // C s_117_2: const #1u : u8
        let s_117_2: bool = true;
        // C s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // D s_117_4: cmp-eq s_117_1 s_117_3
        let s_117_4: bool = ((s_117_1) == (s_117_3));
        // D s_117_5: write-var gs#19508 <= s_117_4
        fn_state.gs_19508 = s_117_4;
        // N s_117_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_118_0: read-var walkparams.7:struct
        let s_118_0: bool = fn_state.walkparams._7;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 1u16);
        // C s_118_2: const #1u : u8
        let s_118_2: bool = true;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // D s_118_5: write-var gs#19507 <= s_118_4
        fn_state.gs_19507 = s_118_4;
        // N s_118_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_119_0: const #1u : u8
        let s_119_0: bool = true;
        // S s_119_1: call Bit(s_119_0)
        let s_119_1: bool = Bit(state, tracer, s_119_0);
        // C s_119_2: const #10s : i
        let s_119_2: i128 = 10;
        // D s_119_3: read-var new_desc:u128
        let s_119_3: u128 = fn_state.new_desc;
        // D s_119_4: cast zx s_119_3 -> bv
        let s_119_4: Bits = Bits::new(s_119_3 as u128, 128u16);
        // C s_119_5: const #1u : u64
        let s_119_5: u64 = 1;
        // D s_119_6: bit-insert s_119_4 s_119_4 s_119_2 s_119_5
        let s_119_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_119_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_119_4.length(),
            );
            (s_119_4 & mask) | (s_119_4 << s_119_2)
        };
        // D s_119_7: cast reint s_119_6 -> u128
        let s_119_7: u128 = (s_119_6.value() as u128);
        // D s_119_8: write-var new_desc <= s_119_7
        fn_state.new_desc = s_119_7;
        // N s_119_9: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_120_0: read-var fault:struct
        let s_120_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_120_1: call AArch64_SettingAccessFlagPermitted(s_120_0)
        let s_120_1: bool = AArch64_SettingAccessFlagPermitted(state, tracer, s_120_0);
        // D s_120_2: write-var gs#19506 <= s_120_1
        fn_state.gs_19506 = s_120_1;
        // N s_120_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_121_0: read-var fault:struct
        let s_121_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_121_1: read-var walkstate:struct
        let s_121_1: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_121_2: read-var walkparams:struct
        let s_121_2: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_121_3: read-var ipa:struct
        let s_121_3: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_121_4: read-var accdesc:struct
        let s_121_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_121_5: call AArch64_S2CheckPermissions(s_121_0, s_121_1, s_121_2, s_121_3, s_121_4)
        let s_121_5: ProductType3b8bd97143a1dd5c = AArch64_S2CheckPermissions(
            state,
            tracer,
            s_121_0,
            s_121_1,
            s_121_2,
            s_121_3,
            s_121_4,
        );
        // D s_121_6: write-var ga#14793 <= s_121_5
        fn_state.ga_14793 = s_121_5;
        // D s_121_7: read-var ga#14793.0:struct
        let s_121_7: ProductType1d757adad216cdef = fn_state.ga_14793._0;
        // D s_121_8: read-var ga#14793.1:struct
        let s_121_8: bool = fn_state.ga_14793._1;
        // D s_121_9: write-var fault <= s_121_7
        fn_state.fault = s_121_7;
        // D s_121_10: write-var s2fs1mro <= s_121_8
        fn_state.s2fs1mro = s_121_8;
        // N s_121_11: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_122_0: const #2u : u32
        let s_122_0: u32 = 2;
        // D s_122_1: write-var fault.16 <= s_122_0
        fn_state.fault._16 = s_122_0;
        // N s_122_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_123_0: const #() : ()
        let s_123_0: () = ();
        // S s_123_1: call __UNKNOWN_AddressDescriptor(s_123_0)
        let s_123_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_123_0,
        );
        // D s_123_2: read-var fault:struct
        let s_123_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_123_3: create-product struct = ["s_123_2", "s_123_1"]
        let s_123_3: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_123_2,
            _1: s_123_1,
        };
        // D s_123_4: write-var return_value <= s_123_3
        fn_state.return_value = s_123_3;
        // N s_123_5: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_124_0: const #128s : i64
        let s_124_0: i64 = 128;
        // D s_124_1: read-var fault:struct
        let s_124_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_124_2: read-var ipa:struct
        let s_124_2: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_124_3: read-var walkparams:struct
        let s_124_3: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_124_4: read-var accdesc:struct
        let s_124_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_124_5: read-var s1level:enum
        let s_124_5: SumTypebf36e919d71ba1d6 = fn_state.s1level;
        // D s_124_6: call AArch64_S2Walk(s_124_1, s_124_2, s_124_3, s_124_4, s_124_5, s_124_0)
        let s_124_6: ProductType4b99944cd5e0b59d = AArch64_S2Walk(
            state,
            tracer,
            s_124_1,
            s_124_2,
            s_124_3,
            s_124_4,
            s_124_5,
            s_124_0,
        );
        // D s_124_7: write-var gs#446519 <= s_124_6
        fn_state.gs_446519 = s_124_6;
        // D s_124_8: read-var gs#446519.0:struct
        let s_124_8: ProductType1d757adad216cdef = fn_state.gs_446519._0;
        // D s_124_9: read-var gs#446519.1:struct
        let s_124_9: ProductTypece7c66ccb2cab13e = fn_state.gs_446519._1;
        // D s_124_10: read-var gs#446519.2:struct
        let s_124_10: ProductType96e7acababe246a1 = fn_state.gs_446519._2;
        // D s_124_11: read-var gs#446519.3:struct
        let s_124_11: Bits = fn_state.gs_446519._3;
        // D s_124_12: cast reint s_124_11 -> u128
        let s_124_12: u128 = (s_124_11.value() as u128);
        // D s_124_13: write-var fault <= s_124_8
        fn_state.fault = s_124_8;
        // D s_124_14: write-var descpaddr <= s_124_9
        fn_state.descpaddr = s_124_9;
        // D s_124_15: write-var walkstate <= s_124_10
        fn_state.walkstate = s_124_10;
        // D s_124_16: write-var descriptor <= s_124_12
        fn_state.descriptor = s_124_12;
        // N s_124_17: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_125_0: const #6u : u32
        let s_125_0: u32 = 6;
        // D s_125_1: write-var fault.16 <= s_125_0
        fn_state.fault._16 = s_125_0;
        // C s_125_2: const #0s : i
        let s_125_2: i128 = 0;
        // D s_125_3: write-var fault.9 <= s_125_2
        fn_state.fault._9 = s_125_2;
        // C s_125_4: const #() : ()
        let s_125_4: () = ();
        // S s_125_5: call __UNKNOWN_AddressDescriptor(s_125_4)
        let s_125_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_125_4,
        );
        // D s_125_6: read-var fault:struct
        let s_125_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_125_7: create-product struct = ["s_125_6", "s_125_5"]
        let s_125_7: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_125_6,
            _1: s_125_5,
        };
        // D s_125_8: write-var return_value <= s_125_7
        fn_state.return_value = s_125_7;
        // N s_125_9: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_126_0: const #6u : u32
        let s_126_0: u32 = 6;
        // D s_126_1: write-var fault.16 <= s_126_0
        fn_state.fault._16 = s_126_0;
        // C s_126_2: const #0s : i
        let s_126_2: i128 = 0;
        // D s_126_3: write-var fault.9 <= s_126_2
        fn_state.fault._9 = s_126_2;
        // C s_126_4: const #() : ()
        let s_126_4: () = ();
        // S s_126_5: call __UNKNOWN_AddressDescriptor(s_126_4)
        let s_126_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_126_4,
        );
        // D s_126_6: read-var fault:struct
        let s_126_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_126_7: create-product struct = ["s_126_6", "s_126_5"]
        let s_126_7: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_126_6,
            _1: s_126_5,
        };
        // D s_126_8: write-var return_value <= s_126_7
        fn_state.return_value = s_126_7;
        // N s_126_9: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_127_0: read-var walkparams:struct
        let s_127_0: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_127_1: call AArch64_S2InvalidSL(s_127_0)
        let s_127_1: bool = AArch64_S2InvalidSL(state, tracer, s_127_0);
        // N s_127_2: branch s_127_1 b130 b128
        if s_127_1 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_128_0: read-var walkparams:struct
        let s_128_0: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_128_1: call AArch64_S2InconsistentSL(s_128_0)
        let s_128_1: bool = AArch64_S2InconsistentSL(state, tracer, s_128_0);
        // D s_128_2: write-var gs#19484 <= s_128_1
        fn_state.gs_19484 = s_128_1;
        // N s_128_3: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_129_0: read-var gs#19484:u8
        let s_129_0: bool = fn_state.gs_19484;
        // D s_129_1: write-var gs#19485 <= s_129_0
        fn_state.gs_19485 = s_129_0;
        // N s_129_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_130_0: const #1u : u8
        let s_130_0: bool = true;
        // D s_130_1: write-var gs#19484 <= s_130_0
        fn_state.gs_19484 = s_130_0;
        // N s_130_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_131_0: const #5s : i
        let s_131_0: i128 = 5;
        // C s_131_1: const #0s : i
        let s_131_1: i128 = 0;
        // D s_131_2: read-var s2maxtxsz:i
        let s_131_2: i128 = fn_state.s2maxtxsz;
        // D s_131_3: call integer_subrange(s_131_2, s_131_0, s_131_1)
        let s_131_3: Bits = integer_subrange(state, tracer, s_131_2, s_131_0, s_131_1);
        // D s_131_4: cast reint s_131_3 -> u8
        let s_131_4: u8 = (s_131_3.value() as u8);
        // D s_131_5: write-var walkparams.29 <= s_131_4
        fn_state.walkparams._29 = s_131_4;
        // N s_131_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_132_0: const #5s : i
        let s_132_0: i128 = 5;
        // C s_132_1: const #0s : i
        let s_132_1: i128 = 0;
        // D s_132_2: read-var s2mintxsz:i
        let s_132_2: i128 = fn_state.s2mintxsz;
        // D s_132_3: call integer_subrange(s_132_2, s_132_0, s_132_1)
        let s_132_3: Bits = integer_subrange(state, tracer, s_132_2, s_132_0, s_132_1);
        // D s_132_4: cast reint s_132_3 -> u8
        let s_132_4: u8 = (s_132_3.value() as u8);
        // D s_132_5: write-var walkparams.29 <= s_132_4
        fn_state.walkparams._29 = s_132_4;
        // N s_132_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_133_0: const #6u : u32
        let s_133_0: u32 = 6;
        // D s_133_1: write-var fault.16 <= s_133_0
        fn_state.fault._16 = s_133_0;
        // C s_133_2: const #0s : i
        let s_133_2: i128 = 0;
        // D s_133_3: write-var fault.9 <= s_133_2
        fn_state.fault._9 = s_133_2;
        // C s_133_4: const #() : ()
        let s_133_4: () = ();
        // S s_133_5: call __UNKNOWN_AddressDescriptor(s_133_4)
        let s_133_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_133_4,
        );
        // D s_133_6: read-var fault:struct
        let s_133_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_133_7: create-product struct = ["s_133_6", "s_133_5"]
        let s_133_7: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_133_6,
            _1: s_133_5,
        };
        // D s_133_8: write-var return_value <= s_133_7
        fn_state.return_value = s_133_7;
        // N s_133_9: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_134_0: read-var fault:struct
        let s_134_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_134_1: read-var ipa:struct
        let s_134_1: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_134_2: create-product struct = ["s_134_0", "s_134_1"]
        let s_134_2: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_134_0,
            _1: s_134_1,
        };
        // D s_134_3: write-var return_value <= s_134_2
        fn_state.return_value = s_134_2;
        // N s_134_4: jump b68
        return block_68(state, tracer, fn_state);
    }
}
