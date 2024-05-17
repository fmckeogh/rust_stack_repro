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
use AArch32_S2HasAlignmentFault::*;
use u_get_HCR2_Type_CD::*;
use StageOA::*;
use AArch32_IPAIsOutOfRange::*;
use AArch32_GetS2TTWParams::*;
use AArch32_S2HasPermissionsFault::*;
use CreateAddressDescriptor::*;
use AArch64_S2Translate::*;
use ELStateUsingAArch32::*;
use S2CombineS1MemAttrs::*;
use IsZero::*;
use u_get_HCR2_Type_ID::*;
use u__UNKNOWN_AddressDescriptor::*;
use NormalNCMemAttr::*;
use AArch32_S2Walk::*;
use HCR2_read::*;
use common::*;
pub fn AArch32_S2Translate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    ipa: ProductTypece7c66ccb2cab13e,
    s1level: SumTypebf36e919d71ba1d6,
    aligned: bool,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductTypedc31059ca7e2391c {
    #[derive(Default)]
    struct FunctionState {
        ga_21772: ProductTypef170cab34335b70c,
        ga_21796: ProductTypeda0231e9dc169f81,
        gs_28170: bool,
        gs_28171: bool,
        s2_memattrs: ProductTypef170cab34335b70c,
        fault: ProductType1d757adad216cdef,
        ga_21767: ProductTypef170cab34335b70c,
        ga_21784: ProductTypef170cab34335b70c,
        gs_28173: bool,
        walkstate: ProductType96e7acababe246a1,
        gs_28169: bool,
        ga_21741: ProductTypeda0231e9dc169f81,
        ga_21792: ProductTypef170cab34335b70c,
        gs_28175: bool,
        gs_28172: bool,
        return_value: ProductTypedc31059ca7e2391c,
        walkparams: ProductTypeb05ce25a107f0c5e,
        ga_21754: ProductTypeda0231e9dc169f81,
        gs_28174: bool,
        ga_21776: ProductTypef170cab34335b70c,
        ga_21760: ProductType201519a0f62623dc,
        fault_in: ProductType1d757adad216cdef,
        ipa: ProductTypece7c66ccb2cab13e,
        s1level: SumTypebf36e919d71ba1d6,
        aligned: bool,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        fault_in,
        ipa,
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
        // D s_0_0: read-var fault_in:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_1: write-var fault <= s_0_0
        fn_state.fault = s_0_0;
        // D s_0_2: read-var ipa.3:struct
        let s_0_2: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_0_3: write-var ga#21741 <= s_0_2
        fn_state.ga_21741 = s_0_2;
        // D s_0_4: read-var ga#21741.0:struct
        let s_0_4: u64 = fn_state.ga_21741._0;
        // C s_0_5: const #40s : i
        let s_0_5: i128 = 40;
        // D s_0_6: cast zx s_0_4 -> bv
        let s_0_6: Bits = Bits::new(s_0_4 as u128, 56u16);
        // C s_0_7: const #1s : i64
        let s_0_7: i64 = 1;
        // C s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // C s_0_9: const #15s : i
        let s_0_9: i128 = 15;
        // C s_0_10: add s_0_9 s_0_8
        let s_0_10: i128 = (s_0_9 + s_0_8);
        // D s_0_11: bit-extract s_0_6 s_0_5 s_0_10
        let s_0_11: Bits = (Bits::new(
            ((s_0_6) >> (s_0_5)).value(),
            u16::try_from(s_0_10).unwrap(),
        ));
        // D s_0_12: cast reint s_0_11 -> u16
        let s_0_12: u16 = (s_0_11.value() as u16);
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 16u16);
        // D s_0_14: call IsZero(s_0_13)
        let s_0_14: bool = IsZero(state, tracer, s_0_13);
        // N s_0_15: assert s_0_14
        let s_0_15: () = assert!(s_0_14);
        // D s_0_16: read-var accdesc.25:struct
        let s_0_16: u32 = fn_state.accdesc._25;
        // C s_0_17: const #3u : u32
        let s_0_17: u32 = 3;
        // D s_0_18: cmp-eq s_0_16 s_0_17
        let s_0_18: bool = ((s_0_16) == (s_0_17));
        // C s_0_19: const #432u : u32
        let s_0_19: u32 = 432;
        // D s_0_20: read-reg s_0_19:u8
        let s_0_20: u8 = {
            let value = state.read_register::<u8>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call ELStateUsingAArch32(s_0_20, s_0_18)
        let s_0_21: bool = ELStateUsingAArch32(state, tracer, s_0_20, s_0_18);
        // D s_0_22: not s_0_21
        let s_0_22: bool = !s_0_21;
        // N s_0_23: branch s_0_22 b38 b1
        if s_0_22 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
        // D s_1_1: write-var fault.16 <= s_1_0
        fn_state.fault._16 = s_1_0;
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // D s_1_3: write-var fault.15 <= s_1_2
        fn_state.fault._15 = s_1_2;
        // D s_1_4: read-var accdesc.1:struct
        let s_1_4: u32 = fn_state.accdesc._1;
        // C s_1_5: const #13u : u32
        let s_1_5: u32 = 13;
        // D s_1_6: cmp-eq s_1_4 s_1_5
        let s_1_6: bool = ((s_1_4) == (s_1_5));
        // D s_1_7: write-var fault.14 <= s_1_6
        fn_state.fault._14 = s_1_6;
        // D s_1_8: read-var ipa.3:struct
        let s_1_8: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_1_9: write-var fault.8 <= s_1_8
        fn_state.fault._8 = s_1_8;
        // C s_1_10: const #() : ()
        let s_1_10: () = ();
        // S s_1_11: call AArch32_GetS2TTWParams(s_1_10)
        let s_1_11: ProductTypeb05ce25a107f0c5e = AArch32_GetS2TTWParams(
            state,
            tracer,
            s_1_10,
        );
        // D s_1_12: write-var walkparams <= s_1_11
        fn_state.walkparams = s_1_11;
        // D s_1_13: read-var walkparams.30:struct
        let s_1_13: bool = fn_state.walkparams._30;
        // D s_1_14: cast zx s_1_13 -> bv
        let s_1_14: Bits = Bits::new(s_1_13 as u128, 1u16);
        // C s_1_15: const #0u : u8
        let s_1_15: bool = false;
        // C s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 1u16);
        // D s_1_17: cmp-eq s_1_14 s_1_16
        let s_1_17: bool = ((s_1_14) == (s_1_16));
        // N s_1_18: branch s_1_17 b37 b2
        if s_1_17 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_2_0: read-var ipa.3:struct
        let s_2_0: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_2_1: write-var ga#21754 <= s_2_0
        fn_state.ga_21754 = s_2_0;
        // D s_2_2: read-var ga#21754.0:struct
        let s_2_2: u64 = fn_state.ga_21754._0;
        // C s_2_3: const #0s : i
        let s_2_3: i128 = 0;
        // D s_2_4: cast zx s_2_2 -> bv
        let s_2_4: Bits = Bits::new(s_2_2 as u128, 56u16);
        // C s_2_5: const #1s : i64
        let s_2_5: i64 = 1;
        // C s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // C s_2_7: const #39s : i
        let s_2_7: i128 = 39;
        // C s_2_8: add s_2_7 s_2_6
        let s_2_8: i128 = (s_2_7 + s_2_6);
        // D s_2_9: bit-extract s_2_4 s_2_3 s_2_8
        let s_2_9: Bits = (Bits::new(
            ((s_2_4) >> (s_2_3)).value(),
            u16::try_from(s_2_8).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u40
        let s_2_10: u64 = (s_2_9.value() as u64);
        // D s_2_11: read-var walkparams:struct
        let s_2_11: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_2_12: call AArch32_IPAIsOutOfRange(s_2_11, s_2_10)
        let s_2_12: bool = AArch32_IPAIsOutOfRange(state, tracer, s_2_11, s_2_10);
        // N s_2_13: branch s_2_12 b36 b3
        if s_2_12 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_3_0: read-var fault:struct
        let s_3_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_3_1: read-var walkparams:struct
        let s_3_1: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_3_2: read-var s1level:enum
        let s_3_2: SumTypebf36e919d71ba1d6 = fn_state.s1level;
        // D s_3_3: read-var accdesc:struct
        let s_3_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_3_4: read-var ipa:struct
        let s_3_4: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_3_5: call AArch32_S2Walk(s_3_0, s_3_1, s_3_2, s_3_3, s_3_4)
        let s_3_5: ProductType201519a0f62623dc = AArch32_S2Walk(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_2,
            s_3_3,
            s_3_4,
        );
        // D s_3_6: write-var ga#21760 <= s_3_5
        fn_state.ga_21760 = s_3_5;
        // D s_3_7: read-var ga#21760.0:struct
        let s_3_7: ProductType1d757adad216cdef = fn_state.ga_21760._0;
        // D s_3_8: read-var ga#21760.1:struct
        let s_3_8: ProductType96e7acababe246a1 = fn_state.ga_21760._1;
        // D s_3_9: write-var fault <= s_3_7
        fn_state.fault = s_3_7;
        // D s_3_10: write-var walkstate <= s_3_8
        fn_state.walkstate = s_3_8;
        // D s_3_11: read-var fault.16:struct
        let s_3_11: u32 = fn_state.fault._16;
        // C s_3_12: const #0u : u32
        let s_3_12: u32 = 0;
        // D s_3_13: cmp-eq s_3_11 s_3_12
        let s_3_13: bool = ((s_3_11) == (s_3_12));
        // N s_3_14: branch s_3_13 b35 b4
        if s_3_13 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_4_0: read-var walkstate.7:struct
        let s_4_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_4_1: read-var accdesc:struct
        let s_4_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_4_2: read-var aligned:u8
        let s_4_2: bool = fn_state.aligned;
        // D s_4_3: call AArch32_S2HasAlignmentFault(s_4_1, s_4_2, s_4_0)
        let s_4_3: bool = AArch32_S2HasAlignmentFault(
            state,
            tracer,
            s_4_1,
            s_4_2,
            s_4_0,
        );
        // N s_4_4: branch s_4_3 b34 b5
        if s_4_3 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_5_0: read-var walkstate.9:struct
        let s_5_0: ProductTypebf05c51f33174538 = fn_state.walkstate._9;
        // D s_5_1: read-var walkstate.7:struct
        let s_5_1: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_5_2: write-var ga#21767 <= s_5_1
        fn_state.ga_21767 = s_5_1;
        // D s_5_3: read-var ga#21767.2:struct
        let s_5_3: u32 = fn_state.ga_21767._2;
        // D s_5_4: read-var walkparams:struct
        let s_5_4: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_5_5: read-var accdesc:struct
        let s_5_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_5_6: call AArch32_S2HasPermissionsFault(s_5_4, s_5_0, s_5_3, s_5_5)
        let s_5_6: bool = AArch32_S2HasPermissionsFault(
            state,
            tracer,
            s_5_4,
            s_5_0,
            s_5_3,
            s_5_5,
        );
        // N s_5_7: branch s_5_6 b33 b6
        if s_5_6 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_7_0: read-var accdesc.1:struct
        let s_7_0: u32 = fn_state.accdesc._1;
        // C s_7_1: const #13u : u32
        let s_7_1: u32 = 13;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b32 b8
        if s_7_2 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#28169 <= s_8_0
        fn_state.gs_28169 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_9_0: read-var gs#28169:u8
        let s_9_0: bool = fn_state.gs_28169;
        // N s_9_1: branch s_9_0 b31 b10
        if s_9_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_10_0: read-var accdesc.1:struct
        let s_10_0: u32 = fn_state.accdesc._1;
        // C s_10_1: const #0u : u32
        let s_10_1: u32 = 0;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b27 b11
        if s_10_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#28171 <= s_11_0
        fn_state.gs_28171 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_12_0: read-var gs#28171:u8
        let s_12_0: bool = fn_state.gs_28171;
        // D s_12_1: write-var gs#28172 <= s_12_0
        fn_state.gs_28172 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_13_0: read-var gs#28172:u8
        let s_13_0: bool = fn_state.gs_28172;
        // N s_13_1: branch s_13_0 b26 b14
        if s_13_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_14_0: read-var accdesc.1:struct
        let s_14_0: u32 = fn_state.accdesc._1;
        // C s_14_1: const #0u : u32
        let s_14_1: u32 = 0;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // N s_14_3: branch s_14_2 b25 b15
        if s_14_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#28173 <= s_15_0
        fn_state.gs_28173 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_16_0: read-var gs#28173:u8
        let s_16_0: bool = fn_state.gs_28173;
        // N s_16_1: branch s_16_0 b24 b17
        if s_16_0 {
            return block_24(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#28174 <= s_17_0
        fn_state.gs_28174 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_18_0: read-var gs#28174:u8
        let s_18_0: bool = fn_state.gs_28174;
        // D s_18_1: write-var gs#28175 <= s_18_0
        fn_state.gs_28175 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_19_0: read-var gs#28175:u8
        let s_19_0: bool = fn_state.gs_28175;
        // N s_19_1: branch s_19_0 b23 b20
        if s_19_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_20_0: read-var walkstate.7:struct
        let s_20_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_20_1: write-var s2_memattrs <= s_20_0
        fn_state.s2_memattrs = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: read-var ipa.2:struct
        let s_21_1: ProductTypef170cab34335b70c = fn_state.ipa._2;
        // D s_21_2: read-var s2_memattrs:struct
        let s_21_2: ProductTypef170cab34335b70c = fn_state.s2_memattrs;
        // D s_21_3: call S2CombineS1MemAttrs(s_21_1, s_21_2, s_21_0)
        let s_21_3: ProductTypef170cab34335b70c = S2CombineS1MemAttrs(
            state,
            tracer,
            s_21_1,
            s_21_2,
            s_21_0,
        );
        // D s_21_4: read-var ipa.3:struct
        let s_21_4: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_21_5: write-var ga#21796 <= s_21_4
        fn_state.ga_21796 = s_21_4;
        // D s_21_6: read-var ga#21796.0:struct
        let s_21_6: u64 = fn_state.ga_21796._0;
        // C s_21_7: const #0s : i
        let s_21_7: i128 = 0;
        // D s_21_8: cast zx s_21_6 -> bv
        let s_21_8: Bits = Bits::new(s_21_6 as u128, 56u16);
        // C s_21_9: const #1s : i64
        let s_21_9: i64 = 1;
        // C s_21_10: cast zx s_21_9 -> i
        let s_21_10: i128 = (i128::try_from(s_21_9).unwrap());
        // C s_21_11: const #39s : i
        let s_21_11: i128 = 39;
        // C s_21_12: add s_21_11 s_21_10
        let s_21_12: i128 = (s_21_11 + s_21_10);
        // D s_21_13: bit-extract s_21_8 s_21_7 s_21_12
        let s_21_13: Bits = (Bits::new(
            ((s_21_8) >> (s_21_7)).value(),
            u16::try_from(s_21_12).unwrap(),
        ));
        // D s_21_14: cast reint s_21_13 -> u40
        let s_21_14: u64 = (s_21_13.value() as u64);
        // C s_21_15: const #64s : i
        let s_21_15: i128 = 64;
        // D s_21_16: cast zx s_21_14 -> bv
        let s_21_16: Bits = Bits::new(s_21_14 as u128, 40u16);
        // D s_21_17: bits-cast zx s_21_16 -> bv length s_21_15
        let s_21_17: Bits = s_21_16.zero_extend(s_21_15);
        // D s_21_18: cast reint s_21_17 -> u64
        let s_21_18: u64 = (s_21_17.value() as u64);
        // D s_21_19: read-var walkparams.2:struct
        let s_21_19: bool = fn_state.walkparams._2;
        // D s_21_20: read-var walkparams.26:struct
        let s_21_20: u32 = fn_state.walkparams._26;
        // D s_21_21: read-var walkstate:struct
        let s_21_21: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_21_22: call StageOA(s_21_18, s_21_19, s_21_20, s_21_21)
        let s_21_22: ProductTypeda0231e9dc169f81 = StageOA(
            state,
            tracer,
            s_21_18,
            s_21_19,
            s_21_20,
            s_21_21,
        );
        // D s_21_23: read-var ipa.7:struct
        let s_21_23: u64 = fn_state.ipa._7;
        // D s_21_24: call CreateAddressDescriptor(s_21_23, s_21_22, s_21_3)
        let s_21_24: ProductTypece7c66ccb2cab13e = CreateAddressDescriptor(
            state,
            tracer,
            s_21_23,
            s_21_22,
            s_21_3,
        );
        // D s_21_25: read-var fault:struct
        let s_21_25: ProductType1d757adad216cdef = fn_state.fault;
        // D s_21_26: create-product struct = ["s_21_25", "s_21_24"]
        let s_21_26: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_21_25,
            _1: s_21_24,
        };
        // D s_21_27: write-var return_value <= s_21_26
        fn_state.return_value = s_21_26;
        // N s_21_28: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_22_0: read-var return_value:struct
        let s_22_0: ProductTypedc31059ca7e2391c = fn_state.return_value;
        // N s_22_1: return s_22_0
        return s_22_0;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call NormalNCMemAttr(s_23_0)
        let s_23_1: ProductTypef170cab34335b70c = NormalNCMemAttr(state, tracer, s_23_0);
        // D s_23_2: write-var s2_memattrs <= s_23_1
        fn_state.s2_memattrs = s_23_1;
        // D s_23_3: read-var walkstate.7:struct
        let s_23_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_23_4: write-var ga#21792 <= s_23_3
        fn_state.ga_21792 = s_23_3;
        // D s_23_5: read-var ga#21792.7:struct
        let s_23_5: bool = fn_state.ga_21792._7;
        // D s_23_6: write-var s2_memattrs.7 <= s_23_5
        fn_state.s2_memattrs._7 = s_23_5;
        // N s_23_7: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HCR2_read(s_24_0)
        let s_24_1: ProductType700c18a878c5601b = HCR2_read(state, tracer, s_24_0);
        // S s_24_2: call _get_HCR2_Type_CD(s_24_1)
        let s_24_2: bool = u_get_HCR2_Type_CD(state, tracer, s_24_1);
        // S s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // S s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // D s_24_7: write-var gs#28174 <= s_24_6
        fn_state.gs_28174 = s_24_6;
        // N s_24_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_25_0: read-var walkstate.7:struct
        let s_25_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_25_1: write-var ga#21784 <= s_25_0
        fn_state.ga_21784 = s_25_0;
        // D s_25_2: read-var ga#21784.2:struct
        let s_25_2: u32 = fn_state.ga_21784._2;
        // C s_25_3: const #0u : u32
        let s_25_3: u32 = 0;
        // D s_25_4: cmp-eq s_25_2 s_25_3
        let s_25_4: bool = ((s_25_2) == (s_25_3));
        // D s_25_5: write-var gs#28173 <= s_25_4
        fn_state.gs_28173 = s_25_4;
        // N s_25_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#28175 <= s_26_0
        fn_state.gs_28175 = s_26_0;
        // N s_26_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_27_0: read-var walkstate.7:struct
        let s_27_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_27_1: write-var ga#21776 <= s_27_0
        fn_state.ga_21776 = s_27_0;
        // D s_27_2: read-var ga#21776.2:struct
        let s_27_2: u32 = fn_state.ga_21776._2;
        // C s_27_3: const #1u : u32
        let s_27_3: u32 = 1;
        // D s_27_4: cmp-eq s_27_2 s_27_3
        let s_27_4: bool = ((s_27_2) == (s_27_3));
        // N s_27_5: branch s_27_4 b30 b28
        if s_27_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call HCR2_read(s_28_0)
        let s_28_1: ProductType700c18a878c5601b = HCR2_read(state, tracer, s_28_0);
        // S s_28_2: call _get_HCR2_Type_ID(s_28_1)
        let s_28_2: bool = u_get_HCR2_Type_ID(state, tracer, s_28_1);
        // S s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // S s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#28170 <= s_28_6
        fn_state.gs_28170 = s_28_6;
        // N s_28_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_29_0: read-var gs#28170:u8
        let s_29_0: bool = fn_state.gs_28170;
        // D s_29_1: write-var gs#28171 <= s_29_0
        fn_state.gs_28171 = s_29_0;
        // N s_29_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#28170 <= s_30_0
        fn_state.gs_28170 = s_30_0;
        // N s_30_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#28172 <= s_31_0
        fn_state.gs_28172 = s_31_0;
        // N s_31_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_32_0: read-var walkstate.7:struct
        let s_32_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_32_1: write-var ga#21772 <= s_32_0
        fn_state.ga_21772 = s_32_0;
        // D s_32_2: read-var ga#21772.2:struct
        let s_32_2: u32 = fn_state.ga_21772._2;
        // C s_32_3: const #1u : u32
        let s_32_3: u32 = 1;
        // D s_32_4: cmp-eq s_32_2 s_32_3
        let s_32_4: bool = ((s_32_2) == (s_32_3));
        // D s_32_5: write-var gs#28169 <= s_32_4
        fn_state.gs_28169 = s_32_4;
        // N s_32_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_33_0: const #5u : u32
        let s_33_0: u32 = 5;
        // D s_33_1: write-var fault.16 <= s_33_0
        fn_state.fault._16 = s_33_0;
        // N s_33_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_34_0: const #2u : u32
        let s_34_0: u32 = 2;
        // D s_34_1: write-var fault.16 <= s_34_0
        fn_state.fault._16 = s_34_0;
        // N s_34_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call __UNKNOWN_AddressDescriptor(s_35_0)
        let s_35_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_35_0,
        );
        // D s_35_2: read-var fault:struct
        let s_35_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_35_3: create-product struct = ["s_35_2", "s_35_1"]
        let s_35_3: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_35_2,
            _1: s_35_1,
        };
        // D s_35_4: write-var return_value <= s_35_3
        fn_state.return_value = s_35_3;
        // N s_35_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_36_0: const #6u : u32
        let s_36_0: u32 = 6;
        // D s_36_1: write-var fault.16 <= s_36_0
        fn_state.fault._16 = s_36_0;
        // C s_36_2: const #1s : i
        let s_36_2: i128 = 1;
        // D s_36_3: write-var fault.9 <= s_36_2
        fn_state.fault._9 = s_36_2;
        // C s_36_4: const #() : ()
        let s_36_4: () = ();
        // S s_36_5: call __UNKNOWN_AddressDescriptor(s_36_4)
        let s_36_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_36_4,
        );
        // D s_36_6: read-var fault:struct
        let s_36_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_36_7: create-product struct = ["s_36_6", "s_36_5"]
        let s_36_7: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_36_6,
            _1: s_36_5,
        };
        // D s_36_8: write-var return_value <= s_36_7
        fn_state.return_value = s_36_7;
        // N s_36_9: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_37_0: read-var fault:struct
        let s_37_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_37_1: read-var ipa:struct
        let s_37_1: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_37_2: create-product struct = ["s_37_0", "s_37_1"]
        let s_37_2: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_37_0,
            _1: s_37_1,
        };
        // D s_37_3: write-var return_value <= s_37_2
        fn_state.return_value = s_37_2;
        // N s_37_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // C s_38_1: const #() : ()
        let s_38_1: () = ();
        // D s_38_2: create-sum enum = 0:"s_38_1"
        let s_38_2: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_0(s_38_1);
        // D s_38_3: read-var fault:struct
        let s_38_3: ProductType1d757adad216cdef = fn_state.fault;
        // D s_38_4: read-var ipa:struct
        let s_38_4: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_38_5: read-var aligned:u8
        let s_38_5: bool = fn_state.aligned;
        // D s_38_6: read-var accdesc:struct
        let s_38_6: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_38_7: call AArch64_S2Translate(s_38_3, s_38_4, s_38_0, s_38_2, s_38_5, s_38_6)
        let s_38_7: ProductTypedc31059ca7e2391c = AArch64_S2Translate(
            state,
            tracer,
            s_38_3,
            s_38_4,
            s_38_0,
            s_38_2,
            s_38_5,
            s_38_6,
        );
        // D s_38_8: write-var return_value <= s_38_7
        fn_state.return_value = s_38_7;
        // N s_38_9: jump b22
        return block_22(state, tracer, fn_state);
    }
}
