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
use AArch64_S1Translate::*;
use CreateFaultyAddressDescriptor::*;
use TranslationRegime::*;
use NoFault__1::*;
use EL2Enabled::*;
use AArch64_S2Translate::*;
use implies::*;
use common::*;
pub fn AArch64_FullTranslate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va: u64,
    accdesc: ProductType9878976b5bcce9c9,
    aligned: bool,
) -> ProductTypece7c66ccb2cab13e {
    #[derive(Default)]
    struct FunctionState {
        gs_19994: bool,
        pa: ProductTypece7c66ccb2cab13e,
        ga_15405: ProductTypedc31059ca7e2391c,
        regime: u32,
        ga_15394: ProductTypedc31059ca7e2391c,
        fault: ProductType1d757adad216cdef,
        return_value: ProductTypece7c66ccb2cab13e,
        ipa: ProductTypece7c66ccb2cab13e,
        va: u64,
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
        // D s_0_0: read-var accdesc.8:struct
        let s_0_0: u8 = fn_state.accdesc._8;
        // D s_0_1: call TranslationRegime(s_0_0)
        let s_0_1: u32 = TranslationRegime(state, tracer, s_0_0);
        // D s_0_2: write-var regime <= s_0_1
        fn_state.regime = s_0_1;
        // D s_0_3: read-var accdesc:struct
        let s_0_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_4: call NoFault__1(s_0_3)
        let s_0_4: ProductType1d757adad216cdef = NoFault__1(state, tracer, s_0_3);
        // D s_0_5: write-var fault <= s_0_4
        fn_state.fault = s_0_4;
        // D s_0_6: read-var fault:struct
        let s_0_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_0_7: read-var regime:u32
        let s_0_7: u32 = fn_state.regime;
        // D s_0_8: read-var va:u64
        let s_0_8: u64 = fn_state.va;
        // D s_0_9: read-var aligned:u8
        let s_0_9: bool = fn_state.aligned;
        // D s_0_10: read-var accdesc:struct
        let s_0_10: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_11: call AArch64_S1Translate(s_0_6, s_0_7, s_0_8, s_0_9, s_0_10)
        let s_0_11: ProductTypedc31059ca7e2391c = AArch64_S1Translate(
            state,
            tracer,
            s_0_6,
            s_0_7,
            s_0_8,
            s_0_9,
            s_0_10,
        );
        // D s_0_12: write-var ga#15394 <= s_0_11
        fn_state.ga_15394 = s_0_11;
        // D s_0_13: read-var ga#15394.0:struct
        let s_0_13: ProductType1d757adad216cdef = fn_state.ga_15394._0;
        // D s_0_14: read-var ga#15394.1:struct
        let s_0_14: ProductTypece7c66ccb2cab13e = fn_state.ga_15394._1;
        // D s_0_15: write-var fault <= s_0_13
        fn_state.fault = s_0_13;
        // D s_0_16: write-var ipa <= s_0_14
        fn_state.ipa = s_0_14;
        // D s_0_17: read-var fault.16:struct
        let s_0_17: u32 = fn_state.fault._16;
        // C s_0_18: const #0u : u32
        let s_0_18: u32 = 0;
        // D s_0_19: cmp-eq s_0_17 s_0_18
        let s_0_19: bool = ((s_0_17) == (s_0_18));
        // N s_0_20: branch s_0_19 b10 b1
        if s_0_19 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_1_0: read-var accdesc.25:struct
        let s_1_0: u32 = fn_state.accdesc._25;
        // C s_1_1: const #2u : u32
        let s_1_1: u32 = 2;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // C s_1_3: const #() : ()
        let s_1_3: () = ();
        // S s_1_4: call EL2Enabled(s_1_3)
        let s_1_4: bool = EL2Enabled(state, tracer, s_1_3);
        // D s_1_5: call implies(s_1_2, s_1_4)
        let s_1_5: bool = implies(state, tracer, s_1_2, s_1_4);
        // N s_1_6: assert s_1_5
        let s_1_6: () = assert!(s_1_5);
        // D s_1_7: read-var regime:u32
        let s_1_7: u32 = fn_state.regime;
        // C s_1_8: const #4u : u32
        let s_1_8: u32 = 4;
        // D s_1_9: cmp-eq s_1_7 s_1_8
        let s_1_9: bool = ((s_1_7) == (s_1_8));
        // N s_1_10: branch s_1_9 b9 b2
        if s_1_9 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#19994 <= s_2_0
        fn_state.gs_19994 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_3_0: read-var gs#19994:u8
        let s_3_0: bool = fn_state.gs_19994;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_4_0: read-var ipa:struct
        let s_4_0: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_5_0: read-var return_value:struct
        let s_5_0: ProductTypece7c66ccb2cab13e = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // C s_6_1: const #() : ()
        let s_6_1: () = ();
        // D s_6_2: create-sum enum = 0:"s_6_1"
        let s_6_2: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_0(s_6_1);
        // D s_6_3: read-var fault:struct
        let s_6_3: ProductType1d757adad216cdef = fn_state.fault;
        // D s_6_4: read-var ipa:struct
        let s_6_4: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_6_5: read-var aligned:u8
        let s_6_5: bool = fn_state.aligned;
        // D s_6_6: read-var accdesc:struct
        let s_6_6: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_6_7: call AArch64_S2Translate(s_6_3, s_6_4, s_6_0, s_6_2, s_6_5, s_6_6)
        let s_6_7: ProductTypedc31059ca7e2391c = AArch64_S2Translate(
            state,
            tracer,
            s_6_3,
            s_6_4,
            s_6_0,
            s_6_2,
            s_6_5,
            s_6_6,
        );
        // D s_6_8: write-var ga#15405 <= s_6_7
        fn_state.ga_15405 = s_6_7;
        // D s_6_9: read-var ga#15405.0:struct
        let s_6_9: ProductType1d757adad216cdef = fn_state.ga_15405._0;
        // D s_6_10: read-var ga#15405.1:struct
        let s_6_10: ProductTypece7c66ccb2cab13e = fn_state.ga_15405._1;
        // D s_6_11: write-var fault <= s_6_9
        fn_state.fault = s_6_9;
        // D s_6_12: write-var pa <= s_6_10
        fn_state.pa = s_6_10;
        // D s_6_13: read-var fault.16:struct
        let s_6_13: u32 = fn_state.fault._16;
        // C s_6_14: const #0u : u32
        let s_6_14: u32 = 0;
        // D s_6_15: cmp-eq s_6_13 s_6_14
        let s_6_15: bool = ((s_6_13) == (s_6_14));
        // N s_6_16: branch s_6_15 b8 b7
        if s_6_15 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_7_0: read-var pa:struct
        let s_7_0: ProductTypece7c66ccb2cab13e = fn_state.pa;
        // D s_7_1: write-var return_value <= s_7_0
        fn_state.return_value = s_7_0;
        // N s_7_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_8_0: read-var va:u64
        let s_8_0: u64 = fn_state.va;
        // D s_8_1: read-var fault:struct
        let s_8_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_8_2: call CreateFaultyAddressDescriptor(s_8_0, s_8_1)
        let s_8_2: ProductTypece7c66ccb2cab13e = CreateFaultyAddressDescriptor(
            state,
            tracer,
            s_8_0,
            s_8_1,
        );
        // D s_8_3: write-var return_value <= s_8_2
        fn_state.return_value = s_8_2;
        // N s_8_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call EL2Enabled(s_9_0)
        let s_9_1: bool = EL2Enabled(state, tracer, s_9_0);
        // D s_9_2: write-var gs#19994 <= s_9_1
        fn_state.gs_19994 = s_9_1;
        // N s_9_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_10_0: read-var va:u64
        let s_10_0: u64 = fn_state.va;
        // D s_10_1: read-var fault:struct
        let s_10_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_10_2: call CreateFaultyAddressDescriptor(s_10_0, s_10_1)
        let s_10_2: ProductTypece7c66ccb2cab13e = CreateFaultyAddressDescriptor(
            state,
            tracer,
            s_10_0,
            s_10_1,
        );
        // D s_10_3: write-var return_value <= s_10_2
        fn_state.return_value = s_10_2;
        // N s_10_4: jump b5
        return block_5(state, tracer, fn_state);
    }
}
