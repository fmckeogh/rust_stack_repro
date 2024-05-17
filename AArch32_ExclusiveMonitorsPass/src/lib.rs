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
use ClearExclusiveLocal::*;
use CreateAccDescExLDST::*;
use AArch32_Abort::*;
use AArch32_IsExclusiveVA::*;
use IsExclusiveLocal::*;
use IsAligned__1::*;
use IsFault::*;
use ProcessorID::*;
use AArch32_TranslateAddress::*;
use IsExclusiveGlobal::*;
use AlignmentFault::*;
use common::*;
pub fn AArch32_ExclusiveMonitorsPass<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i128,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        gs_31220: bool,
        passed: bool,
        ga_24324: ProductTypef170cab34335b70c,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        aligned: bool,
        return_value: bool,
        address: u32,
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
        // D s_0_5: read-var address:u32
        let s_0_5: u32 = fn_state.address;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 32u16);
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
        // N s_0_12: branch s_0_11 b16 b1
        if s_0_11 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call ProcessorID(s_2_0)
        let s_2_1: i128 = ProcessorID(state, tracer, s_2_0);
        // D s_2_2: read-var address:u32
        let s_2_2: u32 = fn_state.address;
        // D s_2_3: read-var size:i
        let s_2_3: i128 = fn_state.size;
        // D s_2_4: call AArch32_IsExclusiveVA(s_2_2, s_2_1, s_2_3)
        let s_2_4: bool = AArch32_IsExclusiveVA(state, tracer, s_2_2, s_2_1, s_2_3);
        // D s_2_5: not s_2_4
        let s_2_5: bool = !s_2_4;
        // N s_2_6: branch s_2_5 b15 b3
        if s_2_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var address:u32
        let s_3_0: u32 = fn_state.address;
        // D s_3_1: read-var accdesc:struct
        let s_3_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_3_2: read-var aligned:u8
        let s_3_2: bool = fn_state.aligned;
        // D s_3_3: read-var size:i
        let s_3_3: i128 = fn_state.size;
        // D s_3_4: call AArch32_TranslateAddress(s_3_0, s_3_1, s_3_2, s_3_3)
        let s_3_4: ProductTypece7c66ccb2cab13e = AArch32_TranslateAddress(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_2,
            s_3_3,
        );
        // D s_3_5: write-var memaddrdesc <= s_3_4
        fn_state.memaddrdesc = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var memaddrdesc:struct
        let s_4_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_4_1: call IsFault(s_4_0)
        let s_4_1: bool = IsFault(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b14 b5
        if s_4_1 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var memaddrdesc.3:struct
        let s_6_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // C s_6_1: const #() : ()
        let s_6_1: () = ();
        // S s_6_2: call ProcessorID(s_6_1)
        let s_6_2: i128 = ProcessorID(state, tracer, s_6_1);
        // D s_6_3: read-var size:i
        let s_6_3: i128 = fn_state.size;
        // D s_6_4: call IsExclusiveLocal(s_6_0, s_6_2, s_6_3)
        let s_6_4: bool = IsExclusiveLocal(state, tracer, s_6_0, s_6_2, s_6_3);
        // D s_6_5: write-var passed <= s_6_4
        fn_state.passed = s_6_4;
        // C s_6_6: const #() : ()
        let s_6_6: () = ();
        // S s_6_7: call ProcessorID(s_6_6)
        let s_6_7: i128 = ProcessorID(state, tracer, s_6_6);
        // S s_6_8: call ClearExclusiveLocal(s_6_7)
        let s_6_8: () = ClearExclusiveLocal(state, tracer, s_6_7);
        // D s_6_9: read-var passed:u8
        let s_6_9: bool = fn_state.passed;
        // N s_6_10: branch s_6_9 b13 b7
        if s_6_9 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#31220 <= s_7_0
        fn_state.gs_31220 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#31220:u8
        let s_8_0: bool = fn_state.gs_31220;
        // N s_8_1: branch s_8_0 b12 b9
        if s_8_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var passed:u8
        let s_10_0: bool = fn_state.passed;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var return_value:u8
        let s_11_0: bool = fn_state.return_value;
        // N s_11_1: return s_11_0
        return s_11_0;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var memaddrdesc.3:struct
        let s_12_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // C s_12_1: const #() : ()
        let s_12_1: () = ();
        // S s_12_2: call ProcessorID(s_12_1)
        let s_12_2: i128 = ProcessorID(state, tracer, s_12_1);
        // D s_12_3: read-var size:i
        let s_12_3: i128 = fn_state.size;
        // D s_12_4: call IsExclusiveGlobal(s_12_0, s_12_2, s_12_3)
        let s_12_4: bool = IsExclusiveGlobal(state, tracer, s_12_0, s_12_2, s_12_3);
        // D s_12_5: write-var passed <= s_12_4
        fn_state.passed = s_12_4;
        // N s_12_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var memaddrdesc.2:struct
        let s_13_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_13_1: write-var ga#24324 <= s_13_0
        fn_state.ga_24324 = s_13_0;
        // D s_13_2: read-var ga#24324.5:struct
        let s_13_2: u32 = fn_state.ga_24324._5;
        // C s_13_3: const #0u : u32
        let s_13_3: u32 = 0;
        // D s_13_4: cmp-eq s_13_2 s_13_3
        let s_13_4: bool = ((s_13_2) == (s_13_3));
        // D s_13_5: write-var gs#31220 <= s_13_4
        fn_state.gs_31220 = s_13_4;
        // N s_13_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var memaddrdesc.0:struct
        let s_14_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_14_1: read-var address:u32
        let s_14_1: u32 = fn_state.address;
        // D s_14_2: call AArch32_Abort(s_14_1, s_14_0)
        let s_14_2: () = AArch32_Abort(state, tracer, s_14_1, s_14_0);
        // N s_14_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var return_value <= s_15_0
        fn_state.return_value = s_15_0;
        // N s_15_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var accdesc:struct
        let s_16_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_16_1: call AlignmentFault(s_16_0)
        let s_16_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_16_0);
        // D s_16_2: read-var address:u32
        let s_16_2: u32 = fn_state.address;
        // D s_16_3: call AArch32_Abort(s_16_2, s_16_1)
        let s_16_3: () = AArch32_Abort(state, tracer, s_16_2, s_16_1);
        // N s_16_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
