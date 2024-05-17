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
use AArch64_GetS1TTWParams::*;
use AArch64_S1Enabled::*;
use AArch64_S1DCacheEnabled::*;
use AArch64_MaxTxSZ::*;
use AArch64_S1Walk::*;
use AArch64_GetVARange::*;
use Zeros::*;
use HaveTME::*;
use AArch64_S1MinTxSZ::*;
use AArch64_S1OutputMECID::*;
use S1TLBCache::*;
use set_subrange_zeros::*;
use AArch64_S2Translate::*;
use HaveSVE::*;
use NormalNCMemAttr::*;
use u_get_HCR_EL2_Type_VM::*;
use AArch64_VAIsOutOfRange::*;
use TranslationSize::*;
use EL2Enabled::*;
use CreateAccDescTTEUpdate::*;
use AArch64_S1HasAlignmentFault::*;
use StageOA::*;
use HaveMTE2Ext::*;
use AArch64_MemSwapTableDesc::*;
use ConstrainUnpredictableBool::*;
use S1TLBLookup::*;
use AArch64_SettingAccessFlagPermitted::*;
use AArch64_SettingDirtyStatePermitted::*;
use SetInGuardedPage::*;
use u__IMPDEF_boolean::*;
use EffectiveShareability::*;
use CreateAddressDescriptor::*;
use AArch64_S1CheckPermissions::*;
use ContiguousSize::*;
use AArch64_S1DisabledOutput::*;
use u__UNKNOWN_AddressDescriptor::*;
use integer_subrange::*;
use Bit::*;
use AArch64_S1TxSZFaults::*;
use AArch64_S1ICacheEnabled::*;
use common::*;
pub fn AArch64_S1Translate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    regime: u32,
    va: u64,
    aligned: bool,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductTypedc31059ca7e2391c {
    #[derive(Default)]
    struct FunctionState {
        ga_15365: ProductTypef170cab34335b70c,
        ipa: ProductTypece7c66ccb2cab13e,
        gs_19974: bool,
        gs_19862: bool,
        gs_446696: ProductType4b99944cd5e0b59d,
        gs_446722: ProductTypeb4cea7287e2eb9d6,
        gs_19821: bool,
        descaccess: ProductType9878976b5bcce9c9,
        translation_info: ProductTypeb525737120e184b3,
        gs_19977: bool,
        ga_15383: ProductType183e6678e5239c85,
        gs_19869: bool,
        ga_15364: ProductTypef170cab34335b70c,
        new_desc: u128,
        gs_19964: bool,
        tlbrecord: ProductTypee47dd77b186df56e,
        gs_19968: bool,
        gs_19859: bool,
        gs_446688: ProductType4b99944cd5e0b59d,
        ga_15381: ProductType183e6678e5239c85,
        gs_19868: bool,
        gs_19827: bool,
        ga_15389: ProductTypeda0231e9dc169f81,
        s1mintxsz: i128,
        gs_19820: bool,
        gs_19963: bool,
        gs_19824: bool,
        mem_desc: u128,
        gs_19866: bool,
        gs_19930: bool,
        gs_446730: ProductTypeb4cea7287e2eb9d6,
        gs_19861: bool,
        gs_19933: bool,
        descriptor: u128,
        gs_19819: bool,
        gs_19975: bool,
        s1maxtxsz: i128,
        gs_19867: bool,
        ga_15356: ProductTypef170cab34335b70c,
        gs_19826: bool,
        ga_15360: ProductTypef170cab34335b70c,
        fault: ProductType1d757adad216cdef,
        s2fault: ProductType1d757adad216cdef,
        gs_19865: bool,
        tlbentryshadow_323: ProductTypeeb828c17bbe5e68,
        gs_19965: bool,
        ga_15297: ProductTypedc31059ca7e2391c,
        gs_19860: bool,
        oa: ProductTypeda0231e9dc169f81,
        memattrs: ProductTypef170cab34335b70c,
        gs_19828: bool,
        ga_15327: ProductTypef170cab34335b70c,
        descipaddr: ProductTypece7c66ccb2cab13e,
        return_value: ProductTypedc31059ca7e2391c,
        gs_19870: bool,
        gs_19823: bool,
        gs_19976: bool,
        ga_15350: ProductTypef170cab34335b70c,
        gs_19966: bool,
        descpaddr: ProductTypece7c66ccb2cab13e,
        walkstate: ProductType96e7acababe246a1,
        gs_19829: bool,
        ga_15377: ProductTypef170cab34335b70c,
        gs_19825: bool,
        walkparams: ProductTypeef284266e139aee2,
        gs_19962: bool,
        gs_19872: bool,
        gs_19822: bool,
        gs_19961: bool,
        fault_in: ProductType1d757adad216cdef,
        regime: u32,
        va: u64,
        aligned: bool,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        fault_in,
        regime,
        va,
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
        // D s_0_0: read-var fault_in:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_1: write-var fault <= s_0_0
        fn_state.fault = s_0_0;
        // C s_0_2: const #0u : u8
        let s_0_2: bool = false;
        // D s_0_3: write-var fault.15 <= s_0_2
        fn_state.fault._15 = s_0_2;
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // D s_0_5: write-var fault.14 <= s_0_4
        fn_state.fault._14 = s_0_4;
        // D s_0_6: read-var accdesc.1:struct
        let s_0_6: u32 = fn_state.accdesc._1;
        // D s_0_7: read-var regime:u32
        let s_0_7: u32 = fn_state.regime;
        // D s_0_8: call AArch64_S1Enabled(s_0_7, s_0_6)
        let s_0_8: bool = AArch64_S1Enabled(state, tracer, s_0_7, s_0_6);
        // D s_0_9: not s_0_8
        let s_0_9: bool = !s_0_8;
        // N s_0_10: branch s_0_9 b184 b1
        if s_0_9 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_1_0: read-var accdesc.25:struct
        let s_1_0: u32 = fn_state.accdesc._25;
        // D s_1_1: read-var regime:u32
        let s_1_1: u32 = fn_state.regime;
        // D s_1_2: read-var va:u64
        let s_1_2: u64 = fn_state.va;
        // D s_1_3: call AArch64_GetS1TTWParams(s_1_1, s_1_0, s_1_2)
        let s_1_3: ProductTypeef284266e139aee2 = AArch64_GetS1TTWParams(
            state,
            tracer,
            s_1_1,
            s_1_0,
            s_1_2,
        );
        // D s_1_4: write-var walkparams <= s_1_3
        fn_state.walkparams = s_1_3;
        // D s_1_5: read-var walkparams.3:struct
        let s_1_5: bool = fn_state.walkparams._3;
        // D s_1_6: read-var walkparams.7:struct
        let s_1_6: bool = fn_state.walkparams._7;
        // D s_1_7: read-var walkparams.36:struct
        let s_1_7: u32 = fn_state.walkparams._36;
        // D s_1_8: read-var regime:u32
        let s_1_8: u32 = fn_state.regime;
        // D s_1_9: call AArch64_S1MinTxSZ(s_1_8, s_1_5, s_1_6, s_1_7)
        let s_1_9: i128 = AArch64_S1MinTxSZ(state, tracer, s_1_8, s_1_5, s_1_6, s_1_7);
        // D s_1_10: write-var s1mintxsz <= s_1_9
        fn_state.s1mintxsz = s_1_9;
        // D s_1_11: read-var walkparams.36:struct
        let s_1_11: u32 = fn_state.walkparams._36;
        // D s_1_12: call AArch64_MaxTxSZ(s_1_11)
        let s_1_12: i128 = AArch64_MaxTxSZ(state, tracer, s_1_11);
        // D s_1_13: write-var s1maxtxsz <= s_1_12
        fn_state.s1maxtxsz = s_1_12;
        // D s_1_14: read-var regime:u32
        let s_1_14: u32 = fn_state.regime;
        // D s_1_15: read-var walkparams:struct
        let s_1_15: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_1_16: call AArch64_S1TxSZFaults(s_1_14, s_1_15)
        let s_1_16: bool = AArch64_S1TxSZFaults(state, tracer, s_1_14, s_1_15);
        // N s_1_17: branch s_1_16 b183 b2
        if s_1_16 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_2_0: read-var walkparams.37:struct
        let s_2_0: u8 = fn_state.walkparams._37;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 6u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: read-var s1mintxsz:i
        let s_2_5: i128 = fn_state.s1mintxsz;
        // D s_2_6: cmp-lt s_2_4 s_2_5
        let s_2_6: bool = ((s_2_4) < (s_2_5));
        // N s_2_7: branch s_2_6 b182 b3
        if s_2_6 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_3_0: read-var walkparams.37:struct
        let s_3_0: u8 = fn_state.walkparams._37;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 6u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var s1maxtxsz:i
        let s_3_5: i128 = fn_state.s1maxtxsz;
        // D s_3_6: cmp-gt s_3_4 s_3_5
        let s_3_6: bool = ((s_3_4) > (s_3_5));
        // N s_3_7: branch s_3_6 b181 b4
        if s_3_6 {
            return block_181(state, tracer, fn_state);
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
        // D s_5_0: read-var accdesc.1:struct
        let s_5_0: u32 = fn_state.accdesc._1;
        // D s_5_1: read-var va:u64
        let s_5_1: u64 = fn_state.va;
        // D s_5_2: read-var regime:u32
        let s_5_2: u32 = fn_state.regime;
        // D s_5_3: read-var walkparams:struct
        let s_5_3: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_5_4: call AArch64_VAIsOutOfRange(s_5_1, s_5_0, s_5_2, s_5_3)
        let s_5_4: bool = AArch64_VAIsOutOfRange(
            state,
            tracer,
            s_5_1,
            s_5_0,
            s_5_2,
            s_5_3,
        );
        // N s_5_5: branch s_5_4 b180 b6
        if s_5_4 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_6_0: read-var accdesc.8:struct
        let s_6_0: u8 = fn_state.accdesc._8;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #448u : u32
        let s_6_2: u32 = 448;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b179 b7
        if s_6_5 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#19819 <= s_7_0
        fn_state.gs_19819 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_8_0: read-var gs#19819:u8
        let s_8_0: bool = fn_state.gs_19819;
        // N s_8_1: branch s_8_0 b178 b9
        if s_8_0 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call HaveTME(s_9_0)
        let s_9_1: bool = HaveTME(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b177 b10
        if s_9_1 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#19820 <= s_10_0
        fn_state.gs_19820 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_11_0: read-var gs#19820:u8
        let s_11_0: bool = fn_state.gs_19820;
        // N s_11_1: branch s_11_0 b176 b12
        if s_11_0 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#19821 <= s_12_0
        fn_state.gs_19821 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_13_0: read-var gs#19821:u8
        let s_13_0: bool = fn_state.gs_19821;
        // N s_13_1: branch s_13_0 b175 b14
        if s_13_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#19822 <= s_14_0
        fn_state.gs_19822 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_15_0: read-var gs#19822:u8
        let s_15_0: bool = fn_state.gs_19822;
        // N s_15_1: branch s_15_0 b174 b16
        if s_15_0 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call HaveSVE(s_16_0)
        let s_16_1: bool = HaveSVE(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b173 b17
        if s_16_1 {
            return block_173(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#19823 <= s_17_0
        fn_state.gs_19823 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_18_0: read-var gs#19823:u8
        let s_18_0: bool = fn_state.gs_19823;
        // N s_18_1: branch s_18_0 b172 b19
        if s_18_0 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#19824 <= s_19_0
        fn_state.gs_19824 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_20_0: read-var gs#19824:u8
        let s_20_0: bool = fn_state.gs_19824;
        // N s_20_1: branch s_20_0 b159 b21
        if s_20_0 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#19829 <= s_21_0
        fn_state.gs_19829 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_22_0: read-var gs#19829:u8
        let s_22_0: bool = fn_state.gs_19829;
        // N s_22_1: branch s_22_0 b158 b23
        if s_22_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_24_0: read-var walkparams.3:struct
        let s_24_0: bool = fn_state.walkparams._3;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // N s_24_5: branch s_24_4 b157 b25
        if s_24_4 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_25_0: const #64s : i64
        let s_25_0: i64 = 64;
        // C s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: read-var fault:struct
        let s_25_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_25_3: read-var walkparams:struct
        let s_25_3: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_25_4: read-var va:u64
        let s_25_4: u64 = fn_state.va;
        // D s_25_5: read-var regime:u32
        let s_25_5: u32 = fn_state.regime;
        // D s_25_6: read-var accdesc:struct
        let s_25_6: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_25_7: call AArch64_S1Walk(s_25_2, s_25_3, s_25_4, s_25_5, s_25_6, s_25_1)
        let s_25_7: ProductType4b99944cd5e0b59d = AArch64_S1Walk(
            state,
            tracer,
            s_25_2,
            s_25_3,
            s_25_4,
            s_25_5,
            s_25_6,
            s_25_1,
        );
        // D s_25_8: write-var gs#446688 <= s_25_7
        fn_state.gs_446688 = s_25_7;
        // D s_25_9: read-var gs#446688.0:struct
        let s_25_9: ProductType1d757adad216cdef = fn_state.gs_446688._0;
        // D s_25_10: read-var gs#446688.1:struct
        let s_25_10: ProductTypece7c66ccb2cab13e = fn_state.gs_446688._1;
        // D s_25_11: read-var gs#446688.2:struct
        let s_25_11: ProductType96e7acababe246a1 = fn_state.gs_446688._2;
        // D s_25_12: read-var gs#446688.3:struct
        let s_25_12: Bits = fn_state.gs_446688._3;
        // D s_25_13: cast reint s_25_12 -> u64
        let s_25_13: u64 = (s_25_12.value() as u64);
        // D s_25_14: write-var fault <= s_25_9
        fn_state.fault = s_25_9;
        // D s_25_15: write-var descipaddr <= s_25_10
        fn_state.descipaddr = s_25_10;
        // D s_25_16: write-var walkstate <= s_25_11
        fn_state.walkstate = s_25_11;
        // C s_25_17: const #0s : i
        let s_25_17: i128 = 0;
        // D s_25_18: read-var descriptor:u128
        let s_25_18: u128 = fn_state.descriptor;
        // D s_25_19: cast zx s_25_18 -> bv
        let s_25_19: Bits = Bits::new(s_25_18 as u128, 128u16);
        // D s_25_20: cast zx s_25_13 -> bv
        let s_25_20: Bits = Bits::new(s_25_13 as u128, 64u16);
        // C s_25_21: const #63s : i
        let s_25_21: i128 = 63;
        // C s_25_22: const #1u : u64
        let s_25_22: u64 = 1;
        // C s_25_23: cast zx s_25_22 -> bv
        let s_25_23: Bits = Bits::new(s_25_22 as u128, 64u16);
        // C s_25_24: lsl s_25_23 s_25_21
        let s_25_24: Bits = s_25_23 << s_25_21;
        // C s_25_25: sub s_25_24 s_25_23
        let s_25_25: Bits = ((s_25_24) - (s_25_23));
        // D s_25_26: and s_25_20 s_25_25
        let s_25_26: Bits = ((s_25_20) & (s_25_25));
        // D s_25_27: lsl s_25_26 s_25_17
        let s_25_27: Bits = s_25_26 << s_25_17;
        // C s_25_28: lsl s_25_25 s_25_17
        let s_25_28: Bits = s_25_25 << s_25_17;
        // C s_25_29: cmpl s_25_28
        let s_25_29: Bits = !s_25_28;
        // D s_25_30: and s_25_19 s_25_29
        let s_25_30: Bits = ((s_25_19) & (s_25_29));
        // D s_25_31: or s_25_30 s_25_27
        let s_25_31: Bits = ((s_25_30) | (s_25_27));
        // D s_25_32: cast reint s_25_31 -> u128
        let s_25_32: u128 = (s_25_31.value() as u128);
        // D s_25_33: write-var descriptor <= s_25_32
        fn_state.descriptor = s_25_32;
        // C s_25_34: const #128s : i
        let s_25_34: i128 = 128;
        // C s_25_35: const #127s : i
        let s_25_35: i128 = 127;
        // C s_25_36: const #64s : i
        let s_25_36: i128 = 64;
        // D s_25_37: read-var descriptor:u128
        let s_25_37: u128 = fn_state.descriptor;
        // D s_25_38: cast zx s_25_37 -> bv
        let s_25_38: Bits = Bits::new(s_25_37 as u128, 128u16);
        // D s_25_39: call set_subrange_zeros(s_25_34, s_25_38, s_25_35, s_25_36)
        let s_25_39: Bits = set_subrange_zeros(
            state,
            tracer,
            s_25_34,
            s_25_38,
            s_25_35,
            s_25_36,
        );
        // D s_25_40: cast reint s_25_39 -> u128
        let s_25_40: u128 = (s_25_39.value() as u128);
        // D s_25_41: write-var descriptor <= s_25_40
        fn_state.descriptor = s_25_40;
        // N s_25_42: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_26_0: read-var fault.16:struct
        let s_26_0: u32 = fn_state.fault._16;
        // C s_26_1: const #0u : u32
        let s_26_1: u32 = 0;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // N s_26_3: branch s_26_2 b156 b27
        if s_26_2 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_27_0: read-var accdesc.1:struct
        let s_27_0: u32 = fn_state.accdesc._1;
        // C s_27_1: const #0u : u32
        let s_27_1: u32 = 0;
        // D s_27_2: cmp-eq s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) == (s_27_1));
        // N s_27_3: branch s_27_2 b155 b28
        if s_27_2 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_29_0: read-var walkparams.21:struct
        let s_29_0: bool = fn_state.walkparams._21;
        // D s_29_1: read-var walkstate.7:struct
        let s_29_1: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_29_2: read-var accdesc:struct
        let s_29_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_29_3: read-var aligned:u8
        let s_29_3: bool = fn_state.aligned;
        // D s_29_4: call AArch64_S1HasAlignmentFault(s_29_2, s_29_3, s_29_0, s_29_1)
        let s_29_4: bool = AArch64_S1HasAlignmentFault(
            state,
            tracer,
            s_29_2,
            s_29_3,
            s_29_0,
            s_29_1,
        );
        // N s_29_5: branch s_29_4 b154 b30
        if s_29_4 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_30_0: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_31_0: read-var fault.16:struct
        let s_31_0: u32 = fn_state.fault._16;
        // C s_31_1: const #0u : u32
        let s_31_1: u32 = 0;
        // D s_31_2: cmp-eq s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) == (s_31_1));
        // N s_31_3: branch s_31_2 b153 b32
        if s_31_2 {
            return block_153(state, tracer, fn_state);
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
        // D s_33_0: read-var descriptor:u128
        let s_33_0: u128 = fn_state.descriptor;
        // D s_33_1: write-var new_desc <= s_33_0
        fn_state.new_desc = s_33_0;
        // D s_33_2: read-var walkparams.12:struct
        let s_33_2: bool = fn_state.walkparams._12;
        // D s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // C s_33_4: const #1u : u8
        let s_33_4: bool = true;
        // C s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 1u16);
        // D s_33_6: cmp-eq s_33_3 s_33_5
        let s_33_6: bool = ((s_33_3) == (s_33_5));
        // N s_33_7: branch s_33_6 b152 b34
        if s_33_6 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#19860 <= s_34_0
        fn_state.gs_19860 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_35_0: read-var gs#19860:u8
        let s_35_0: bool = fn_state.gs_19860;
        // N s_35_1: branch s_35_0 b151 b36
        if s_35_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_36_0: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_37_0: read-var fault:struct
        let s_37_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_37_1: call AArch64_SettingDirtyStatePermitted(s_37_0)
        let s_37_1: bool = AArch64_SettingDirtyStatePermitted(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b150 b38
        if s_37_1 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#19861 <= s_38_0
        fn_state.gs_19861 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_39_0: read-var gs#19861:u8
        let s_39_0: bool = fn_state.gs_19861;
        // N s_39_1: branch s_39_0 b149 b40
        if s_39_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#19862 <= s_40_0
        fn_state.gs_19862 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_41_0: read-var gs#19862:u8
        let s_41_0: bool = fn_state.gs_19862;
        // N s_41_1: branch s_41_0 b145 b42
        if s_41_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#19866 <= s_42_0
        fn_state.gs_19866 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_43_0: read-var gs#19866:u8
        let s_43_0: bool = fn_state.gs_19866;
        // N s_43_1: branch s_43_0 b144 b44
        if s_43_0 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#19867 <= s_44_0
        fn_state.gs_19867 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_45_0: read-var gs#19867:u8
        let s_45_0: bool = fn_state.gs_19867;
        // N s_45_1: branch s_45_0 b137 b46
        if s_45_0 {
            return block_137(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#19870 <= s_46_0
        fn_state.gs_19870 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_47_0: read-var gs#19870:u8
        let s_47_0: bool = fn_state.gs_19870;
        // N s_47_1: branch s_47_0 b136 b48
        if s_47_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_48_0: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_49_0: read-var new_desc:u128
        let s_49_0: u128 = fn_state.new_desc;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 128u16);
        // D s_49_2: read-var descriptor:u128
        let s_49_2: u128 = fn_state.descriptor;
        // D s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 128u16);
        // D s_49_4: cmp-ne s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) != (s_49_3));
        // N s_49_5: branch s_49_4 b125 b50
        if s_49_4 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_50_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_51_0: read-var new_desc:u128
        let s_51_0: u128 = fn_state.new_desc;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 128u16);
        // D s_51_2: read-var descriptor:u128
        let s_51_2: u128 = fn_state.descriptor;
        // D s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 128u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // N s_51_5: branch s_51_4 b124 b52
        if s_51_4 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_52_0: read-var mem_desc:u128
        let s_52_0: u128 = fn_state.mem_desc;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 128u16);
        // D s_52_2: read-var new_desc:u128
        let s_52_2: u128 = fn_state.new_desc;
        // D s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 128u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: write-var gs#19859 <= s_52_4
        fn_state.gs_19859 = s_52_4;
        // N s_52_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_53_0: read-var gs#19859:u8
        let s_53_0: bool = fn_state.gs_19859;
        // N s_53_1: branch s_53_0 b54 b24
        if s_53_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_54_0: const #19088u : u32
        let s_54_0: u32 = 19088;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: bool = {
            let value = state.read_register::<bool>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // N s_54_2: branch s_54_1 b123 b55
        if s_54_1 {
            return block_123(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#19930 <= s_55_0
        fn_state.gs_19930 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_56_0: read-var gs#19930:u8
        let s_56_0: bool = fn_state.gs_19930;
        // N s_56_1: branch s_56_0 b122 b57
        if s_56_0 {
            return block_122(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#19933 <= s_57_0
        fn_state.gs_19933 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_58_0: read-var gs#19933:u8
        let s_58_0: bool = fn_state.gs_19933;
        // N s_58_1: branch s_58_0 b113 b59
        if s_58_0 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_59_0: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_60_0: read-var fault.16:struct
        let s_60_0: u32 = fn_state.fault._16;
        // C s_60_1: const #0u : u32
        let s_60_1: u32 = 0;
        // D s_60_2: cmp-eq s_60_0 s_60_1
        let s_60_2: bool = ((s_60_0) == (s_60_1));
        // N s_60_3: branch s_60_2 b112 b61
        if s_60_2 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_61_0: read-var walkparams.3:struct
        let s_61_0: bool = fn_state.walkparams._3;
        // D s_61_1: read-var walkparams.36:struct
        let s_61_1: u32 = fn_state.walkparams._36;
        // D s_61_2: read-var va:u64
        let s_61_2: u64 = fn_state.va;
        // D s_61_3: read-var walkstate:struct
        let s_61_3: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_61_4: call StageOA(s_61_2, s_61_0, s_61_1, s_61_3)
        let s_61_4: ProductTypeda0231e9dc169f81 = StageOA(
            state,
            tracer,
            s_61_2,
            s_61_0,
            s_61_1,
            s_61_3,
        );
        // D s_61_5: write-var oa <= s_61_4
        fn_state.oa = s_61_4;
        // D s_61_6: read-var accdesc.1:struct
        let s_61_6: u32 = fn_state.accdesc._1;
        // C s_61_7: const #0u : u32
        let s_61_7: u32 = 0;
        // D s_61_8: cmp-eq s_61_6 s_61_7
        let s_61_8: bool = ((s_61_6) == (s_61_7));
        // N s_61_9: branch s_61_8 b108 b62
        if s_61_8 {
            return block_108(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#19962 <= s_62_0
        fn_state.gs_19962 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_63_0: read-var gs#19962:u8
        let s_63_0: bool = fn_state.gs_19962;
        // N s_63_1: branch s_63_0 b107 b64
        if s_63_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_64_0: read-var accdesc.1:struct
        let s_64_0: u32 = fn_state.accdesc._1;
        // C s_64_1: const #0u : u32
        let s_64_1: u32 = 0;
        // D s_64_2: cmp-eq s_64_0 s_64_1
        let s_64_2: bool = ((s_64_0) == (s_64_1));
        // N s_64_3: branch s_64_2 b106 b65
        if s_64_2 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#19974 <= s_65_0
        fn_state.gs_19974 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_66_0: read-var gs#19974:u8
        let s_66_0: bool = fn_state.gs_19974;
        // N s_66_1: branch s_66_0 b105 b67
        if s_66_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#19975 <= s_67_0
        fn_state.gs_19975 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_68_0: read-var gs#19975:u8
        let s_68_0: bool = fn_state.gs_19975;
        // N s_68_1: branch s_68_0 b95 b69
        if s_68_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_69_0: read-var walkstate.7:struct
        let s_69_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_69_1: write-var memattrs <= s_69_0
        fn_state.memattrs = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_70_0: read-var regime:u32
        let s_70_0: u32 = fn_state.regime;
        // C s_70_1: const #4u : u32
        let s_70_1: u32 = 4;
        // D s_70_2: cmp-eq s_70_0 s_70_1
        let s_70_2: bool = ((s_70_0) == (s_70_1));
        // N s_70_3: branch s_70_2 b94 b71
        if s_70_2 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#19963 <= s_71_0
        fn_state.gs_19963 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_72_0: read-var gs#19963:u8
        let s_72_0: bool = fn_state.gs_19963;
        // N s_72_1: branch s_72_0 b93 b73
        if s_72_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#19964 <= s_73_0
        fn_state.gs_19964 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_74_0: read-var gs#19964:u8
        let s_74_0: bool = fn_state.gs_19964;
        // N s_74_1: branch s_74_0 b92 b75
        if s_74_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#19965 <= s_75_0
        fn_state.gs_19965 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_76_0: read-var gs#19965:u8
        let s_76_0: bool = fn_state.gs_19965;
        // N s_76_1: branch s_76_0 b91 b77
        if s_76_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_77_0: read-var memattrs:struct
        let s_77_0: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_77_1: call EffectiveShareability(s_77_0)
        let s_77_1: u32 = EffectiveShareability(state, tracer, s_77_0);
        // D s_77_2: write-var memattrs.5 <= s_77_1
        fn_state.memattrs._5 = s_77_1;
        // N s_77_3: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_78_0: read-var accdesc.13:struct
        let s_78_0: bool = fn_state.accdesc._13;
        // N s_78_1: branch s_78_0 b90 b79
        if s_78_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#19966 <= s_79_0
        fn_state.gs_19966 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_80_0: read-var gs#19966:u8
        let s_80_0: bool = fn_state.gs_19966;
        // N s_80_1: branch s_80_0 b84 b81
        if s_80_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_81_0: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_82_0: read-var va:u64
        let s_82_0: u64 = fn_state.va;
        // D s_82_1: read-var oa:struct
        let s_82_1: ProductTypeda0231e9dc169f81 = fn_state.oa;
        // D s_82_2: read-var memattrs:struct
        let s_82_2: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_82_3: call CreateAddressDescriptor(s_82_0, s_82_1, s_82_2)
        let s_82_3: ProductTypece7c66ccb2cab13e = CreateAddressDescriptor(
            state,
            tracer,
            s_82_0,
            s_82_1,
            s_82_2,
        );
        // D s_82_4: write-var ipa <= s_82_3
        fn_state.ipa = s_82_3;
        // D s_82_5: read-var descipaddr.6:struct
        let s_82_5: ProductTypec0d0fb0603850c4c = fn_state.descipaddr._6;
        // D s_82_6: write-var ipa.6 <= s_82_5
        fn_state.ipa._6 = s_82_5;
        // D s_82_7: read-var walkstate.10:struct
        let s_82_7: bool = fn_state.walkstate._10;
        // D s_82_8: write-var ipa.4 <= s_82_7
        fn_state.ipa._4 = s_82_7;
        // D s_82_9: read-var va:u64
        let s_82_9: u64 = fn_state.va;
        // D s_82_10: call AArch64_GetVARange(s_82_9)
        let s_82_10: u32 = AArch64_GetVARange(state, tracer, s_82_9);
        // D s_82_11: read-var ipa.3:struct
        let s_82_11: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_82_12: write-var ga#15389 <= s_82_11
        fn_state.ga_15389 = s_82_11;
        // D s_82_13: read-var ga#15389.1:struct
        let s_82_13: u32 = fn_state.ga_15389._1;
        // D s_82_14: read-var descriptor:u128
        let s_82_14: u128 = fn_state.descriptor;
        // D s_82_15: cast zx s_82_14 -> bv
        let s_82_15: Bits = Bits::new(s_82_14 as u128, 128u16);
        // D s_82_16: read-var walkparams:struct
        let s_82_16: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_82_17: read-var regime:u32
        let s_82_17: u32 = fn_state.regime;
        // D s_82_18: call AArch64_S1OutputMECID(s_82_16, s_82_17, s_82_10, s_82_13, s_82_15)
        let s_82_18: u16 = AArch64_S1OutputMECID(
            state,
            tracer,
            s_82_16,
            s_82_17,
            s_82_10,
            s_82_13,
            s_82_15,
        );
        // D s_82_19: write-var ipa.1 <= s_82_18
        fn_state.ipa._1 = s_82_18;
        // D s_82_20: read-var fault:struct
        let s_82_20: ProductType1d757adad216cdef = fn_state.fault;
        // D s_82_21: create-product struct = ["s_82_20", "s_82_3"]
        let s_82_21: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_82_20,
            _1: s_82_3,
        };
        // D s_82_22: write-var return_value <= s_82_21
        fn_state.return_value = s_82_21;
        // N s_82_23: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_83_0: read-var return_value:struct
        let s_83_0: ProductTypedc31059ca7e2391c = fn_state.return_value;
        // N s_83_1: return s_83_0
        return s_83_0;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_84_0: read-var memattrs.1:struct
        let s_84_0: ProductType183e6678e5239c85 = fn_state.memattrs._1;
        // D s_84_1: write-var ga#15381 <= s_84_0
        fn_state.ga_15381 = s_84_0;
        // D s_84_2: read-var ga#15381.0:struct
        let s_84_2: u8 = fn_state.ga_15381._0;
        // D s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 2u16);
        // C s_84_4: const #464u : u32
        let s_84_4: u32 = 464;
        // D s_84_5: read-reg s_84_4:u8
        let s_84_5: u8 = {
            let value = state.read_register::<u8>(s_84_4 as isize);
            tracer.read_register(s_84_4 as isize, value);
            value
        };
        // D s_84_6: cast zx s_84_5 -> bv
        let s_84_6: Bits = Bits::new(s_84_5 as u128, 2u16);
        // D s_84_7: cmp-ne s_84_3 s_84_6
        let s_84_7: bool = ((s_84_3) != (s_84_6));
        // N s_84_8: branch s_84_7 b89 b85
        if s_84_7 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_85_0: read-var memattrs.4:struct
        let s_85_0: ProductType183e6678e5239c85 = fn_state.memattrs._4;
        // D s_85_1: write-var ga#15383 <= s_85_0
        fn_state.ga_15383 = s_85_0;
        // D s_85_2: read-var ga#15383.0:struct
        let s_85_2: u8 = fn_state.ga_15383._0;
        // D s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 2u16);
        // C s_85_4: const #464u : u32
        let s_85_4: u32 = 464;
        // D s_85_5: read-reg s_85_4:u8
        let s_85_5: u8 = {
            let value = state.read_register::<u8>(s_85_4 as isize);
            tracer.read_register(s_85_4 as isize, value);
            value
        };
        // D s_85_6: cast zx s_85_5 -> bv
        let s_85_6: Bits = Bits::new(s_85_5 as u128, 2u16);
        // D s_85_7: cmp-ne s_85_3 s_85_6
        let s_85_7: bool = ((s_85_3) != (s_85_6));
        // D s_85_8: write-var gs#19968 <= s_85_7
        fn_state.gs_19968 = s_85_7;
        // N s_85_9: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_86_0: read-var gs#19968:u8
        let s_86_0: bool = fn_state.gs_19968;
        // N s_86_1: branch s_86_0 b88 b87
        if s_86_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_87_0: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_88_0: const #22u : u32
        let s_88_0: u32 = 22;
        // D s_88_1: write-var fault.16 <= s_88_0
        fn_state.fault._16 = s_88_0;
        // C s_88_2: const #() : ()
        let s_88_2: () = ();
        // S s_88_3: call __UNKNOWN_AddressDescriptor(s_88_2)
        let s_88_3: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_88_2,
        );
        // D s_88_4: read-var fault:struct
        let s_88_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_88_5: create-product struct = ["s_88_4", "s_88_3"]
        let s_88_5: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_88_4,
            _1: s_88_3,
        };
        // D s_88_6: write-var return_value <= s_88_5
        fn_state.return_value = s_88_5;
        // N s_88_7: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_89_0: const #1u : u8
        let s_89_0: bool = true;
        // D s_89_1: write-var gs#19968 <= s_89_0
        fn_state.gs_19968 = s_89_0;
        // N s_89_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_90_0: read-var memattrs.2:struct
        let s_90_0: u32 = fn_state.memattrs._2;
        // C s_90_1: const #0u : u32
        let s_90_1: u32 = 0;
        // D s_90_2: cmp-eq s_90_0 s_90_1
        let s_90_2: bool = ((s_90_0) == (s_90_1));
        // D s_90_3: write-var gs#19966 <= s_90_2
        fn_state.gs_19966 = s_90_2;
        // N s_90_4: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_91_0: read-var walkstate.7:struct
        let s_91_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_91_1: write-var ga#15377 <= s_91_0
        fn_state.ga_15377 = s_91_0;
        // D s_91_2: read-var ga#15377.5:struct
        let s_91_2: u32 = fn_state.ga_15377._5;
        // D s_91_3: write-var memattrs.5 <= s_91_2
        fn_state.memattrs._5 = s_91_2;
        // N s_91_4: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_92_0: const #"Apply effective shareability at stage 1" : str
        let s_92_0: &'static str = "Apply effective shareability at stage 1";
        // S s_92_1: call __IMPDEF_boolean(s_92_0)
        let s_92_1: bool = u__IMPDEF_boolean(state, tracer, s_92_0);
        // S s_92_2: not s_92_1
        let s_92_2: bool = !s_92_1;
        // D s_92_3: write-var gs#19965 <= s_92_2
        fn_state.gs_19965 = s_92_2;
        // N s_92_4: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_93_0: const #102552u : u32
        let s_93_0: u32 = 102552;
        // D s_93_1: read-reg s_93_0:struct
        let s_93_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_93_0 as isize);
            tracer.read_register(s_93_0 as isize, value);
            value
        };
        // D s_93_2: call _get_HCR_EL2_Type_VM(s_93_1)
        let s_93_2: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_93_1);
        // D s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // C s_93_4: const #1u : u8
        let s_93_4: bool = true;
        // C s_93_5: cast zx s_93_4 -> bv
        let s_93_5: Bits = Bits::new(s_93_4 as u128, 1u16);
        // D s_93_6: cmp-eq s_93_3 s_93_5
        let s_93_6: bool = ((s_93_3) == (s_93_5));
        // D s_93_7: write-var gs#19964 <= s_93_6
        fn_state.gs_19964 = s_93_6;
        // N s_93_8: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_94_0: const #() : ()
        let s_94_0: () = ();
        // S s_94_1: call EL2Enabled(s_94_0)
        let s_94_1: bool = EL2Enabled(state, tracer, s_94_0);
        // D s_94_2: write-var gs#19963 <= s_94_1
        fn_state.gs_19963 = s_94_1;
        // N s_94_3: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_95_0: const #() : ()
        let s_95_0: () = ();
        // S s_95_1: call NormalNCMemAttr(s_95_0)
        let s_95_1: ProductTypef170cab34335b70c = NormalNCMemAttr(state, tracer, s_95_0);
        // D s_95_2: write-var memattrs <= s_95_1
        fn_state.memattrs = s_95_1;
        // D s_95_3: read-var walkstate.7:struct
        let s_95_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_95_4: write-var ga#15364 <= s_95_3
        fn_state.ga_15364 = s_95_3;
        // D s_95_5: read-var ga#15364.7:struct
        let s_95_5: bool = fn_state.ga_15364._7;
        // D s_95_6: write-var memattrs.7 <= s_95_5
        fn_state.memattrs._7 = s_95_5;
        // C s_95_7: const #() : ()
        let s_95_7: () = ();
        // S s_95_8: call HaveMTE2Ext(s_95_7)
        let s_95_8: bool = HaveMTE2Ext(state, tracer, s_95_7);
        // N s_95_9: branch s_95_8 b104 b96
        if s_95_8 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#19976 <= s_96_0
        fn_state.gs_19976 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_97_0: read-var gs#19976:u8
        let s_97_0: bool = fn_state.gs_19976;
        // N s_97_1: branch s_97_0 b103 b98
        if s_97_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#19977 <= s_98_0
        fn_state.gs_19977 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_99_0: read-var gs#19977:u8
        let s_99_0: bool = fn_state.gs_19977;
        // N s_99_1: branch s_99_0 b102 b100
        if s_99_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_100_0: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_101_0: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_102_0: const #0u : u32
        let s_102_0: u32 = 0;
        // D s_102_1: write-var memattrs.6 <= s_102_0
        fn_state.memattrs._6 = s_102_0;
        // N s_102_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_103_0: const #10u : u32
        let s_103_0: u32 = 10;
        // S s_103_1: call ConstrainUnpredictableBool(s_103_0)
        let s_103_1: bool = ConstrainUnpredictableBool(state, tracer, s_103_0);
        // S s_103_2: not s_103_1
        let s_103_2: bool = !s_103_1;
        // D s_103_3: write-var gs#19977 <= s_103_2
        fn_state.gs_19977 = s_103_2;
        // N s_103_4: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_104_0: read-var walkstate.7:struct
        let s_104_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_104_1: write-var ga#15365 <= s_104_0
        fn_state.ga_15365 = s_104_0;
        // D s_104_2: read-var ga#15365.6:struct
        let s_104_2: u32 = fn_state.ga_15365._6;
        // C s_104_3: const #1u : u32
        let s_104_3: u32 = 1;
        // D s_104_4: cmp-eq s_104_2 s_104_3
        let s_104_4: bool = ((s_104_2) == (s_104_3));
        // D s_104_5: write-var gs#19976 <= s_104_4
        fn_state.gs_19976 = s_104_4;
        // N s_104_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_105_0: read-var walkstate.7:struct
        let s_105_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_105_1: write-var ga#15360 <= s_105_0
        fn_state.ga_15360 = s_105_0;
        // D s_105_2: read-var ga#15360.2:struct
        let s_105_2: u32 = fn_state.ga_15360._2;
        // C s_105_3: const #0u : u32
        let s_105_3: u32 = 0;
        // D s_105_4: cmp-eq s_105_2 s_105_3
        let s_105_4: bool = ((s_105_2) == (s_105_3));
        // D s_105_5: write-var gs#19975 <= s_105_4
        fn_state.gs_19975 = s_105_4;
        // N s_105_6: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_106_0: read-var regime:u32
        let s_106_0: u32 = fn_state.regime;
        // D s_106_1: call AArch64_S1DCacheEnabled(s_106_0)
        let s_106_1: bool = AArch64_S1DCacheEnabled(state, tracer, s_106_0);
        // D s_106_2: not s_106_1
        let s_106_2: bool = !s_106_1;
        // D s_106_3: write-var gs#19974 <= s_106_2
        fn_state.gs_19974 = s_106_2;
        // N s_106_4: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_107_0: const #() : ()
        let s_107_0: () = ();
        // S s_107_1: call NormalNCMemAttr(s_107_0)
        let s_107_1: ProductTypef170cab34335b70c = NormalNCMemAttr(
            state,
            tracer,
            s_107_0,
        );
        // D s_107_2: write-var memattrs <= s_107_1
        fn_state.memattrs = s_107_1;
        // D s_107_3: read-var walkstate.7:struct
        let s_107_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_107_4: write-var ga#15356 <= s_107_3
        fn_state.ga_15356 = s_107_3;
        // D s_107_5: read-var ga#15356.7:struct
        let s_107_5: bool = fn_state.ga_15356._7;
        // D s_107_6: write-var memattrs.7 <= s_107_5
        fn_state.memattrs._7 = s_107_5;
        // N s_107_7: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_108_0: read-var walkstate.7:struct
        let s_108_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_108_1: write-var ga#15350 <= s_108_0
        fn_state.ga_15350 = s_108_0;
        // D s_108_2: read-var ga#15350.2:struct
        let s_108_2: u32 = fn_state.ga_15350._2;
        // C s_108_3: const #1u : u32
        let s_108_3: u32 = 1;
        // D s_108_4: cmp-eq s_108_2 s_108_3
        let s_108_4: bool = ((s_108_2) == (s_108_3));
        // N s_108_5: branch s_108_4 b111 b109
        if s_108_4 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_109_0: read-var regime:u32
        let s_109_0: u32 = fn_state.regime;
        // D s_109_1: call AArch64_S1ICacheEnabled(s_109_0)
        let s_109_1: bool = AArch64_S1ICacheEnabled(state, tracer, s_109_0);
        // D s_109_2: not s_109_1
        let s_109_2: bool = !s_109_1;
        // D s_109_3: write-var gs#19961 <= s_109_2
        fn_state.gs_19961 = s_109_2;
        // N s_109_4: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_110_0: read-var gs#19961:u8
        let s_110_0: bool = fn_state.gs_19961;
        // D s_110_1: write-var gs#19962 <= s_110_0
        fn_state.gs_19962 = s_110_0;
        // N s_110_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_111_0: const #1u : u8
        let s_111_0: bool = true;
        // D s_111_1: write-var gs#19961 <= s_111_0
        fn_state.gs_19961 = s_111_0;
        // N s_111_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_112_0: const #() : ()
        let s_112_0: () = ();
        // S s_112_1: call __UNKNOWN_AddressDescriptor(s_112_0)
        let s_112_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_112_0,
        );
        // D s_112_2: read-var fault:struct
        let s_112_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_112_3: create-product struct = ["s_112_2", "s_112_1"]
        let s_112_3: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_112_2,
            _1: s_112_1,
        };
        // D s_112_4: write-var return_value <= s_112_3
        fn_state.return_value = s_112_3;
        // N s_112_5: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_113_0: const #10s : i
        let s_113_0: i128 = 10;
        // D s_113_1: read-var descriptor:u128
        let s_113_1: u128 = fn_state.descriptor;
        // D s_113_2: cast zx s_113_1 -> bv
        let s_113_2: Bits = Bits::new(s_113_1 as u128, 128u16);
        // C s_113_3: const #1u : u64
        let s_113_3: u64 = 1;
        // D s_113_4: bit-extract s_113_2 s_113_0 s_113_3
        let s_113_4: Bits = (Bits::new(
            ((s_113_2) >> (s_113_0)).value(),
            u16::try_from(s_113_3).unwrap(),
        ));
        // D s_113_5: cast reint s_113_4 -> u8
        let s_113_5: bool = ((s_113_4.value()) != 0);
        // C s_113_6: const #0s : i
        let s_113_6: i128 = 0;
        // C s_113_7: const #0u : u64
        let s_113_7: u64 = 0;
        // D s_113_8: cast zx s_113_5 -> u64
        let s_113_8: u64 = (s_113_5 as u64);
        // C s_113_9: const #1u : u64
        let s_113_9: u64 = 1;
        // D s_113_10: and s_113_8 s_113_9
        let s_113_10: u64 = ((s_113_8) & (s_113_9));
        // D s_113_11: cmp-eq s_113_10 s_113_9
        let s_113_11: bool = ((s_113_10) == (s_113_9));
        // D s_113_12: lsl s_113_8 s_113_6
        let s_113_12: u64 = s_113_8 << s_113_6;
        // D s_113_13: or s_113_7 s_113_12
        let s_113_13: u64 = ((s_113_7) | (s_113_12));
        // D s_113_14: cmpl s_113_12
        let s_113_14: u64 = !s_113_12;
        // D s_113_15: and s_113_7 s_113_14
        let s_113_15: u64 = ((s_113_7) & (s_113_14));
        // D s_113_16: select s_113_11 s_113_13 s_113_15
        let s_113_16: u64 = if s_113_11 { s_113_13 } else { s_113_15 };
        // D s_113_17: cast trunc s_113_16 -> u8
        let s_113_17: bool = ((s_113_16) != 0);
        // D s_113_18: cast zx s_113_17 -> bv
        let s_113_18: Bits = Bits::new(s_113_17 as u128, 1u16);
        // C s_113_19: const #1u : u8
        let s_113_19: bool = true;
        // C s_113_20: cast zx s_113_19 -> bv
        let s_113_20: Bits = Bits::new(s_113_19 as u128, 1u16);
        // D s_113_21: cmp-eq s_113_18 s_113_20
        let s_113_21: bool = ((s_113_18) == (s_113_20));
        // N s_113_22: branch s_113_21 b121 b114
        if s_113_21 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_114_0: read-var descipaddr.6:struct
        let s_114_0: ProductTypec0d0fb0603850c4c = fn_state.descipaddr._6;
        // D s_114_1: write-var tlbrecord.1 <= s_114_0
        fn_state.tlbrecord._1 = s_114_0;
        // D s_114_2: read-var walkstate.7:struct
        let s_114_2: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_114_3: write-var ga#15327 <= s_114_2
        fn_state.ga_15327 = s_114_2;
        // D s_114_4: read-var ga#15327.7:struct
        let s_114_4: bool = fn_state.ga_15327._7;
        // D s_114_5: write-var tlbrecord.1.14 <= s_114_4
        fn_state.tlbrecord._1._14 = s_114_4;
        // D s_114_6: read-var walkstate.6:struct
        let s_114_6: i128 = fn_state.walkstate._6;
        // D s_114_7: write-var tlbrecord.1.8 <= s_114_6
        fn_state.tlbrecord._1._8 = s_114_6;
        // D s_114_8: read-var walkstate.8:struct
        let s_114_8: bool = fn_state.walkstate._8;
        // D s_114_9: write-var tlbrecord.1.9 <= s_114_8
        fn_state.tlbrecord._1._9 = s_114_8;
        // D s_114_10: read-var walkparams.3:struct
        let s_114_10: bool = fn_state.walkparams._3;
        // D s_114_11: cast zx s_114_10 -> bv
        let s_114_11: Bits = Bits::new(s_114_10 as u128, 1u16);
        // C s_114_12: const #1u : u8
        let s_114_12: bool = true;
        // C s_114_13: cast zx s_114_12 -> bv
        let s_114_13: Bits = Bits::new(s_114_12 as u128, 1u16);
        // D s_114_14: cmp-eq s_114_11 s_114_13
        let s_114_14: bool = ((s_114_11) == (s_114_13));
        // D s_114_15: write-var tlbrecord.1.7 <= s_114_14
        fn_state.tlbrecord._1._7 = s_114_14;
        // D s_114_16: read-var walkstate:struct
        let s_114_16: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_114_17: write-var tlbrecord.5 <= s_114_16
        fn_state.tlbrecord._5 = s_114_16;
        // D s_114_18: read-var walkparams.3:struct
        let s_114_18: bool = fn_state.walkparams._3;
        // D s_114_19: read-var walkparams.36:struct
        let s_114_19: u32 = fn_state.walkparams._36;
        // D s_114_20: read-var walkstate.6:struct
        let s_114_20: i128 = fn_state.walkstate._6;
        // D s_114_21: call TranslationSize(s_114_18, s_114_19, s_114_20)
        let s_114_21: i128 = TranslationSize(
            state,
            tracer,
            s_114_18,
            s_114_19,
            s_114_20,
        );
        // D s_114_22: write-var tlbrecord.0 <= s_114_21
        fn_state.tlbrecord._0 = s_114_21;
        // D s_114_23: read-var walkstate.1:struct
        let s_114_23: bool = fn_state.walkstate._1;
        // D s_114_24: cast zx s_114_23 -> bv
        let s_114_24: Bits = Bits::new(s_114_23 as u128, 1u16);
        // C s_114_25: const #1u : u8
        let s_114_25: bool = true;
        // C s_114_26: cast zx s_114_25 -> bv
        let s_114_26: Bits = Bits::new(s_114_25 as u128, 1u16);
        // D s_114_27: cmp-eq s_114_24 s_114_26
        let s_114_27: bool = ((s_114_24) == (s_114_26));
        // N s_114_28: branch s_114_27 b120 b115
        if s_114_27 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_115_0: const #0s : i
        let s_115_0: i128 = 0;
        // D s_115_1: write-var tlbrecord.2 <= s_115_0
        fn_state.tlbrecord._2 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_116_0: read-var walkparams.3:struct
        let s_116_0: bool = fn_state.walkparams._3;
        // D s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 1u16);
        // C s_116_2: const #1u : u8
        let s_116_2: bool = true;
        // C s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 1u16);
        // D s_116_4: cmp-eq s_116_1 s_116_3
        let s_116_4: bool = ((s_116_1) == (s_116_3));
        // N s_116_5: branch s_116_4 b119 b117
        if s_116_4 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_117_0: read-var tlbrecord:struct
        let s_117_0: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_117_1: write-var tlbrecord <= s_117_0
        fn_state.tlbrecord = s_117_0;
        // C s_117_2: const #64s : i
        let s_117_2: i128 = 64;
        // S s_117_3: call Zeros(s_117_2)
        let s_117_3: Bits = Zeros(state, tracer, s_117_2);
        // D s_117_4: read-var tlbrecord:struct
        let s_117_4: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_117_5: write-var tlbrecord <= s_117_4
        fn_state.tlbrecord = s_117_4;
        // N s_117_6: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_118_0: read-var tlbrecord:struct
        let s_118_0: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_118_1: call S1TLBCache(s_118_0)
        let s_118_1: () = S1TLBCache(state, tracer, s_118_0);
        // N s_118_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_119_0: read-var mem_desc:u128
        let s_119_0: u128 = fn_state.mem_desc;
        // D s_119_1: write-var tlbrecord.3 <= s_119_0
        fn_state.tlbrecord._3 = s_119_0;
        // N s_119_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_120_0: read-var walkparams.3:struct
        let s_120_0: bool = fn_state.walkparams._3;
        // D s_120_1: read-var walkparams.36:struct
        let s_120_1: u32 = fn_state.walkparams._36;
        // D s_120_2: read-var walkstate.6:struct
        let s_120_2: i128 = fn_state.walkstate._6;
        // D s_120_3: call ContiguousSize(s_120_0, s_120_1, s_120_2)
        let s_120_3: i128 = ContiguousSize(state, tracer, s_120_0, s_120_1, s_120_2);
        // D s_120_4: write-var tlbrecord.2 <= s_120_3
        fn_state.tlbrecord._2 = s_120_3;
        // N s_120_5: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_121_0: read-var descipaddr.6:struct
        let s_121_0: ProductTypec0d0fb0603850c4c = fn_state.descipaddr._6;
        // D s_121_1: call S1TLBLookup(s_121_0)
        let s_121_1: ProductTypeeb828c17bbe5e68 = S1TLBLookup(state, tracer, s_121_0);
        // D s_121_2: write-var tlbentryshadow#323 <= s_121_1
        fn_state.tlbentryshadow_323 = s_121_1;
        // D s_121_3: read-var tlbentryshadow#323.1:struct
        let s_121_3: bool = fn_state.tlbentryshadow_323._1;
        // N s_121_4: assert s_121_3
        let s_121_4: () = assert!(s_121_3);
        // D s_121_5: read-var tlbentryshadow#323.0:struct
        let s_121_5: ProductTypee47dd77b186df56e = fn_state.tlbentryshadow_323._0;
        // D s_121_6: write-var tlbrecord <= s_121_5
        fn_state.tlbrecord = s_121_5;
        // N s_121_7: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_122_0: const #10s : i
        let s_122_0: i128 = 10;
        // D s_122_1: read-var mem_desc:u128
        let s_122_1: u128 = fn_state.mem_desc;
        // D s_122_2: cast zx s_122_1 -> bv
        let s_122_2: Bits = Bits::new(s_122_1 as u128, 128u16);
        // C s_122_3: const #1u : u64
        let s_122_3: u64 = 1;
        // D s_122_4: bit-extract s_122_2 s_122_0 s_122_3
        let s_122_4: Bits = (Bits::new(
            ((s_122_2) >> (s_122_0)).value(),
            u16::try_from(s_122_3).unwrap(),
        ));
        // D s_122_5: cast reint s_122_4 -> u8
        let s_122_5: bool = ((s_122_4.value()) != 0);
        // C s_122_6: const #0s : i
        let s_122_6: i128 = 0;
        // C s_122_7: const #0u : u64
        let s_122_7: u64 = 0;
        // D s_122_8: cast zx s_122_5 -> u64
        let s_122_8: u64 = (s_122_5 as u64);
        // C s_122_9: const #1u : u64
        let s_122_9: u64 = 1;
        // D s_122_10: and s_122_8 s_122_9
        let s_122_10: u64 = ((s_122_8) & (s_122_9));
        // D s_122_11: cmp-eq s_122_10 s_122_9
        let s_122_11: bool = ((s_122_10) == (s_122_9));
        // D s_122_12: lsl s_122_8 s_122_6
        let s_122_12: u64 = s_122_8 << s_122_6;
        // D s_122_13: or s_122_7 s_122_12
        let s_122_13: u64 = ((s_122_7) | (s_122_12));
        // D s_122_14: cmpl s_122_12
        let s_122_14: u64 = !s_122_12;
        // D s_122_15: and s_122_7 s_122_14
        let s_122_15: u64 = ((s_122_7) & (s_122_14));
        // D s_122_16: select s_122_11 s_122_13 s_122_15
        let s_122_16: u64 = if s_122_11 { s_122_13 } else { s_122_15 };
        // D s_122_17: cast trunc s_122_16 -> u8
        let s_122_17: bool = ((s_122_16) != 0);
        // D s_122_18: cast zx s_122_17 -> bv
        let s_122_18: Bits = Bits::new(s_122_17 as u128, 1u16);
        // C s_122_19: const #1u : u8
        let s_122_19: bool = true;
        // C s_122_20: cast zx s_122_19 -> bv
        let s_122_20: Bits = Bits::new(s_122_19 as u128, 1u16);
        // D s_122_21: cmp-eq s_122_18 s_122_20
        let s_122_21: bool = ((s_122_18) == (s_122_20));
        // D s_122_22: write-var gs#19933 <= s_122_21
        fn_state.gs_19933 = s_122_21;
        // N s_122_23: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_123_0: read-var new_desc:u128
        let s_123_0: u128 = fn_state.new_desc;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 128u16);
        // D s_123_2: read-var descriptor:u128
        let s_123_2: u128 = fn_state.descriptor;
        // D s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 128u16);
        // D s_123_4: cmp-ne s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) != (s_123_3));
        // D s_123_5: write-var gs#19930 <= s_123_4
        fn_state.gs_19930 = s_123_4;
        // N s_123_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_124_0: const #1u : u8
        let s_124_0: bool = true;
        // D s_124_1: write-var gs#19859 <= s_124_0
        fn_state.gs_19859 = s_124_0;
        // N s_124_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_125_0: read-var accdesc:struct
        let s_125_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_125_1: call CreateAccDescTTEUpdate(s_125_0)
        let s_125_1: ProductType9878976b5bcce9c9 = CreateAccDescTTEUpdate(
            state,
            tracer,
            s_125_0,
        );
        // D s_125_2: write-var descaccess <= s_125_1
        fn_state.descaccess = s_125_1;
        // D s_125_3: read-var regime:u32
        let s_125_3: u32 = fn_state.regime;
        // C s_125_4: const #4u : u32
        let s_125_4: u32 = 4;
        // D s_125_5: cmp-eq s_125_3 s_125_4
        let s_125_5: bool = ((s_125_3) == (s_125_4));
        // N s_125_6: branch s_125_5 b135 b126
        if s_125_5 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_126_0: const #0u : u8
        let s_126_0: bool = false;
        // D s_126_1: write-var gs#19872 <= s_126_0
        fn_state.gs_19872 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_127_0: read-var gs#19872:u8
        let s_127_0: bool = fn_state.gs_19872;
        // N s_127_1: branch s_127_0 b132 b128
        if s_127_0 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_128_0: read-var descipaddr:struct
        let s_128_0: ProductTypece7c66ccb2cab13e = fn_state.descipaddr;
        // D s_128_1: write-var descpaddr <= s_128_0
        fn_state.descpaddr = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_129_0: const #() : ()
        let s_129_0: () = ();
        // D s_129_1: create-sum enum = 0:"s_129_0"
        let s_129_1: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_129_0);
        // C s_129_2: const #() : ()
        let s_129_2: () = ();
        // D s_129_3: create-sum enum = 0:"s_129_2"
        let s_129_3: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_129_2);
        // D s_129_4: read-var walkstate.6:struct
        let s_129_4: i128 = fn_state.walkstate._6;
        // D s_129_5: create-sum enum = 1:"s_129_4"
        let s_129_5: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_1(s_129_4);
        // C s_129_6: const #() : ()
        let s_129_6: () = ();
        // D s_129_7: create-sum enum = 0:"s_129_6"
        let s_129_7: SumType3cca557f9e907281 = SumType3cca557f9e907281::_0(s_129_6);
        // D s_129_8: read-var walkparams:struct
        let s_129_8: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_129_9: create-sum enum = 1:"s_129_8"
        let s_129_9: SumTypefc0aa8a49e605a17 = SumTypefc0aa8a49e605a17::_1(s_129_8);
        // C s_129_10: const #() : ()
        let s_129_10: () = ();
        // D s_129_11: create-sum enum = 0:"s_129_10"
        let s_129_11: SumType3436044442b382d9 = SumType3436044442b382d9::_0(s_129_10);
        // D s_129_12: read-var walkstate.7:struct
        let s_129_12: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_129_13: read-var regime:u32
        let s_129_13: u32 = fn_state.regime;
        // D s_129_14: read-var va:u64
        let s_129_14: u64 = fn_state.va;
        // D s_129_15: create-product struct = ["s_129_3", "s_129_12", "s_129_13", "s_129_5", "s_129_9", "s_129_7", "s_129_11", "s_129_14", "s_129_1"]
        let s_129_15: ProductTypeb525737120e184b3 = ProductTypeb525737120e184b3 {
            _0: s_129_3,
            _1: s_129_12,
            _2: s_129_13,
            _3: s_129_5,
            _4: s_129_9,
            _5: s_129_7,
            _6: s_129_11,
            _7: s_129_14,
            _8: s_129_1,
        };
        // D s_129_16: write-var translation_info <= s_129_15
        fn_state.translation_info = s_129_15;
        // D s_129_17: read-var walkparams.3:struct
        let s_129_17: bool = fn_state.walkparams._3;
        // D s_129_18: cast zx s_129_17 -> bv
        let s_129_18: Bits = Bits::new(s_129_17 as u128, 1u16);
        // C s_129_19: const #1u : u8
        let s_129_19: bool = true;
        // C s_129_20: cast zx s_129_19 -> bv
        let s_129_20: Bits = Bits::new(s_129_19 as u128, 1u16);
        // D s_129_21: cmp-eq s_129_18 s_129_20
        let s_129_21: bool = ((s_129_18) == (s_129_20));
        // N s_129_22: branch s_129_21 b131 b130
        if s_129_21 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_130_0: const #0s : i
        let s_130_0: i128 = 0;
        // D s_130_1: read-var descriptor:u128
        let s_130_1: u128 = fn_state.descriptor;
        // D s_130_2: cast zx s_130_1 -> bv
        let s_130_2: Bits = Bits::new(s_130_1 as u128, 128u16);
        // C s_130_3: const #1s : i64
        let s_130_3: i64 = 1;
        // C s_130_4: cast zx s_130_3 -> i
        let s_130_4: i128 = (i128::try_from(s_130_3).unwrap());
        // C s_130_5: const #63s : i
        let s_130_5: i128 = 63;
        // C s_130_6: add s_130_5 s_130_4
        let s_130_6: i128 = (s_130_5 + s_130_4);
        // D s_130_7: bit-extract s_130_2 s_130_0 s_130_6
        let s_130_7: Bits = (Bits::new(
            ((s_130_2) >> (s_130_0)).value(),
            u16::try_from(s_130_6).unwrap(),
        ));
        // D s_130_8: cast reint s_130_7 -> u64
        let s_130_8: u64 = (s_130_7.value() as u64);
        // C s_130_9: const #0s : i
        let s_130_9: i128 = 0;
        // D s_130_10: read-var new_desc:u128
        let s_130_10: u128 = fn_state.new_desc;
        // D s_130_11: cast zx s_130_10 -> bv
        let s_130_11: Bits = Bits::new(s_130_10 as u128, 128u16);
        // C s_130_12: const #1s : i64
        let s_130_12: i64 = 1;
        // C s_130_13: cast zx s_130_12 -> i
        let s_130_13: i128 = (i128::try_from(s_130_12).unwrap());
        // C s_130_14: const #63s : i
        let s_130_14: i128 = 63;
        // C s_130_15: add s_130_14 s_130_13
        let s_130_15: i128 = (s_130_14 + s_130_13);
        // D s_130_16: bit-extract s_130_11 s_130_9 s_130_15
        let s_130_16: Bits = (Bits::new(
            ((s_130_11) >> (s_130_9)).value(),
            u16::try_from(s_130_15).unwrap(),
        ));
        // D s_130_17: cast reint s_130_16 -> u64
        let s_130_17: u64 = (s_130_16.value() as u64);
        // D s_130_18: read-var walkparams.9:struct
        let s_130_18: bool = fn_state.walkparams._9;
        // D s_130_19: cast zx s_130_8 -> bv
        let s_130_19: Bits = Bits::new(s_130_8 as u128, 64u16);
        // D s_130_20: cast zx s_130_17 -> bv
        let s_130_20: Bits = Bits::new(s_130_17 as u128, 64u16);
        // D s_130_21: read-var fault:struct
        let s_130_21: ProductType1d757adad216cdef = fn_state.fault;
        // D s_130_22: read-var descaccess:struct
        let s_130_22: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_130_23: read-var descpaddr:struct
        let s_130_23: ProductTypece7c66ccb2cab13e = fn_state.descpaddr;
        // D s_130_24: read-var translation_info:struct
        let s_130_24: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_130_25: call AArch64_MemSwapTableDesc(s_130_21, s_130_19, s_130_20, s_130_18, s_130_22, s_130_23, s_130_24)
        let s_130_25: ProductTypeb4cea7287e2eb9d6 = AArch64_MemSwapTableDesc(
            state,
            tracer,
            s_130_21,
            s_130_19,
            s_130_20,
            s_130_18,
            s_130_22,
            s_130_23,
            s_130_24,
        );
        // D s_130_26: write-var gs#446722 <= s_130_25
        fn_state.gs_446722 = s_130_25;
        // D s_130_27: read-var gs#446722.0:struct
        let s_130_27: ProductType1d757adad216cdef = fn_state.gs_446722._0;
        // D s_130_28: read-var gs#446722.1:struct
        let s_130_28: Bits = fn_state.gs_446722._1;
        // D s_130_29: cast reint s_130_28 -> u64
        let s_130_29: u64 = (s_130_28.value() as u64);
        // D s_130_30: write-var fault <= s_130_27
        fn_state.fault = s_130_27;
        // C s_130_31: const #0s : i
        let s_130_31: i128 = 0;
        // D s_130_32: read-var mem_desc:u128
        let s_130_32: u128 = fn_state.mem_desc;
        // D s_130_33: cast zx s_130_32 -> bv
        let s_130_33: Bits = Bits::new(s_130_32 as u128, 128u16);
        // D s_130_34: cast zx s_130_29 -> bv
        let s_130_34: Bits = Bits::new(s_130_29 as u128, 64u16);
        // C s_130_35: const #63s : i
        let s_130_35: i128 = 63;
        // C s_130_36: const #1u : u64
        let s_130_36: u64 = 1;
        // C s_130_37: cast zx s_130_36 -> bv
        let s_130_37: Bits = Bits::new(s_130_36 as u128, 64u16);
        // C s_130_38: lsl s_130_37 s_130_35
        let s_130_38: Bits = s_130_37 << s_130_35;
        // C s_130_39: sub s_130_38 s_130_37
        let s_130_39: Bits = ((s_130_38) - (s_130_37));
        // D s_130_40: and s_130_34 s_130_39
        let s_130_40: Bits = ((s_130_34) & (s_130_39));
        // D s_130_41: lsl s_130_40 s_130_31
        let s_130_41: Bits = s_130_40 << s_130_31;
        // C s_130_42: lsl s_130_39 s_130_31
        let s_130_42: Bits = s_130_39 << s_130_31;
        // C s_130_43: cmpl s_130_42
        let s_130_43: Bits = !s_130_42;
        // D s_130_44: and s_130_33 s_130_43
        let s_130_44: Bits = ((s_130_33) & (s_130_43));
        // D s_130_45: or s_130_44 s_130_41
        let s_130_45: Bits = ((s_130_44) | (s_130_41));
        // D s_130_46: cast reint s_130_45 -> u128
        let s_130_46: u128 = (s_130_45.value() as u128);
        // D s_130_47: write-var mem_desc <= s_130_46
        fn_state.mem_desc = s_130_46;
        // C s_130_48: const #128s : i
        let s_130_48: i128 = 128;
        // C s_130_49: const #127s : i
        let s_130_49: i128 = 127;
        // C s_130_50: const #64s : i
        let s_130_50: i128 = 64;
        // D s_130_51: read-var mem_desc:u128
        let s_130_51: u128 = fn_state.mem_desc;
        // D s_130_52: cast zx s_130_51 -> bv
        let s_130_52: Bits = Bits::new(s_130_51 as u128, 128u16);
        // D s_130_53: call set_subrange_zeros(s_130_48, s_130_52, s_130_49, s_130_50)
        let s_130_53: Bits = set_subrange_zeros(
            state,
            tracer,
            s_130_48,
            s_130_52,
            s_130_49,
            s_130_50,
        );
        // D s_130_54: cast reint s_130_53 -> u128
        let s_130_54: u128 = (s_130_53.value() as u128);
        // D s_130_55: write-var mem_desc <= s_130_54
        fn_state.mem_desc = s_130_54;
        // N s_130_56: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_131_0: read-var walkparams.9:struct
        let s_131_0: bool = fn_state.walkparams._9;
        // D s_131_1: read-var descriptor:u128
        let s_131_1: u128 = fn_state.descriptor;
        // D s_131_2: cast zx s_131_1 -> bv
        let s_131_2: Bits = Bits::new(s_131_1 as u128, 128u16);
        // D s_131_3: read-var new_desc:u128
        let s_131_3: u128 = fn_state.new_desc;
        // D s_131_4: cast zx s_131_3 -> bv
        let s_131_4: Bits = Bits::new(s_131_3 as u128, 128u16);
        // D s_131_5: read-var fault:struct
        let s_131_5: ProductType1d757adad216cdef = fn_state.fault;
        // D s_131_6: read-var descaccess:struct
        let s_131_6: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_131_7: read-var descpaddr:struct
        let s_131_7: ProductTypece7c66ccb2cab13e = fn_state.descpaddr;
        // D s_131_8: read-var translation_info:struct
        let s_131_8: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_131_9: call AArch64_MemSwapTableDesc(s_131_5, s_131_2, s_131_4, s_131_0, s_131_6, s_131_7, s_131_8)
        let s_131_9: ProductTypeb4cea7287e2eb9d6 = AArch64_MemSwapTableDesc(
            state,
            tracer,
            s_131_5,
            s_131_2,
            s_131_4,
            s_131_0,
            s_131_6,
            s_131_7,
            s_131_8,
        );
        // D s_131_10: write-var gs#446730 <= s_131_9
        fn_state.gs_446730 = s_131_9;
        // D s_131_11: read-var gs#446730.0:struct
        let s_131_11: ProductType1d757adad216cdef = fn_state.gs_446730._0;
        // D s_131_12: read-var gs#446730.1:struct
        let s_131_12: Bits = fn_state.gs_446730._1;
        // D s_131_13: cast reint s_131_12 -> u128
        let s_131_13: u128 = (s_131_12.value() as u128);
        // D s_131_14: write-var fault <= s_131_11
        fn_state.fault = s_131_11;
        // D s_131_15: write-var mem_desc <= s_131_13
        fn_state.mem_desc = s_131_13;
        // N s_131_16: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_132_0: const #1u : u8
        let s_132_0: bool = true;
        // C s_132_1: const #1u : u8
        let s_132_1: bool = true;
        // C s_132_2: const #() : ()
        let s_132_2: () = ();
        // D s_132_3: create-sum enum = 0:"s_132_2"
        let s_132_3: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_0(s_132_2);
        // D s_132_4: read-var fault:struct
        let s_132_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_132_5: read-var descipaddr:struct
        let s_132_5: ProductTypece7c66ccb2cab13e = fn_state.descipaddr;
        // D s_132_6: read-var descaccess:struct
        let s_132_6: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_132_7: call AArch64_S2Translate(s_132_4, s_132_5, s_132_0, s_132_3, s_132_1, s_132_6)
        let s_132_7: ProductTypedc31059ca7e2391c = AArch64_S2Translate(
            state,
            tracer,
            s_132_4,
            s_132_5,
            s_132_0,
            s_132_3,
            s_132_1,
            s_132_6,
        );
        // D s_132_8: write-var ga#15297 <= s_132_7
        fn_state.ga_15297 = s_132_7;
        // D s_132_9: read-var ga#15297.0:struct
        let s_132_9: ProductType1d757adad216cdef = fn_state.ga_15297._0;
        // D s_132_10: read-var ga#15297.1:struct
        let s_132_10: ProductTypece7c66ccb2cab13e = fn_state.ga_15297._1;
        // D s_132_11: write-var s2fault <= s_132_9
        fn_state.s2fault = s_132_9;
        // D s_132_12: write-var descpaddr <= s_132_10
        fn_state.descpaddr = s_132_10;
        // D s_132_13: read-var s2fault.16:struct
        let s_132_13: u32 = fn_state.s2fault._16;
        // C s_132_14: const #0u : u32
        let s_132_14: u32 = 0;
        // D s_132_15: cmp-eq s_132_13 s_132_14
        let s_132_15: bool = ((s_132_13) == (s_132_14));
        // N s_132_16: branch s_132_15 b134 b133
        if s_132_15 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_133_0: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_134_0: const #() : ()
        let s_134_0: () = ();
        // S s_134_1: call __UNKNOWN_AddressDescriptor(s_134_0)
        let s_134_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_134_0,
        );
        // D s_134_2: read-var s2fault:struct
        let s_134_2: ProductType1d757adad216cdef = fn_state.s2fault;
        // D s_134_3: create-product struct = ["s_134_2", "s_134_1"]
        let s_134_3: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_134_2,
            _1: s_134_1,
        };
        // D s_134_4: write-var return_value <= s_134_3
        fn_state.return_value = s_134_3;
        // N s_134_5: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_135_0: const #() : ()
        let s_135_0: () = ();
        // S s_135_1: call EL2Enabled(s_135_0)
        let s_135_1: bool = EL2Enabled(state, tracer, s_135_0);
        // D s_135_2: write-var gs#19872 <= s_135_1
        fn_state.gs_19872 = s_135_1;
        // N s_135_3: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // S s_136_1: call Bit(s_136_0)
        let s_136_1: bool = Bit(state, tracer, s_136_0);
        // C s_136_2: const #7s : i
        let s_136_2: i128 = 7;
        // D s_136_3: read-var new_desc:u128
        let s_136_3: u128 = fn_state.new_desc;
        // D s_136_4: cast zx s_136_3 -> bv
        let s_136_4: Bits = Bits::new(s_136_3 as u128, 128u16);
        // C s_136_5: const #1u : u64
        let s_136_5: u64 = 1;
        // D s_136_6: bit-insert s_136_4 s_136_4 s_136_2 s_136_5
        let s_136_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_136_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_136_4.length(),
            );
            (s_136_4 & mask) | (s_136_4 << s_136_2)
        };
        // D s_136_7: cast reint s_136_6 -> u128
        let s_136_7: u128 = (s_136_6.value() as u128);
        // D s_136_8: write-var new_desc <= s_136_7
        fn_state.new_desc = s_136_7;
        // N s_136_9: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_137_0: read-var accdesc.1:struct
        let s_137_0: u32 = fn_state.accdesc._1;
        // C s_137_1: const #8u : u32
        let s_137_1: u32 = 8;
        // D s_137_2: cmp-eq s_137_0 s_137_1
        let s_137_2: bool = ((s_137_0) == (s_137_1));
        // N s_137_3: branch s_137_2 b143 b138
        if s_137_2 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_138_0: read-var accdesc.1:struct
        let s_138_0: u32 = fn_state.accdesc._1;
        // C s_138_1: const #5u : u32
        let s_138_1: u32 = 5;
        // D s_138_2: cmp-eq s_138_0 s_138_1
        let s_138_2: bool = ((s_138_0) == (s_138_1));
        // N s_138_3: branch s_138_2 b142 b139
        if s_138_2 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_139_0: read-var accdesc.1:struct
        let s_139_0: u32 = fn_state.accdesc._1;
        // C s_139_1: const #6u : u32
        let s_139_1: u32 = 6;
        // D s_139_2: cmp-eq s_139_0 s_139_1
        let s_139_2: bool = ((s_139_0) == (s_139_1));
        // D s_139_3: write-var gs#19868 <= s_139_2
        fn_state.gs_19868 = s_139_2;
        // N s_139_4: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_140_0: read-var gs#19868:u8
        let s_140_0: bool = fn_state.gs_19868;
        // D s_140_1: write-var gs#19869 <= s_140_0
        fn_state.gs_19869 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_141_0: read-var gs#19869:u8
        let s_141_0: bool = fn_state.gs_19869;
        // D s_141_1: not s_141_0
        let s_141_1: bool = !s_141_0;
        // D s_141_2: write-var gs#19870 <= s_141_1
        fn_state.gs_19870 = s_141_1;
        // N s_141_3: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_142_0: const #1u : u8
        let s_142_0: bool = true;
        // D s_142_1: write-var gs#19868 <= s_142_0
        fn_state.gs_19868 = s_142_0;
        // N s_142_2: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_143_0: const #1u : u8
        let s_143_0: bool = true;
        // D s_143_1: write-var gs#19869 <= s_143_0
        fn_state.gs_19869 = s_143_0;
        // N s_143_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_144_0: read-var accdesc.32:struct
        let s_144_0: bool = fn_state.accdesc._32;
        // D s_144_1: write-var gs#19867 <= s_144_0
        fn_state.gs_19867 = s_144_0;
        // N s_144_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_145_0: read-var walkparams.24:struct
        let s_145_0: bool = fn_state.walkparams._24;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 1u16);
        // C s_145_2: const #1u : u8
        let s_145_2: bool = true;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_4: cmp-eq s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) == (s_145_3));
        // N s_145_5: branch s_145_4 b148 b146
        if s_145_4 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_146_0: const #51s : i
        let s_146_0: i128 = 51;
        // D s_146_1: read-var descriptor:u128
        let s_146_1: u128 = fn_state.descriptor;
        // D s_146_2: cast zx s_146_1 -> bv
        let s_146_2: Bits = Bits::new(s_146_1 as u128, 128u16);
        // C s_146_3: const #1u : u64
        let s_146_3: u64 = 1;
        // D s_146_4: bit-extract s_146_2 s_146_0 s_146_3
        let s_146_4: Bits = (Bits::new(
            ((s_146_2) >> (s_146_0)).value(),
            u16::try_from(s_146_3).unwrap(),
        ));
        // D s_146_5: cast reint s_146_4 -> u8
        let s_146_5: bool = ((s_146_4.value()) != 0);
        // C s_146_6: const #0s : i
        let s_146_6: i128 = 0;
        // C s_146_7: const #0u : u64
        let s_146_7: u64 = 0;
        // D s_146_8: cast zx s_146_5 -> u64
        let s_146_8: u64 = (s_146_5 as u64);
        // C s_146_9: const #1u : u64
        let s_146_9: u64 = 1;
        // D s_146_10: and s_146_8 s_146_9
        let s_146_10: u64 = ((s_146_8) & (s_146_9));
        // D s_146_11: cmp-eq s_146_10 s_146_9
        let s_146_11: bool = ((s_146_10) == (s_146_9));
        // D s_146_12: lsl s_146_8 s_146_6
        let s_146_12: u64 = s_146_8 << s_146_6;
        // D s_146_13: or s_146_7 s_146_12
        let s_146_13: u64 = ((s_146_7) | (s_146_12));
        // D s_146_14: cmpl s_146_12
        let s_146_14: u64 = !s_146_12;
        // D s_146_15: and s_146_7 s_146_14
        let s_146_15: u64 = ((s_146_7) & (s_146_14));
        // D s_146_16: select s_146_11 s_146_13 s_146_15
        let s_146_16: u64 = if s_146_11 { s_146_13 } else { s_146_15 };
        // D s_146_17: cast trunc s_146_16 -> u8
        let s_146_17: bool = ((s_146_16) != 0);
        // D s_146_18: cast zx s_146_17 -> bv
        let s_146_18: Bits = Bits::new(s_146_17 as u128, 1u16);
        // C s_146_19: const #1u : u8
        let s_146_19: bool = true;
        // C s_146_20: cast zx s_146_19 -> bv
        let s_146_20: Bits = Bits::new(s_146_19 as u128, 1u16);
        // D s_146_21: cmp-eq s_146_18 s_146_20
        let s_146_21: bool = ((s_146_18) == (s_146_20));
        // D s_146_22: write-var gs#19865 <= s_146_21
        fn_state.gs_19865 = s_146_21;
        // N s_146_23: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_147_0: read-var gs#19865:u8
        let s_147_0: bool = fn_state.gs_19865;
        // D s_147_1: write-var gs#19866 <= s_147_0
        fn_state.gs_19866 = s_147_0;
        // N s_147_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_148_0: const #1u : u8
        let s_148_0: bool = true;
        // D s_148_1: write-var gs#19865 <= s_148_0
        fn_state.gs_19865 = s_148_0;
        // N s_148_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_149_0: read-var walkparams.14:struct
        let s_149_0: bool = fn_state.walkparams._14;
        // D s_149_1: cast zx s_149_0 -> bv
        let s_149_1: Bits = Bits::new(s_149_0 as u128, 1u16);
        // C s_149_2: const #1u : u8
        let s_149_2: bool = true;
        // C s_149_3: cast zx s_149_2 -> bv
        let s_149_3: Bits = Bits::new(s_149_2 as u128, 1u16);
        // D s_149_4: cmp-eq s_149_1 s_149_3
        let s_149_4: bool = ((s_149_1) == (s_149_3));
        // D s_149_5: write-var gs#19862 <= s_149_4
        fn_state.gs_19862 = s_149_4;
        // N s_149_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_150_0: read-var walkparams.12:struct
        let s_150_0: bool = fn_state.walkparams._12;
        // D s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 1u16);
        // C s_150_2: const #1u : u8
        let s_150_2: bool = true;
        // C s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 1u16);
        // D s_150_4: cmp-eq s_150_1 s_150_3
        let s_150_4: bool = ((s_150_1) == (s_150_3));
        // D s_150_5: write-var gs#19861 <= s_150_4
        fn_state.gs_19861 = s_150_4;
        // N s_150_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_151_0: const #1u : u8
        let s_151_0: bool = true;
        // S s_151_1: call Bit(s_151_0)
        let s_151_1: bool = Bit(state, tracer, s_151_0);
        // C s_151_2: const #10s : i
        let s_151_2: i128 = 10;
        // D s_151_3: read-var new_desc:u128
        let s_151_3: u128 = fn_state.new_desc;
        // D s_151_4: cast zx s_151_3 -> bv
        let s_151_4: Bits = Bits::new(s_151_3 as u128, 128u16);
        // C s_151_5: const #1u : u64
        let s_151_5: u64 = 1;
        // D s_151_6: bit-insert s_151_4 s_151_4 s_151_2 s_151_5
        let s_151_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_151_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_151_4.length(),
            );
            (s_151_4 & mask) | (s_151_4 << s_151_2)
        };
        // D s_151_7: cast reint s_151_6 -> u128
        let s_151_7: u128 = (s_151_6.value() as u128);
        // D s_151_8: write-var new_desc <= s_151_7
        fn_state.new_desc = s_151_7;
        // N s_151_9: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_152_0: read-var fault:struct
        let s_152_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_152_1: call AArch64_SettingAccessFlagPermitted(s_152_0)
        let s_152_1: bool = AArch64_SettingAccessFlagPermitted(state, tracer, s_152_0);
        // D s_152_2: write-var gs#19860 <= s_152_1
        fn_state.gs_19860 = s_152_1;
        // N s_152_3: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_153_0: read-var fault:struct
        let s_153_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_153_1: read-var regime:u32
        let s_153_1: u32 = fn_state.regime;
        // D s_153_2: read-var walkstate:struct
        let s_153_2: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_153_3: read-var walkparams:struct
        let s_153_3: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_153_4: read-var accdesc:struct
        let s_153_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_153_5: call AArch64_S1CheckPermissions(s_153_0, s_153_1, s_153_2, s_153_3, s_153_4)
        let s_153_5: ProductType1d757adad216cdef = AArch64_S1CheckPermissions(
            state,
            tracer,
            s_153_0,
            s_153_1,
            s_153_2,
            s_153_3,
            s_153_4,
        );
        // D s_153_6: write-var fault <= s_153_5
        fn_state.fault = s_153_5;
        // N s_153_7: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_154_0: const #2u : u32
        let s_154_0: u32 = 2;
        // D s_154_1: write-var fault.16 <= s_154_0
        fn_state.fault._16 = s_154_0;
        // N s_154_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_155_0: read-var walkstate.4:struct
        let s_155_0: bool = fn_state.walkstate._4;
        // D s_155_1: cast zx s_155_0 -> bv
        let s_155_1: Bits = Bits::new(s_155_0 as u128, 1u16);
        // C s_155_2: const #1u : u8
        let s_155_2: bool = true;
        // C s_155_3: cast zx s_155_2 -> bv
        let s_155_3: Bits = Bits::new(s_155_2 as u128, 1u16);
        // D s_155_4: cmp-eq s_155_1 s_155_3
        let s_155_4: bool = ((s_155_1) == (s_155_3));
        // D s_155_5: call SetInGuardedPage(s_155_4)
        let s_155_5: () = SetInGuardedPage(state, tracer, s_155_4);
        // N s_155_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_156_0: const #() : ()
        let s_156_0: () = ();
        // S s_156_1: call __UNKNOWN_AddressDescriptor(s_156_0)
        let s_156_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_156_0,
        );
        // D s_156_2: read-var fault:struct
        let s_156_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_156_3: create-product struct = ["s_156_2", "s_156_1"]
        let s_156_3: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_156_2,
            _1: s_156_1,
        };
        // D s_156_4: write-var return_value <= s_156_3
        fn_state.return_value = s_156_3;
        // N s_156_5: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_157_0: const #128s : i64
        let s_157_0: i64 = 128;
        // C s_157_1: cast zx s_157_0 -> i
        let s_157_1: i128 = (i128::try_from(s_157_0).unwrap());
        // D s_157_2: read-var fault:struct
        let s_157_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_157_3: read-var walkparams:struct
        let s_157_3: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_157_4: read-var va:u64
        let s_157_4: u64 = fn_state.va;
        // D s_157_5: read-var regime:u32
        let s_157_5: u32 = fn_state.regime;
        // D s_157_6: read-var accdesc:struct
        let s_157_6: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_157_7: call AArch64_S1Walk(s_157_2, s_157_3, s_157_4, s_157_5, s_157_6, s_157_1)
        let s_157_7: ProductType4b99944cd5e0b59d = AArch64_S1Walk(
            state,
            tracer,
            s_157_2,
            s_157_3,
            s_157_4,
            s_157_5,
            s_157_6,
            s_157_1,
        );
        // D s_157_8: write-var gs#446696 <= s_157_7
        fn_state.gs_446696 = s_157_7;
        // D s_157_9: read-var gs#446696.0:struct
        let s_157_9: ProductType1d757adad216cdef = fn_state.gs_446696._0;
        // D s_157_10: read-var gs#446696.1:struct
        let s_157_10: ProductTypece7c66ccb2cab13e = fn_state.gs_446696._1;
        // D s_157_11: read-var gs#446696.2:struct
        let s_157_11: ProductType96e7acababe246a1 = fn_state.gs_446696._2;
        // D s_157_12: read-var gs#446696.3:struct
        let s_157_12: Bits = fn_state.gs_446696._3;
        // D s_157_13: cast reint s_157_12 -> u128
        let s_157_13: u128 = (s_157_12.value() as u128);
        // D s_157_14: write-var fault <= s_157_9
        fn_state.fault = s_157_9;
        // D s_157_15: write-var descipaddr <= s_157_10
        fn_state.descipaddr = s_157_10;
        // D s_157_16: write-var walkstate <= s_157_11
        fn_state.walkstate = s_157_11;
        // D s_157_17: write-var descriptor <= s_157_13
        fn_state.descriptor = s_157_13;
        // N s_157_18: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_158_0: const #6u : u32
        let s_158_0: u32 = 6;
        // D s_158_1: write-var fault.16 <= s_158_0
        fn_state.fault._16 = s_158_0;
        // C s_158_2: const #0s : i
        let s_158_2: i128 = 0;
        // D s_158_3: write-var fault.9 <= s_158_2
        fn_state.fault._9 = s_158_2;
        // C s_158_4: const #() : ()
        let s_158_4: () = ();
        // S s_158_5: call __UNKNOWN_AddressDescriptor(s_158_4)
        let s_158_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_158_4,
        );
        // D s_158_6: read-var fault:struct
        let s_158_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_158_7: create-product struct = ["s_158_6", "s_158_5"]
        let s_158_7: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_158_6,
            _1: s_158_5,
        };
        // D s_158_8: write-var return_value <= s_158_7
        fn_state.return_value = s_158_7;
        // N s_158_9: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_159_0: read-var accdesc.17:struct
        let s_159_0: bool = fn_state.accdesc._17;
        // N s_159_1: branch s_159_0 b171 b160
        if s_159_0 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_160_0: const #0u : u8
        let s_160_0: bool = false;
        // D s_160_1: write-var gs#19825 <= s_160_0
        fn_state.gs_19825 = s_160_0;
        // N s_160_2: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_161_0: read-var gs#19825:u8
        let s_161_0: bool = fn_state.gs_19825;
        // N s_161_1: branch s_161_0 b170 b162
        if s_161_0 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_162(state, tracer, fn_state);
        };
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_162_0: read-var accdesc.11:struct
        let s_162_0: bool = fn_state.accdesc._11;
        // N s_162_1: branch s_162_0 b169 b163
        if s_162_0 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_163_0: const #0u : u8
        let s_163_0: bool = false;
        // D s_163_1: write-var gs#19826 <= s_163_0
        fn_state.gs_19826 = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_164_0: read-var gs#19826:u8
        let s_164_0: bool = fn_state.gs_19826;
        // N s_164_1: branch s_164_0 b168 b165
        if s_164_0 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_165_0: const #0u : u8
        let s_165_0: bool = false;
        // D s_165_1: write-var gs#19827 <= s_165_0
        fn_state.gs_19827 = s_165_0;
        // N s_165_2: jump b166
        return block_166(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_166_0: read-var gs#19827:u8
        let s_166_0: bool = fn_state.gs_19827;
        // D s_166_1: write-var gs#19828 <= s_166_0
        fn_state.gs_19828 = s_166_0;
        // N s_166_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_167_0: read-var gs#19828:u8
        let s_167_0: bool = fn_state.gs_19828;
        // D s_167_1: write-var gs#19829 <= s_167_0
        fn_state.gs_19829 = s_167_0;
        // N s_167_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_168_0: read-var accdesc.7:struct
        let s_168_0: bool = fn_state.accdesc._7;
        // D s_168_1: not s_168_0
        let s_168_1: bool = !s_168_0;
        // D s_168_2: write-var gs#19827 <= s_168_1
        fn_state.gs_19827 = s_168_1;
        // N s_168_3: jump b166
        return block_166(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_169_0: read-var accdesc.10:struct
        let s_169_0: bool = fn_state.accdesc._10;
        // D s_169_1: not s_169_0
        let s_169_1: bool = !s_169_0;
        // D s_169_2: write-var gs#19826 <= s_169_1
        fn_state.gs_19826 = s_169_1;
        // N s_169_3: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_170_0: const #1u : u8
        let s_170_0: bool = true;
        // D s_170_1: write-var gs#19828 <= s_170_0
        fn_state.gs_19828 = s_170_0;
        // N s_170_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_171_0: read-var accdesc.7:struct
        let s_171_0: bool = fn_state.accdesc._7;
        // D s_171_1: write-var gs#19825 <= s_171_0
        fn_state.gs_19825 = s_171_0;
        // N s_171_2: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_172_0: read-var walkparams.20:struct
        let s_172_0: bool = fn_state.walkparams._20;
        // D s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 1u16);
        // C s_172_2: const #1u : u8
        let s_172_2: bool = true;
        // C s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 1u16);
        // D s_172_4: cmp-eq s_172_1 s_172_3
        let s_172_4: bool = ((s_172_1) == (s_172_3));
        // D s_172_5: write-var gs#19824 <= s_172_4
        fn_state.gs_19824 = s_172_4;
        // N s_172_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_173_0: read-var accdesc.8:struct
        let s_173_0: u8 = fn_state.accdesc._8;
        // D s_173_1: cast zx s_173_0 -> bv
        let s_173_1: Bits = Bits::new(s_173_0 as u128, 2u16);
        // C s_173_2: const #448u : u32
        let s_173_2: u32 = 448;
        // D s_173_3: read-reg s_173_2:u8
        let s_173_3: u8 = {
            let value = state.read_register::<u8>(s_173_2 as isize);
            tracer.read_register(s_173_2 as isize, value);
            value
        };
        // D s_173_4: cast zx s_173_3 -> bv
        let s_173_4: Bits = Bits::new(s_173_3 as u128, 2u16);
        // D s_173_5: cmp-eq s_173_1 s_173_4
        let s_173_5: bool = ((s_173_1) == (s_173_4));
        // D s_173_6: write-var gs#19823 <= s_173_5
        fn_state.gs_19823 = s_173_5;
        // N s_173_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_174_0: const #6u : u32
        let s_174_0: u32 = 6;
        // D s_174_1: write-var fault.16 <= s_174_0
        fn_state.fault._16 = s_174_0;
        // C s_174_2: const #0s : i
        let s_174_2: i128 = 0;
        // D s_174_3: write-var fault.9 <= s_174_2
        fn_state.fault._9 = s_174_2;
        // C s_174_4: const #() : ()
        let s_174_4: () = ();
        // S s_174_5: call __UNKNOWN_AddressDescriptor(s_174_4)
        let s_174_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_174_4,
        );
        // D s_174_6: read-var fault:struct
        let s_174_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_174_7: create-product struct = ["s_174_6", "s_174_5"]
        let s_174_7: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_174_6,
            _1: s_174_5,
        };
        // D s_174_8: write-var return_value <= s_174_7
        fn_state.return_value = s_174_7;
        // N s_174_9: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_175_0: read-var accdesc.30:struct
        let s_175_0: bool = fn_state.accdesc._30;
        // D s_175_1: write-var gs#19822 <= s_175_0
        fn_state.gs_19822 = s_175_0;
        // N s_175_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_176_0: read-var walkparams.20:struct
        let s_176_0: bool = fn_state.walkparams._20;
        // D s_176_1: cast zx s_176_0 -> bv
        let s_176_1: Bits = Bits::new(s_176_0 as u128, 1u16);
        // C s_176_2: const #1u : u8
        let s_176_2: bool = true;
        // C s_176_3: cast zx s_176_2 -> bv
        let s_176_3: Bits = Bits::new(s_176_2 as u128, 1u16);
        // D s_176_4: cmp-eq s_176_1 s_176_3
        let s_176_4: bool = ((s_176_1) == (s_176_3));
        // D s_176_5: write-var gs#19821 <= s_176_4
        fn_state.gs_19821 = s_176_4;
        // N s_176_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_177_0: read-var accdesc.8:struct
        let s_177_0: u8 = fn_state.accdesc._8;
        // D s_177_1: cast zx s_177_0 -> bv
        let s_177_1: Bits = Bits::new(s_177_0 as u128, 2u16);
        // C s_177_2: const #448u : u32
        let s_177_2: u32 = 448;
        // D s_177_3: read-reg s_177_2:u8
        let s_177_3: u8 = {
            let value = state.read_register::<u8>(s_177_2 as isize);
            tracer.read_register(s_177_2 as isize, value);
            value
        };
        // D s_177_4: cast zx s_177_3 -> bv
        let s_177_4: Bits = Bits::new(s_177_3 as u128, 2u16);
        // D s_177_5: cmp-eq s_177_1 s_177_4
        let s_177_5: bool = ((s_177_1) == (s_177_4));
        // D s_177_6: write-var gs#19820 <= s_177_5
        fn_state.gs_19820 = s_177_5;
        // N s_177_7: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_178_0: const #6u : u32
        let s_178_0: u32 = 6;
        // D s_178_1: write-var fault.16 <= s_178_0
        fn_state.fault._16 = s_178_0;
        // C s_178_2: const #0s : i
        let s_178_2: i128 = 0;
        // D s_178_3: write-var fault.9 <= s_178_2
        fn_state.fault._9 = s_178_2;
        // C s_178_4: const #() : ()
        let s_178_4: () = ();
        // S s_178_5: call __UNKNOWN_AddressDescriptor(s_178_4)
        let s_178_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_178_4,
        );
        // D s_178_6: read-var fault:struct
        let s_178_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_178_7: create-product struct = ["s_178_6", "s_178_5"]
        let s_178_7: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_178_6,
            _1: s_178_5,
        };
        // D s_178_8: write-var return_value <= s_178_7
        fn_state.return_value = s_178_7;
        // N s_178_9: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_179_0: read-var walkparams.8:struct
        let s_179_0: bool = fn_state.walkparams._8;
        // D s_179_1: cast zx s_179_0 -> bv
        let s_179_1: Bits = Bits::new(s_179_0 as u128, 1u16);
        // C s_179_2: const #1u : u8
        let s_179_2: bool = true;
        // C s_179_3: cast zx s_179_2 -> bv
        let s_179_3: Bits = Bits::new(s_179_2 as u128, 1u16);
        // D s_179_4: cmp-eq s_179_1 s_179_3
        let s_179_4: bool = ((s_179_1) == (s_179_3));
        // D s_179_5: write-var gs#19819 <= s_179_4
        fn_state.gs_19819 = s_179_4;
        // N s_179_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_180_0: const #6u : u32
        let s_180_0: u32 = 6;
        // D s_180_1: write-var fault.16 <= s_180_0
        fn_state.fault._16 = s_180_0;
        // C s_180_2: const #0s : i
        let s_180_2: i128 = 0;
        // D s_180_3: write-var fault.9 <= s_180_2
        fn_state.fault._9 = s_180_2;
        // C s_180_4: const #() : ()
        let s_180_4: () = ();
        // S s_180_5: call __UNKNOWN_AddressDescriptor(s_180_4)
        let s_180_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_180_4,
        );
        // D s_180_6: read-var fault:struct
        let s_180_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_180_7: create-product struct = ["s_180_6", "s_180_5"]
        let s_180_7: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_180_6,
            _1: s_180_5,
        };
        // D s_180_8: write-var return_value <= s_180_7
        fn_state.return_value = s_180_7;
        // N s_180_9: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_181_0: const #5s : i
        let s_181_0: i128 = 5;
        // C s_181_1: const #0s : i
        let s_181_1: i128 = 0;
        // D s_181_2: read-var s1maxtxsz:i
        let s_181_2: i128 = fn_state.s1maxtxsz;
        // D s_181_3: call integer_subrange(s_181_2, s_181_0, s_181_1)
        let s_181_3: Bits = integer_subrange(state, tracer, s_181_2, s_181_0, s_181_1);
        // D s_181_4: cast reint s_181_3 -> u8
        let s_181_4: u8 = (s_181_3.value() as u8);
        // D s_181_5: write-var walkparams.37 <= s_181_4
        fn_state.walkparams._37 = s_181_4;
        // N s_181_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_182_0: const #5s : i
        let s_182_0: i128 = 5;
        // C s_182_1: const #0s : i
        let s_182_1: i128 = 0;
        // D s_182_2: read-var s1mintxsz:i
        let s_182_2: i128 = fn_state.s1mintxsz;
        // D s_182_3: call integer_subrange(s_182_2, s_182_0, s_182_1)
        let s_182_3: Bits = integer_subrange(state, tracer, s_182_2, s_182_0, s_182_1);
        // D s_182_4: cast reint s_182_3 -> u8
        let s_182_4: u8 = (s_182_3.value() as u8);
        // D s_182_5: write-var walkparams.37 <= s_182_4
        fn_state.walkparams._37 = s_182_4;
        // N s_182_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_183_0: const #6u : u32
        let s_183_0: u32 = 6;
        // D s_183_1: write-var fault.16 <= s_183_0
        fn_state.fault._16 = s_183_0;
        // C s_183_2: const #0s : i
        let s_183_2: i128 = 0;
        // D s_183_3: write-var fault.9 <= s_183_2
        fn_state.fault._9 = s_183_2;
        // C s_183_4: const #() : ()
        let s_183_4: () = ();
        // S s_183_5: call __UNKNOWN_AddressDescriptor(s_183_4)
        let s_183_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_183_4,
        );
        // D s_183_6: read-var fault:struct
        let s_183_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_183_7: create-product struct = ["s_183_6", "s_183_5"]
        let s_183_7: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_183_6,
            _1: s_183_5,
        };
        // D s_183_8: write-var return_value <= s_183_7
        fn_state.return_value = s_183_7;
        // N s_183_9: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_184_0: read-var fault:struct
        let s_184_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_184_1: read-var regime:u32
        let s_184_1: u32 = fn_state.regime;
        // D s_184_2: read-var va:u64
        let s_184_2: u64 = fn_state.va;
        // D s_184_3: read-var accdesc:struct
        let s_184_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_184_4: read-var aligned:u8
        let s_184_4: bool = fn_state.aligned;
        // D s_184_5: call AArch64_S1DisabledOutput(s_184_0, s_184_1, s_184_2, s_184_3, s_184_4)
        let s_184_5: ProductTypedc31059ca7e2391c = AArch64_S1DisabledOutput(
            state,
            tracer,
            s_184_0,
            s_184_1,
            s_184_2,
            s_184_3,
            s_184_4,
        );
        // D s_184_6: write-var return_value <= s_184_5
        fn_state.return_value = s_184_5;
        // N s_184_7: jump b83
        return block_83(state, tracer, fn_state);
    }
}
