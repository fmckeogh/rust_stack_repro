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
use AArch32_Abort::*;
use CreateAccDescExLDST::*;
use MarkExclusiveGlobal::*;
use ProcessorID::*;
use AArch32_MarkExclusiveVA::*;
use IsAligned__1::*;
use IsFault::*;
use MarkExclusiveLocal::*;
use AArch32_TranslateAddress::*;
use AlignmentFault::*;
use common::*;
pub fn AArch32_SetExclusiveMonitors<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        ga_24308: ProductTypef170cab34335b70c,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        aligned: bool,
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
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // C s_0_1: const #101160u : u32
        let s_0_1: u32 = 101160;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<bool>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // C s_0_3: const #0u : u8
        let s_0_3: bool = false;
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // C s_0_5: const #0u : u32
        let s_0_5: u32 = 0;
        // S s_0_6: call CreateAccDescExLDST(s_0_5, s_0_3, s_0_4)
        let s_0_6: ProductType9878976b5bcce9c9 = CreateAccDescExLDST(
            state,
            tracer,
            s_0_5,
            s_0_3,
            s_0_4,
        );
        // D s_0_7: write-var accdesc <= s_0_6
        fn_state.accdesc = s_0_6;
        // D s_0_8: read-var address:u32
        let s_0_8: u32 = fn_state.address;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 32u16);
        // D s_0_10: read-var size:i
        let s_0_10: i128 = fn_state.size;
        // D s_0_11: call IsAligned__1(s_0_9, s_0_10)
        let s_0_11: bool = IsAligned__1(state, tracer, s_0_9, s_0_10);
        // D s_0_12: write-var aligned <= s_0_11
        fn_state.aligned = s_0_11;
        // D s_0_13: read-var aligned:u8
        let s_0_13: bool = fn_state.aligned;
        // D s_0_14: not s_0_13
        let s_0_14: bool = !s_0_13;
        // N s_0_15: branch s_0_14 b9 b1
        if s_0_14 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var address:u32
        let s_2_0: u32 = fn_state.address;
        // D s_2_1: read-var accdesc:struct
        let s_2_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_2_2: read-var aligned:u8
        let s_2_2: bool = fn_state.aligned;
        // D s_2_3: read-var size:i
        let s_2_3: i128 = fn_state.size;
        // D s_2_4: call AArch32_TranslateAddress(s_2_0, s_2_1, s_2_2, s_2_3)
        let s_2_4: ProductTypece7c66ccb2cab13e = AArch32_TranslateAddress(
            state,
            tracer,
            s_2_0,
            s_2_1,
            s_2_2,
            s_2_3,
        );
        // D s_2_5: write-var memaddrdesc <= s_2_4
        fn_state.memaddrdesc = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var memaddrdesc:struct
        let s_3_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_3_1: call IsFault(s_3_0)
        let s_3_1: bool = IsFault(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b8 b4
        if s_3_1 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var memaddrdesc.2:struct
        let s_4_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_4_1: write-var ga#24308 <= s_4_0
        fn_state.ga_24308 = s_4_0;
        // D s_4_2: read-var ga#24308.5:struct
        let s_4_2: u32 = fn_state.ga_24308._5;
        // C s_4_3: const #0u : u32
        let s_4_3: u32 = 0;
        // D s_4_4: cmp-eq s_4_2 s_4_3
        let s_4_4: bool = ((s_4_2) == (s_4_3));
        // N s_4_5: branch s_4_4 b7 b5
        if s_4_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var memaddrdesc.3:struct
        let s_6_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // C s_6_1: const #() : ()
        let s_6_1: () = ();
        // S s_6_2: call ProcessorID(s_6_1)
        let s_6_2: i128 = ProcessorID(state, tracer, s_6_1);
        // D s_6_3: read-var size:i
        let s_6_3: i128 = fn_state.size;
        // D s_6_4: call MarkExclusiveLocal(s_6_0, s_6_2, s_6_3)
        let s_6_4: () = MarkExclusiveLocal(state, tracer, s_6_0, s_6_2, s_6_3);
        // C s_6_5: const #() : ()
        let s_6_5: () = ();
        // S s_6_6: call ProcessorID(s_6_5)
        let s_6_6: i128 = ProcessorID(state, tracer, s_6_5);
        // D s_6_7: read-var address:u32
        let s_6_7: u32 = fn_state.address;
        // D s_6_8: read-var size:i
        let s_6_8: i128 = fn_state.size;
        // D s_6_9: call AArch32_MarkExclusiveVA(s_6_7, s_6_6, s_6_8)
        let s_6_9: () = AArch32_MarkExclusiveVA(state, tracer, s_6_7, s_6_6, s_6_8);
        // N s_6_10: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var memaddrdesc.3:struct
        let s_7_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // C s_7_1: const #() : ()
        let s_7_1: () = ();
        // S s_7_2: call ProcessorID(s_7_1)
        let s_7_2: i128 = ProcessorID(state, tracer, s_7_1);
        // D s_7_3: read-var size:i
        let s_7_3: i128 = fn_state.size;
        // D s_7_4: call MarkExclusiveGlobal(s_7_0, s_7_2, s_7_3)
        let s_7_4: () = MarkExclusiveGlobal(state, tracer, s_7_0, s_7_2, s_7_3);
        // N s_7_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var accdesc:struct
        let s_9_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_9_1: call AlignmentFault(s_9_0)
        let s_9_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_9_0);
        // D s_9_2: read-var address:u32
        let s_9_2: u32 = fn_state.address;
        // D s_9_3: call AArch32_Abort(s_9_2, s_9_1)
        let s_9_3: () = AArch32_Abort(state, tracer, s_9_2, s_9_1);
        // N s_9_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
