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
use AlignmentFault::*;
use AArch64_UnalignedAccessFaults::*;
use ClearExclusiveLocal::*;
use CreateAccDescExLDST::*;
use ProcessorID::*;
use IsExclusiveLocal::*;
use IsAligned__1::*;
use IsFault::*;
use IsExclusiveGlobal::*;
use AArch64_Abort::*;
use AArch64_TranslateAddress::*;
use AArch64_IsExclusiveVA::*;
use common::*;
pub fn AArch64_ExclusiveMonitorsPass<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    size: i128,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        ga_16434: ProductTypef170cab34335b70c,
        gs_21014: bool,
        passed: bool,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        aligned: bool,
        return_value: bool,
        gs_21010: bool,
        address: u64,
        size: i128,
    }
    let fn_state = FunctionState {
        address,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // C s_0_1: const #0u : u8
        let s_0_1: bool = false;
        // C s_0_2: const #1u : u32
        let s_0_2: u32 = 1;
        // S s_0_3: call CreateAccDescExLDST(s_0_2, s_0_0, s_0_1)
        let s_0_3: ProductType9878976b5bcce9c9 = CreateAccDescExLDST(
            state,
            tracer,
            s_0_2,
            s_0_0,
            s_0_1,
        );
        // D s_0_4: write-var accdesc <= s_0_3
        fn_state.accdesc = s_0_3;
        // D s_0_5: read-var address:u64
        let s_0_5: u64 = fn_state.address;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 64u16);
        // D s_0_7: read-var size:i
        let s_0_7: i128 = fn_state.size;
        // D s_0_8: call IsAligned__1(s_0_6, s_0_7)
        let s_0_8: bool = IsAligned__1(state, tracer, s_0_6, s_0_7);
        // D s_0_9: write-var aligned <= s_0_8
        fn_state.aligned = s_0_8;
        // D s_0_10: read-var aligned:u8
        let s_0_10: bool = fn_state.aligned;
        // D s_0_11: not s_0_10
        let s_0_11: bool = !s_0_10;
        // N s_0_12: branch s_0_11 b19 b1
        if s_0_11 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#21010 <= s_1_0
        fn_state.gs_21010 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#21010:u8
        let s_2_0: bool = fn_state.gs_21010;
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
    ) -> bool {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call ProcessorID(s_4_0)
        let s_4_1: i128 = ProcessorID(state, tracer, s_4_0);
        // D s_4_2: read-var address:u64
        let s_4_2: u64 = fn_state.address;
        // D s_4_3: read-var size:i
        let s_4_3: i128 = fn_state.size;
        // D s_4_4: call AArch64_IsExclusiveVA(s_4_2, s_4_1, s_4_3)
        let s_4_4: bool = AArch64_IsExclusiveVA(state, tracer, s_4_2, s_4_1, s_4_3);
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b17 b5
        if s_4_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var address:u64
        let s_5_0: u64 = fn_state.address;
        // D s_5_1: read-var accdesc:struct
        let s_5_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_5_2: read-var aligned:u8
        let s_5_2: bool = fn_state.aligned;
        // D s_5_3: read-var size:i
        let s_5_3: i128 = fn_state.size;
        // D s_5_4: call AArch64_TranslateAddress(s_5_0, s_5_1, s_5_2, s_5_3)
        let s_5_4: ProductTypece7c66ccb2cab13e = AArch64_TranslateAddress(
            state,
            tracer,
            s_5_0,
            s_5_1,
            s_5_2,
            s_5_3,
        );
        // D s_5_5: write-var memaddrdesc <= s_5_4
        fn_state.memaddrdesc = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var memaddrdesc:struct
        let s_6_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_6_1: call IsFault(s_6_0)
        let s_6_1: bool = IsFault(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b16 b7
        if s_6_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var memaddrdesc.3:struct
        let s_8_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // C s_8_1: const #() : ()
        let s_8_1: () = ();
        // S s_8_2: call ProcessorID(s_8_1)
        let s_8_2: i128 = ProcessorID(state, tracer, s_8_1);
        // D s_8_3: read-var size:i
        let s_8_3: i128 = fn_state.size;
        // D s_8_4: call IsExclusiveLocal(s_8_0, s_8_2, s_8_3)
        let s_8_4: bool = IsExclusiveLocal(state, tracer, s_8_0, s_8_2, s_8_3);
        // D s_8_5: write-var passed <= s_8_4
        fn_state.passed = s_8_4;
        // C s_8_6: const #() : ()
        let s_8_6: () = ();
        // S s_8_7: call ProcessorID(s_8_6)
        let s_8_7: i128 = ProcessorID(state, tracer, s_8_6);
        // S s_8_8: call ClearExclusiveLocal(s_8_7)
        let s_8_8: () = ClearExclusiveLocal(state, tracer, s_8_7);
        // D s_8_9: read-var passed:u8
        let s_8_9: bool = fn_state.passed;
        // N s_8_10: branch s_8_9 b15 b9
        if s_8_9 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#21014 <= s_9_0
        fn_state.gs_21014 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#21014:u8
        let s_10_0: bool = fn_state.gs_21014;
        // N s_10_1: branch s_10_0 b14 b11
        if s_10_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var passed:u8
        let s_12_0: bool = fn_state.passed;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var return_value:u8
        let s_13_0: bool = fn_state.return_value;
        // N s_13_1: return s_13_0
        return s_13_0;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var memaddrdesc.3:struct
        let s_14_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // C s_14_1: const #() : ()
        let s_14_1: () = ();
        // S s_14_2: call ProcessorID(s_14_1)
        let s_14_2: i128 = ProcessorID(state, tracer, s_14_1);
        // D s_14_3: read-var size:i
        let s_14_3: i128 = fn_state.size;
        // D s_14_4: call IsExclusiveGlobal(s_14_0, s_14_2, s_14_3)
        let s_14_4: bool = IsExclusiveGlobal(state, tracer, s_14_0, s_14_2, s_14_3);
        // D s_14_5: write-var passed <= s_14_4
        fn_state.passed = s_14_4;
        // N s_14_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var memaddrdesc.2:struct
        let s_15_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_15_1: write-var ga#16434 <= s_15_0
        fn_state.ga_16434 = s_15_0;
        // D s_15_2: read-var ga#16434.5:struct
        let s_15_2: u32 = fn_state.ga_16434._5;
        // C s_15_3: const #0u : u32
        let s_15_3: u32 = 0;
        // D s_15_4: cmp-eq s_15_2 s_15_3
        let s_15_4: bool = ((s_15_2) == (s_15_3));
        // D s_15_5: write-var gs#21014 <= s_15_4
        fn_state.gs_21014 = s_15_4;
        // N s_15_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var memaddrdesc.0:struct
        let s_16_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_16_1: read-var address:u64
        let s_16_1: u64 = fn_state.address;
        // D s_16_2: call AArch64_Abort(s_16_1, s_16_0)
        let s_16_2: () = AArch64_Abort(state, tracer, s_16_1, s_16_0);
        // N s_16_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var accdesc:struct
        let s_18_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_18_1: call AlignmentFault(s_18_0)
        let s_18_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_18_0);
        // D s_18_2: read-var address:u64
        let s_18_2: u64 = fn_state.address;
        // D s_18_3: call AArch64_Abort(s_18_2, s_18_1)
        let s_18_3: () = AArch64_Abort(state, tracer, s_18_2, s_18_1);
        // N s_18_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var accdesc:struct
        let s_19_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_19_1: read-var address:u64
        let s_19_1: u64 = fn_state.address;
        // D s_19_2: read-var size:i
        let s_19_2: i128 = fn_state.size;
        // D s_19_3: call AArch64_UnalignedAccessFaults(s_19_0, s_19_1, s_19_2)
        let s_19_3: bool = AArch64_UnalignedAccessFaults(
            state,
            tracer,
            s_19_0,
            s_19_1,
            s_19_2,
        );
        // D s_19_4: write-var gs#21010 <= s_19_3
        fn_state.gs_21010 = s_19_3;
        // N s_19_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
