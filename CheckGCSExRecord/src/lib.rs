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
use SetCurrentGCSPointer::*;
use GetCurrentGCSPointer::*;
use Mem_read::*;
use Zeros::*;
use GCSDataCheckException::*;
use CreateAccDescGCS::*;
use common::*;
pub fn CheckGCSExRecord<T: Tracer>(
    state: &mut State,
    tracer: &T,
    elr: u64,
    spsr: u64,
    lr: u64,
    gcsinst_type: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_449070: Bits,
        gs_449061: Bits,
        accdesc: ProductType9878976b5bcce9c9,
        gs_449075: Bits,
        ptr: u64,
        gs_449080: Bits,
        elr: u64,
        spsr: u64,
        lr: u64,
        gcsinst_type: u32,
    }
    let fn_state = FunctionState {
        elr,
        spsr,
        lr,
        gcsinst_type,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // D s_0_4: write-var accdesc <= s_0_3
        fn_state.accdesc = s_0_3;
        // C s_0_5: const #() : ()
        let s_0_5: () = ();
        // S s_0_6: call GetCurrentGCSPointer(s_0_5)
        let s_0_6: u64 = GetCurrentGCSPointer(state, tracer, s_0_5);
        // D s_0_7: write-var ptr <= s_0_6
        fn_state.ptr = s_0_6;
        // C s_0_8: const #8s : i
        let s_0_8: i128 = 8;
        // D s_0_9: read-var ptr:u64
        let s_0_9: u64 = fn_state.ptr;
        // D s_0_10: read-var accdesc:struct
        let s_0_10: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_11: call Mem_read(s_0_9, s_0_8, s_0_10)
        let s_0_11: Bits = Mem_read(state, tracer, s_0_9, s_0_8, s_0_10);
        // D s_0_12: write-var gs#449061 <= s_0_11
        fn_state.gs_449061 = s_0_11;
        // N s_0_13: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var gs#449061:bv
        let s_1_0: Bits = fn_state.gs_449061;
        // D s_1_1: cast reint s_1_0 -> u64
        let s_1_1: u64 = (s_1_0.value() as u64);
        // C s_1_2: const #60s : i
        let s_1_2: i128 = 60;
        // S s_1_3: call Zeros(s_1_2)
        let s_1_3: Bits = Zeros(state, tracer, s_1_2);
        // S s_1_4: cast reint s_1_3 -> u60
        let s_1_4: u64 = (s_1_3.value() as u64);
        // S s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 60u16);
        // C s_1_6: const #9u : u8
        let s_1_6: u8 = 9;
        // C s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 4u16);
        // S s_1_8: cast reint s_1_5 -> u128
        let s_1_8: u128 = (s_1_5.value() as u128);
        // D s_1_9: size-of s_1_5
        let s_1_9: u16 = s_1_5.length();
        // C s_1_10: cast reint s_1_7 -> u128
        let s_1_10: u128 = (s_1_7.value() as u128);
        // D s_1_11: size-of s_1_7
        let s_1_11: u16 = s_1_7.length();
        // D s_1_12: lsl s_1_8 s_1_11
        let s_1_12: u128 = s_1_8 << s_1_11;
        // D s_1_13: or s_1_12 s_1_10
        let s_1_13: u128 = ((s_1_12) | (s_1_10));
        // D s_1_14: add s_1_9 s_1_11
        let s_1_14: u16 = (s_1_9 + s_1_11);
        // D s_1_15: create-bits s_1_13 s_1_14
        let s_1_15: Bits = Bits::new(s_1_13, s_1_14);
        // D s_1_16: cast reint s_1_15 -> u64
        let s_1_16: u64 = (s_1_15.value() as u64);
        // D s_1_17: cast zx s_1_1 -> bv
        let s_1_17: Bits = Bits::new(s_1_1 as u128, 64u16);
        // D s_1_18: cast zx s_1_16 -> bv
        let s_1_18: Bits = Bits::new(s_1_16 as u128, 64u16);
        // D s_1_19: cmp-ne s_1_17 s_1_18
        let s_1_19: bool = ((s_1_17) != (s_1_18));
        // N s_1_20: branch s_1_19 b16 b2
        if s_1_19 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var ptr:u64
        let s_3_1: u64 = fn_state.ptr;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 64u16);
        // C s_3_3: cast cvt s_3_0 -> bv
        let s_3_3: Bits = Bits::new(s_3_0 as u128, 128);
        // D s_3_4: add s_3_2 s_3_3
        let s_3_4: Bits = (s_3_2 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> u64
        let s_3_5: u64 = (s_3_4.value() as u64);
        // C s_3_6: const #8s : i
        let s_3_6: i128 = 8;
        // D s_3_7: read-var accdesc:struct
        let s_3_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_3_8: call Mem_read(s_3_5, s_3_6, s_3_7)
        let s_3_8: Bits = Mem_read(state, tracer, s_3_5, s_3_6, s_3_7);
        // D s_3_9: write-var gs#449070 <= s_3_8
        fn_state.gs_449070 = s_3_8;
        // N s_3_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#449070:bv
        let s_4_0: Bits = fn_state.gs_449070;
        // D s_4_1: cast reint s_4_0 -> u64
        let s_4_1: u64 = (s_4_0.value() as u64);
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 64u16);
        // D s_4_3: read-var elr:u64
        let s_4_3: u64 = fn_state.elr;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 64u16);
        // D s_4_5: cmp-ne s_4_2 s_4_4
        let s_4_5: bool = ((s_4_2) != (s_4_4));
        // N s_4_6: branch s_4_5 b15 b5
        if s_4_5 {
            return block_15(state, tracer, fn_state);
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
        // C s_6_0: const #16s : i
        let s_6_0: i128 = 16;
        // D s_6_1: read-var ptr:u64
        let s_6_1: u64 = fn_state.ptr;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 64u16);
        // C s_6_3: cast cvt s_6_0 -> bv
        let s_6_3: Bits = Bits::new(s_6_0 as u128, 128);
        // D s_6_4: add s_6_2 s_6_3
        let s_6_4: Bits = (s_6_2 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> u64
        let s_6_5: u64 = (s_6_4.value() as u64);
        // C s_6_6: const #8s : i
        let s_6_6: i128 = 8;
        // D s_6_7: read-var accdesc:struct
        let s_6_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_6_8: call Mem_read(s_6_5, s_6_6, s_6_7)
        let s_6_8: Bits = Mem_read(state, tracer, s_6_5, s_6_6, s_6_7);
        // D s_6_9: write-var gs#449075 <= s_6_8
        fn_state.gs_449075 = s_6_8;
        // N s_6_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#449075:bv
        let s_7_0: Bits = fn_state.gs_449075;
        // D s_7_1: cast reint s_7_0 -> u64
        let s_7_1: u64 = (s_7_0.value() as u64);
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 64u16);
        // D s_7_3: read-var spsr:u64
        let s_7_3: u64 = fn_state.spsr;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 64u16);
        // D s_7_5: cmp-ne s_7_2 s_7_4
        let s_7_5: bool = ((s_7_2) != (s_7_4));
        // N s_7_6: branch s_7_5 b14 b8
        if s_7_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #24s : i
        let s_9_0: i128 = 24;
        // D s_9_1: read-var ptr:u64
        let s_9_1: u64 = fn_state.ptr;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 64u16);
        // C s_9_3: cast cvt s_9_0 -> bv
        let s_9_3: Bits = Bits::new(s_9_0 as u128, 128);
        // D s_9_4: add s_9_2 s_9_3
        let s_9_4: Bits = (s_9_2 + s_9_3);
        // D s_9_5: cast reint s_9_4 -> u64
        let s_9_5: u64 = (s_9_4.value() as u64);
        // C s_9_6: const #8s : i
        let s_9_6: i128 = 8;
        // D s_9_7: read-var accdesc:struct
        let s_9_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_9_8: call Mem_read(s_9_5, s_9_6, s_9_7)
        let s_9_8: Bits = Mem_read(state, tracer, s_9_5, s_9_6, s_9_7);
        // D s_9_9: write-var gs#449080 <= s_9_8
        fn_state.gs_449080 = s_9_8;
        // N s_9_10: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#449080:bv
        let s_10_0: Bits = fn_state.gs_449080;
        // D s_10_1: cast reint s_10_0 -> u64
        let s_10_1: u64 = (s_10_0.value() as u64);
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 64u16);
        // D s_10_3: read-var lr:u64
        let s_10_3: u64 = fn_state.lr;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 64u16);
        // D s_10_5: cmp-ne s_10_2 s_10_4
        let s_10_5: bool = ((s_10_2) != (s_10_4));
        // N s_10_6: branch s_10_5 b13 b11
        if s_10_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #32s : i
        let s_12_0: i128 = 32;
        // D s_12_1: read-var ptr:u64
        let s_12_1: u64 = fn_state.ptr;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 64u16);
        // C s_12_3: cast cvt s_12_0 -> bv
        let s_12_3: Bits = Bits::new(s_12_0 as u128, 128);
        // D s_12_4: add s_12_2 s_12_3
        let s_12_4: Bits = (s_12_2 + s_12_3);
        // D s_12_5: cast reint s_12_4 -> u64
        let s_12_5: u64 = (s_12_4.value() as u64);
        // D s_12_6: call SetCurrentGCSPointer(s_12_5)
        let s_12_6: () = SetCurrentGCSPointer(state, tracer, s_12_5);
        // N s_12_7: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gcsinst_type:u32
        let s_13_0: u32 = fn_state.gcsinst_type;
        // D s_13_1: call GCSDataCheckException(s_13_0)
        let s_13_1: () = GCSDataCheckException(state, tracer, s_13_0);
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gcsinst_type:u32
        let s_14_0: u32 = fn_state.gcsinst_type;
        // D s_14_1: call GCSDataCheckException(s_14_0)
        let s_14_1: () = GCSDataCheckException(state, tracer, s_14_0);
        // N s_14_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gcsinst_type:u32
        let s_15_0: u32 = fn_state.gcsinst_type;
        // D s_15_1: call GCSDataCheckException(s_15_0)
        let s_15_1: () = GCSDataCheckException(state, tracer, s_15_0);
        // N s_15_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gcsinst_type:u32
        let s_16_0: u32 = fn_state.gcsinst_type;
        // D s_16_1: call GCSDataCheckException(s_16_0)
        let s_16_1: () = GCSDataCheckException(state, tracer, s_16_0);
        // N s_16_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
