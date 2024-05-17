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
use GCSDataCheckException::*;
use GetCurrentGCSPointer::*;
use Mem_read::*;
use GCSReturnValueCheckEnabled::*;
use CreateAccDescGCS::*;
use common::*;
pub fn LoadCheckGCSRecord<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
    gcsinst_type: u32,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        recorded_va: u64,
        gs_23075: bool,
        gs_449058: Bits,
        vaddress: u64,
        gcsinst_type: u32,
    }
    let fn_state = FunctionState {
        vaddress,
        gcsinst_type,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #0u : u32
        let s_0_2: u32 = 0;
        // D s_0_3: call CreateAccDescGCS(s_0_1, s_0_2)
        let s_0_3: ProductType9878976b5bcce9c9 = CreateAccDescGCS(
            state,
            tracer,
            s_0_1,
            s_0_2,
        );
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call GetCurrentGCSPointer(s_0_4)
        let s_0_5: u64 = GetCurrentGCSPointer(state, tracer, s_0_4);
        // C s_0_6: const #8s : i
        let s_0_6: i128 = 8;
        // D s_0_7: call Mem_read(s_0_5, s_0_6, s_0_3)
        let s_0_7: Bits = Mem_read(state, tracer, s_0_5, s_0_6, s_0_3);
        // D s_0_8: write-var gs#449058 <= s_0_7
        fn_state.gs_449058 = s_0_7;
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_1_0: read-var gs#449058:bv
        let s_1_0: Bits = fn_state.gs_449058;
        // D s_1_1: cast reint s_1_0 -> u64
        let s_1_1: u64 = (s_1_0.value() as u64);
        // D s_1_2: write-var recorded_va <= s_1_1
        fn_state.recorded_va = s_1_1;
        // C s_1_3: const #16975u : u32
        let s_1_3: u32 = 16975;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: call GCSReturnValueCheckEnabled(s_1_4)
        let s_1_5: bool = GCSReturnValueCheckEnabled(state, tracer, s_1_4);
        // N s_1_6: branch s_1_5 b7 b2
        if s_1_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#23075 <= s_2_0
        fn_state.gs_23075 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_3_0: read-var gs#23075:u8
        let s_3_0: bool = fn_state.gs_23075;
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
    ) -> u64 {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_5_0: read-var recorded_va:u64
        let s_5_0: u64 = fn_state.recorded_va;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_6_0: read-var gcsinst_type:u32
        let s_6_0: u32 = fn_state.gcsinst_type;
        // D s_6_1: call GCSDataCheckException(s_6_0)
        let s_6_1: () = GCSDataCheckException(state, tracer, s_6_0);
        // N s_6_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var recorded_va:u64
        let s_7_0: u64 = fn_state.recorded_va;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 64u16);
        // D s_7_2: read-var vaddress:u64
        let s_7_2: u64 = fn_state.vaddress;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 64u16);
        // D s_7_4: cmp-ne s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) != (s_7_3));
        // D s_7_5: write-var gs#23075 <= s_7_4
        fn_state.gs_23075 = s_7_4;
        // N s_7_6: jump b3
        return block_3(state, tracer, fn_state);
    }
}
