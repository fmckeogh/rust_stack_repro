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
use AArch32_S1TranslateLD::*;
use AArch32_S2Translate::*;
use CreateFaultyAddressDescriptor::*;
use NoFault__1::*;
use TranslationRegime::*;
use TTBCR_read::*;
use AArch32_S1TranslateSD::*;
use u_get_TTBCR_Type_EAE::*;
use EL2Enabled::*;
use common::*;
pub fn AArch32_FullTranslate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va: u32,
    accdesc: ProductType9878976b5bcce9c9,
    aligned: bool,
) -> ProductTypece7c66ccb2cab13e {
    #[derive(Default)]
    struct FunctionState {
        gs_30248: bool,
        pa: ProductTypece7c66ccb2cab13e,
        regime: u32,
        ga_23434: ProductType234df14d4fab6c9d,
        fault: ProductType1d757adad216cdef,
        return_value: ProductTypece7c66ccb2cab13e,
        ga_23442: ProductTypedc31059ca7e2391c,
        gs_30238: bool,
        ipa: ProductTypece7c66ccb2cab13e,
        ga_23433: ProductTypedc31059ca7e2391c,
        va: u32,
        accdesc: ProductType9878976b5bcce9c9,
        aligned: bool,
    }
    let fn_state = FunctionState {
        va,
        accdesc,
        aligned,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_0_0: read-var accdesc:struct
        let s_0_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_1: call NoFault__1(s_0_0)
        let s_0_1: ProductType1d757adad216cdef = NoFault__1(state, tracer, s_0_0);
        // D s_0_2: write-var fault <= s_0_1
        fn_state.fault = s_0_1;
        // D s_0_3: read-var accdesc.8:struct
        let s_0_3: u8 = fn_state.accdesc._8;
        // D s_0_4: call TranslationRegime(s_0_3)
        let s_0_4: u32 = TranslationRegime(state, tracer, s_0_3);
        // D s_0_5: write-var regime <= s_0_4
        fn_state.regime = s_0_4;
        // D s_0_6: read-var regime:u32
        let s_0_6: u32 = fn_state.regime;
        // C s_0_7: const #2u : u32
        let s_0_7: u32 = 2;
        // D s_0_8: cmp-eq s_0_6 s_0_7
        let s_0_8: bool = ((s_0_6) == (s_0_7));
        // N s_0_9: branch s_0_8 b16 b1
        if s_0_8 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call TTBCR_read(s_1_0)
        let s_1_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_1_0);
        // S s_1_2: call _get_TTBCR_Type_EAE(s_1_1)
        let s_1_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_1_1);
        // S s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // C s_1_4: const #1u : u8
        let s_1_4: bool = true;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // S s_1_6: cmp-eq s_1_3 s_1_5
        let s_1_6: bool = ((s_1_3) == (s_1_5));
        // D s_1_7: write-var gs#30238 <= s_1_6
        fn_state.gs_30238 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_2_0: read-var gs#30238:u8
        let s_2_0: bool = fn_state.gs_30238;
        // N s_2_1: branch s_2_0 b15 b3
        if s_2_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_3_0: read-var fault:struct
        let s_3_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_3_1: read-var regime:u32
        let s_3_1: u32 = fn_state.regime;
        // D s_3_2: read-var va:u32
        let s_3_2: u32 = fn_state.va;
        // D s_3_3: read-var aligned:u8
        let s_3_3: bool = fn_state.aligned;
        // D s_3_4: read-var accdesc:struct
        let s_3_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_3_5: call AArch32_S1TranslateSD(s_3_0, s_3_1, s_3_2, s_3_3, s_3_4)
        let s_3_5: ProductType234df14d4fab6c9d = AArch32_S1TranslateSD(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_2,
            s_3_3,
            s_3_4,
        );
        // D s_3_6: write-var ga#23434 <= s_3_5
        fn_state.ga_23434 = s_3_5;
        // D s_3_7: read-var ga#23434.0:struct
        let s_3_7: ProductType1d757adad216cdef = fn_state.ga_23434._0;
        // D s_3_8: read-var ga#23434.1:struct
        let s_3_8: ProductTypece7c66ccb2cab13e = fn_state.ga_23434._1;
        // D s_3_9: write-var fault <= s_3_7
        fn_state.fault = s_3_7;
        // D s_3_10: write-var ipa <= s_3_8
        fn_state.ipa = s_3_8;
        // N s_3_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_4_0: read-var fault.16:struct
        let s_4_0: u32 = fn_state.fault._16;
        // C s_4_1: const #0u : u32
        let s_4_1: u32 = 0;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // N s_4_3: branch s_4_2 b14 b5
        if s_4_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_5_0: read-var regime:u32
        let s_5_0: u32 = fn_state.regime;
        // C s_5_1: const #4u : u32
        let s_5_1: u32 = 4;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // N s_5_3: branch s_5_2 b13 b6
        if s_5_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#30248 <= s_6_0
        fn_state.gs_30248 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_7_0: read-var gs#30248:u8
        let s_7_0: bool = fn_state.gs_30248;
        // N s_7_1: branch s_7_0 b10 b8
        if s_7_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_8_0: read-var ipa:struct
        let s_8_0: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_9_0: read-var return_value:struct
        let s_9_0: ProductTypece7c66ccb2cab13e = fn_state.return_value;
        // N s_9_1: return s_9_0
        return s_9_0;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_10_0: const #64s : i
        let s_10_0: i128 = 64;
        // D s_10_1: read-var va:u32
        let s_10_1: u32 = fn_state.va;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 32u16);
        // D s_10_3: bits-cast zx s_10_2 -> bv length s_10_0
        let s_10_3: Bits = s_10_2.zero_extend(s_10_0);
        // D s_10_4: cast reint s_10_3 -> u64
        let s_10_4: u64 = (s_10_3.value() as u64);
        // D s_10_5: write-var ipa.7 <= s_10_4
        fn_state.ipa._7 = s_10_4;
        // C s_10_6: const #() : ()
        let s_10_6: () = ();
        // D s_10_7: create-sum enum = 0:"s_10_6"
        let s_10_7: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_0(s_10_6);
        // D s_10_8: read-var fault:struct
        let s_10_8: ProductType1d757adad216cdef = fn_state.fault;
        // D s_10_9: read-var ipa:struct
        let s_10_9: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_10_10: read-var aligned:u8
        let s_10_10: bool = fn_state.aligned;
        // D s_10_11: read-var accdesc:struct
        let s_10_11: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_10_12: call AArch32_S2Translate(s_10_8, s_10_9, s_10_7, s_10_10, s_10_11)
        let s_10_12: ProductTypedc31059ca7e2391c = AArch32_S2Translate(
            state,
            tracer,
            s_10_8,
            s_10_9,
            s_10_7,
            s_10_10,
            s_10_11,
        );
        // D s_10_13: write-var ga#23442 <= s_10_12
        fn_state.ga_23442 = s_10_12;
        // D s_10_14: read-var ga#23442.0:struct
        let s_10_14: ProductType1d757adad216cdef = fn_state.ga_23442._0;
        // D s_10_15: read-var ga#23442.1:struct
        let s_10_15: ProductTypece7c66ccb2cab13e = fn_state.ga_23442._1;
        // D s_10_16: write-var fault <= s_10_14
        fn_state.fault = s_10_14;
        // D s_10_17: write-var pa <= s_10_15
        fn_state.pa = s_10_15;
        // D s_10_18: read-var fault.16:struct
        let s_10_18: u32 = fn_state.fault._16;
        // C s_10_19: const #0u : u32
        let s_10_19: u32 = 0;
        // D s_10_20: cmp-eq s_10_18 s_10_19
        let s_10_20: bool = ((s_10_18) == (s_10_19));
        // N s_10_21: branch s_10_20 b12 b11
        if s_10_20 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_11_0: read-var pa:struct
        let s_11_0: ProductTypece7c66ccb2cab13e = fn_state.pa;
        // D s_11_1: write-var return_value <= s_11_0
        fn_state.return_value = s_11_0;
        // N s_11_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_12_0: const #64s : i
        let s_12_0: i128 = 64;
        // D s_12_1: read-var va:u32
        let s_12_1: u32 = fn_state.va;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 32u16);
        // D s_12_3: bits-cast zx s_12_2 -> bv length s_12_0
        let s_12_3: Bits = s_12_2.zero_extend(s_12_0);
        // D s_12_4: cast reint s_12_3 -> u64
        let s_12_4: u64 = (s_12_3.value() as u64);
        // D s_12_5: read-var fault:struct
        let s_12_5: ProductType1d757adad216cdef = fn_state.fault;
        // D s_12_6: call CreateFaultyAddressDescriptor(s_12_4, s_12_5)
        let s_12_6: ProductTypece7c66ccb2cab13e = CreateFaultyAddressDescriptor(
            state,
            tracer,
            s_12_4,
            s_12_5,
        );
        // D s_12_7: write-var return_value <= s_12_6
        fn_state.return_value = s_12_6;
        // N s_12_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call EL2Enabled(s_13_0)
        let s_13_1: bool = EL2Enabled(state, tracer, s_13_0);
        // D s_13_2: write-var gs#30248 <= s_13_1
        fn_state.gs_30248 = s_13_1;
        // N s_13_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_14_0: const #64s : i
        let s_14_0: i128 = 64;
        // D s_14_1: read-var va:u32
        let s_14_1: u32 = fn_state.va;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 32u16);
        // D s_14_3: bits-cast zx s_14_2 -> bv length s_14_0
        let s_14_3: Bits = s_14_2.zero_extend(s_14_0);
        // D s_14_4: cast reint s_14_3 -> u64
        let s_14_4: u64 = (s_14_3.value() as u64);
        // D s_14_5: read-var fault:struct
        let s_14_5: ProductType1d757adad216cdef = fn_state.fault;
        // D s_14_6: call CreateFaultyAddressDescriptor(s_14_4, s_14_5)
        let s_14_6: ProductTypece7c66ccb2cab13e = CreateFaultyAddressDescriptor(
            state,
            tracer,
            s_14_4,
            s_14_5,
        );
        // D s_14_7: write-var return_value <= s_14_6
        fn_state.return_value = s_14_6;
        // N s_14_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_15_0: read-var fault:struct
        let s_15_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_15_1: read-var regime:u32
        let s_15_1: u32 = fn_state.regime;
        // D s_15_2: read-var va:u32
        let s_15_2: u32 = fn_state.va;
        // D s_15_3: read-var aligned:u8
        let s_15_3: bool = fn_state.aligned;
        // D s_15_4: read-var accdesc:struct
        let s_15_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_15_5: call AArch32_S1TranslateLD(s_15_0, s_15_1, s_15_2, s_15_3, s_15_4)
        let s_15_5: ProductTypedc31059ca7e2391c = AArch32_S1TranslateLD(
            state,
            tracer,
            s_15_0,
            s_15_1,
            s_15_2,
            s_15_3,
            s_15_4,
        );
        // D s_15_6: write-var ga#23433 <= s_15_5
        fn_state.ga_23433 = s_15_5;
        // D s_15_7: read-var ga#23433.0:struct
        let s_15_7: ProductType1d757adad216cdef = fn_state.ga_23433._0;
        // D s_15_8: read-var ga#23433.1:struct
        let s_15_8: ProductTypece7c66ccb2cab13e = fn_state.ga_23433._1;
        // D s_15_9: write-var fault <= s_15_7
        fn_state.fault = s_15_7;
        // D s_15_10: write-var ipa <= s_15_8
        fn_state.ipa = s_15_8;
        // N s_15_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#30238 <= s_16_0
        fn_state.gs_30238 = s_16_0;
        // N s_16_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
